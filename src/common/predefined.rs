use crate::macro_counter::MacroCounter;

use super::display_data::*;
use super::pathing::*;
use std::{fs, io};

pub fn predefined() {
    println!("\nOperation:");

    let dir_path = String::from("/home/vallen/Workspace/rust_macro_counter/predefined_meals");
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

        if !Pathing::file_exists(&file_path) {
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
        let mut display_data = DisplayData {
            dir_path: dir_path.clone(),
            file_path: String::new(),
            macro_totals: vec![],
            totals: Vec::new(),
        };
        DisplayData::display_previous_file(&mut display_data, dir_path, false, true);
    }

    return predefined();
}
