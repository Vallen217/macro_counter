pub mod input_data;
pub mod write_file;

use crate::common::{display_data::DisplayData, pathing::Pathing, utils::Date};
use crate::instantiate_display_data;
use regex::Regex;
use std::{fs, io};

enum MacroType {
    Calorie,
    Fat,
    Carb,
    Protein,
}

#[derive(Debug)]
pub struct MacroCounter {
    pub file_path: String,
    pub calorie: Vec<f32>,
    pub fat: Vec<f32>,
    pub carb: Vec<f32>,
    pub protein: Vec<f32>,
    pub totals: Vec<f32>,
}

impl MacroCounter {
    pub fn compile_data(&mut self, clean_data: bool) {
        if clean_data {
            self.calorie.clear();
            self.fat.clear();
            self.carb.clear();
            self.protein.clear();
            self.totals.clear();
        }

        let file_data = match fs::read_to_string(self.file_path.clone()) {
            Ok(file_data) => file_data,
            Err(err) => {
                dbg!(err);
                panic!("Error: reading '{}'", &self.file_path);
            }
        };

        for line in file_data.lines() {
            if line.is_empty() {
                break;
            }

            // ignores file lines that aren't numbers
            let re = Regex::new(r"\d+\.?\d?g?").unwrap();
            if re.is_match(&line) {
                for (iter, mut datum) in line.split_whitespace().enumerate() {
                    // remove the 'gram' annotaion from file lines.
                    if datum.contains('g') {
                        datum = &datum[0..datum.len() - 1];
                    }

                    // converting file data into compilable numbers.
                    let datum: f32 = match datum.parse() {
                        Ok(data) => data,
                        Err(error) => {
                            dbg!(error);
                            panic!("Error: compiling datum: '{}'", datum);
                        }
                    };

                    match iter {
                        0 => self.calorie.push(datum),
                        1 => self.fat.push(datum),
                        2 => self.carb.push(datum),
                        3 => self.protein.push(datum),
                        _ => {
                            dbg!(iter);
                            panic!("Error: iterator out of bounds while compiling file data");
                        }
                    };
                }
            }
        }
    }

    pub fn get_operation(&mut self, predefined: bool) {
        self.compile_data(true);

        let mut operation = String::new();
        match io::stdin().read_line(&mut operation) {
            Ok(oper) => oper,
            Err(_) => {
                println!("Error: unable to read operation '{}'", operation);
                return self.get_operation(false);
            }
        };

        if operation.trim() == "." {
            self.repeat_data_entry(predefined, MacroType::Calorie);
            self.repeat_data_entry(predefined, MacroType::Fat);
            self.repeat_data_entry(predefined, MacroType::Carb);
            self.repeat_data_entry(predefined, MacroType::Protein);
            self.write_file();

            println!("\n(Press enter to continue)\nOperation:");
            return self.get_operation(predefined);
        }

        if operation.trim() == "q" {
            let mut display_data = DisplayData {
                file_path: self.file_path.clone(),
                dir_path: String::new(),
                macro_totals: vec![],
                totals: Vec::new(),
            };
            DisplayData::display_file(&mut display_data, None);

            match predefined {
                true => return super::common::predefined::predefined(),
                false => return crate::main(),
            };
        }

        let re = Regex::new(r"rlq?[0-9]*").unwrap();
        if re.is_match(&operation) {
            return self.remove_data(operation, predefined);
        }

        if operation.contains("q") {
            let mut display_data = DisplayData {
                file_path: self.file_path.clone(),
                dir_path: String::new(),
                macro_totals: vec![],
                totals: Vec::new(),
            };
            DisplayData::display_file(&mut display_data, None);

            match predefined {
                true => super::common::predefined::predefined(),
                false => crate::main(),
            }
        } else {
            self.collect_data(predefined);
        }
    }

    fn remove_data(&mut self, operation: String, predefined: bool) {
        loop {
            let iter: i8 = if operation.contains("q") {
                // remove 1 file line if the number of lines to remove is not specified.
                if &operation.trim().len() < &4 {
                    1
                } else {
                    match operation.clone().trim()[3..].parse() {
                        Ok(data) => data,
                        Err(error) => {
                            dbg!(error);
                            panic!("Error: parsing operation '{}'", operation);
                        }
                    }
                }
            } else {
                // remove 1 file line if the number of lines to remove is not specified.
                if &operation.trim().len() < &3 {
                    1
                } else {
                    match operation.clone().trim()[2..].parse() {
                        Ok(data) => data,
                        Err(error) => {
                            dbg!(error);
                            panic!("Error: parsing operation '{}'", operation);
                        }
                    }
                }
            };

            for _ in 0..iter {
                self.calorie.pop();
                self.fat.pop();
                self.carb.pop();
                self.protein.pop();
            }
            // write the new MacroCounter fields to the file.
            self.write_file();

            if operation.trim().contains("q") {
                let pathing = Pathing::generate_file_path(&Date::current_date(), true);
                let display_file_path =
                    instantiate_display_data(pathing.day_path.clone(), pathing.month_path.clone());
                DisplayData::display_file(&display_file_path, None);
                break;
            }
            return self.collect_data(predefined);
        }
    }

    fn repeat_data_entry(&mut self, predefined: bool, macro_type: MacroType) {
        let macro_datum = match macro_type {
            MacroType::Calorie => &self.calorie,
            MacroType::Carb => &self.carb,
            MacroType::Fat => &self.fat,
            MacroType::Protein => &self.protein,
        };

        let datum = match macro_datum.last() {
            Some(datum) => datum,
            None => {
                println!(
                    "Error: Empty vector field\n
                    Input data before attempting to repeat a data entry"
                );
                return self.get_operation(predefined);
            }
        };

        match macro_type {
            MacroType::Calorie => self.calorie.push(*datum),
            MacroType::Carb => self.carb.push(*datum),
            MacroType::Fat => self.fat.push(*datum),
            MacroType::Protein => self.protein.push(*datum),
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::common::utils;

    #[test]
    fn check_data_fields_eq() {
        let file_path = format!("{}/test_data/good_data/data_1.txt", utils::user_test_path());
        let mut test_data = utils::instantiate_macro_counter(file_path);

        MacroCounter::compile_data(&mut test_data, false);

        let test_cal: Vec<f32> = Vec::from([15.0, 300.0, 200.0, 180.0, 180.0]);
        let test_fat: Vec<f32> = Vec::from([7.7, 0.1, 11.0, 2.0]);

        assert_eq!(&test_data.calorie, &test_cal);
        assert_ne!(&test_data.fat, &test_fat);
    }

    #[test]
    #[should_panic]
    fn check_data_fields_ne() {
        let file_path = format!("{}/test_data/good_data/data_1.txt", utils::user_test_path());
        let mut test_data = utils::instantiate_macro_counter(file_path);

        MacroCounter::compile_data(&mut test_data, false);

        let test_carb: Vec<f32> = Vec::from([4.0, 58.0, 24.0, 21.0, 21.0]);
        let test_protein: Vec<f32> = Vec::from([42.0, 22.8, 10.1, 62.1]);

        assert_ne!(&test_data.carb, &test_carb);
        assert_eq!(&test_data.protein, &test_protein);
    }

    #[test]
    #[should_panic]
    fn compile_bad_data() {
        let file_path = format!("{}/test_data/bad_data/data.txt", utils::user_test_path());
        let mut test_data = utils::instantiate_macro_counter(file_path);
        MacroCounter::compile_data(&mut test_data, false);
    }

    #[test]
    fn check_repeat_data_entry() {
        let file_path = format!("{}/test_data/good_data/data_1.txt", utils::user_test_path());
        let mut test_data = utils::instantiate_macro_counter(file_path);

        MacroCounter::compile_data(&mut test_data, false);

        let expected_cal: Vec<f32> = Vec::from([15.0, 300.0, 200.0, 180.0, 180.0, 180.0]);
        let expected_carb: Vec<f32> = Vec::from([4.0, 58.0, 24.0, 21.0, 21.0, 21.0]);

        MacroCounter::repeat_data_entry(&mut test_data, false, MacroType::Calorie);
        MacroCounter::repeat_data_entry(&mut test_data, false, MacroType::Fat);
        MacroCounter::repeat_data_entry(&mut test_data, false, MacroType::Carb);
        MacroCounter::repeat_data_entry(&mut test_data, false, MacroType::Protein);

        assert_eq!(test_data.calorie, expected_cal);
        assert_eq!(test_data.carb, expected_carb);

        test_data.calorie.pop();
        test_data.fat.pop();
        test_data.carb.pop();
        test_data.protein.pop();
    }
}
