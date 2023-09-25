use crate::common::utils::instantiate_display_data;
use crate::macro_counter::MacroCounter;

use super::display_data::*;
use super::pathing::{self, Pathing};
use std::{fs, io};

pub fn predefined() {
    println!("\nOperation:");

    let user_dir = pathing::user_path();
    let dir_path = format!("{}/predefined_meals", user_dir);

    let directory = match fs::read_dir(dir_path.clone()) {
        Ok(dir) => dir,
        Err(err) => {
            dbg!(err);
            panic!("Error: unable to read: '{}'", dir_path);
        }
    };

    let mut operation = String::new();
    match io::stdin().read_line(&mut operation) {
        Ok(oper) => oper,
        Err(_) => {
            println!("Error: unable to read operation '{}'", operation);
            return predefined();
        }
    };

    if operation.contains("q") {
        return crate::main();
    }

    if operation.contains("cf") {
        let mut dir_len = 1;
        for _ in directory {
            dir_len += 1;
        }

        let file_name = format!("m{}.txt", dir_len);
        let pathing = Pathing {
            dir_path: dir_path.clone(),
            file_path: format!("{}/{}", dir_path, file_name),
        };
        Pathing::create_file(&pathing);
    }

    if operation.contains("mf") {
        let directory = match fs::read_dir(&dir_path) {
            Ok(dir) => dir,
            Err(err) => {
                dbg!(err);
                panic!("Error: unable to read: '{}'", dir_path);
            }
        };

        println!("\nEnter a relative file path to modify from:");
        for file in directory {
            println!("{}", file.unwrap().path().display());
        }

        let mut file_name = String::new();
        match io::stdin().read_line(&mut file_name) {
            Ok(file_name) => file_name,
            Err(_) => {
                println!("Error: unable to read '{}'", file_name);
                return predefined();
            }
        };

        let file_path: String = if file_name.contains(".txt") {
            format!("{}/{}", dir_path, &file_name[0..file_name.len() - 1])
        } else {
            format!("{}/{}.txt", dir_path, &file_name[0..file_name.len() - 1])
        };

        if !pathing::file_exists(&file_path) {
            dbg!(&file_path);
            panic!("Error: Invalid file");
        }

        let mut macro_counter = MacroCounter {
            file_path,
            calorie: Vec::new(),
            fat: Vec::new(),
            carb: Vec::new(),
            protein: Vec::new(),
            totals: Vec::new(),
        };
        MacroCounter::compile_data(&mut macro_counter, true);
        MacroCounter::collect_data(&mut macro_counter);
    }

    if operation.contains("df") {
        let mut display_data = instantiate_display_data(String::new(), dir_path.clone());
        DisplayData::display_previous_file(&mut display_data, dir_path, false, true);
    }

    return predefined();
}

#[cfg(test)]
mod integration_test {
    use super::*;
    use crate::common::utils::instantiate_macro_counter;
    use crate::pathing;

    // removes whatever data is appended to test files during test_predefined()
    // so it doesn't interfere with other tests.
    fn remove_test_data(mut test_data: MacroCounter, iter: usize) {
        MacroCounter::compile_data(&mut test_data, true);

        for _ in 0..iter {
            test_data.calorie.pop();
            test_data.fat.pop();
            test_data.carb.pop();
            test_data.protein.pop();
        }

        MacroCounter::write_file(&mut test_data);
    }

    #[test]
    fn test_predefined() {
        let test_file = format!("{}/test_data/good_data/data_3.txt", pathing::user_path());
        let mut test_data = instantiate_macro_counter(test_file);

        // The 1st call to MacroCounter::compile_data() is to save data
        // already in the file.
        MacroCounter::compile_data(&mut test_data, true);

        let predefined_file = format!("{}/predefined_meals/m1.txt", pathing::user_path());
        test_data.file_path.clear();
        test_data.file_path.push_str(&predefined_file);

        // The 2nd call to MacroCounter::compile_data() is to append data read
        // from the predefined_file and aggregate it to the MacroCounter struct fields.
        MacroCounter::compile_data(&mut test_data, false);

        let test_file = format!("{}/test_data/good_data/data_3.txt", pathing::user_path());
        test_data.file_path.clear();
        test_data.file_path.push_str(&test_file);
        MacroCounter::write_file(&mut test_data);

        let expected_values: Vec<f32> = vec![770.0, 39.0, 87.0, 59.0];
        let resultant_values: Vec<f32> = test_data.totals.clone();

        remove_test_data(test_data, 3);
        assert_eq!(expected_values, resultant_values);
    }
}
