pub mod input_data;
pub mod write_file;

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

        if operation.trim() == "q" {
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
            match predefined {
                true => return super::common::predefined::predefined(),
                false => return crate::main(),
            };
        } else {
            return self.collect_data(predefined);
        }
    }

    fn remove_data(&mut self, operation: String, predefined: bool) {
        loop {
            let iter: i8 = if operation.contains("q") {
                match operation.clone().trim()[3..].parse() {
                    Ok(data) => data,
                    Err(error) => {
                        dbg!(error);
                        panic!("Error: parsing operation '{}'", operation);
                    }
                }
            } else {
                match operation.clone().trim()[2..].parse() {
                    Ok(data) => data,
                    Err(error) => {
                        dbg!(error);
                        panic!("Error: parsing operation '{}'", operation);
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
                break;
            } else {
                return self.collect_data(predefined);
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::common::utils::instantiate_macro_counter;
    use crate::pathing;

    #[test]
    fn check_data_fields_eq() {
        let file_path = format!("{}/test_data/good_data/data_1.txt", pathing::user_path());
        let mut test_data = instantiate_macro_counter(file_path);

        MacroCounter::compile_data(&mut test_data, false);

        let test_cal: Vec<f32> = Vec::from([15.0, 300.0, 200.0, 180.0, 180.0]);
        let test_fat: Vec<f32> = Vec::from([7.7, 0.1, 11.0, 2.0]);

        assert_eq!(&test_data.calorie, &test_cal);
        assert_ne!(&test_data.fat, &test_fat);
    }

    #[test]
    #[should_panic]
    fn check_data_fields_ne() {
        let file_path = format!("{}/test_data/good_data/data_1.txt", pathing::user_path());
        let mut test_data = instantiate_macro_counter(file_path);

        MacroCounter::compile_data(&mut test_data, false);

        let test_carb: Vec<f32> = Vec::from([4.0, 58.0, 24.0, 21.0, 21.0]);
        let test_protein: Vec<f32> = Vec::from([42.0, 22.8, 10.1, 62.1]);

        assert_ne!(&test_data.carb, &test_carb);
        assert_eq!(&test_data.protein, &test_protein);
    }

    #[test]
    #[should_panic]
    fn compile_bad_data() {
        let file_path = format!("{}/test_data/bad_data/data.txt", pathing::user_path());
        let mut test_data = instantiate_macro_counter(file_path);
        MacroCounter::compile_data(&mut test_data, false);
    }
}
