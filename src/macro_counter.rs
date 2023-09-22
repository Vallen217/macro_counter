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
        }

        let file_data = fs::read_to_string(self.file_path.clone()).expect("Error reading file");

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
                    let datum: f32 = datum.parse().unwrap();

                    match iter {
                        0 => self.calorie.push(datum),
                        1 => self.fat.push(datum),
                        2 => self.carb.push(datum),
                        3 => self.protein.push(datum),
                        _ => panic!("4"),
                    };
                }
            }
        }
    }

    pub fn get_operation(&mut self) {
        let mut operation = String::from("");
        io::stdin().read_line(&mut operation).unwrap();

        if operation.trim() == "q" {
            return crate::main();
        }

        let re = Regex::new(r"rlq?[0-9]*").unwrap();
        if re.is_match(&operation) {
            return self.remove_data(operation);
        }

        if operation.contains("q") {
            return crate::main();
        } else {
            return self.collect_data();
        }
    }

    fn remove_data(&mut self, operation: String) {
        loop {
            let iter: i8 = if operation.contains("q") {
                operation.clone().trim()[3..].parse().unwrap()
            } else {
                println!("{}", operation.clone().trim());
                operation.clone().trim()[2..].parse().unwrap()
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
                return self.collect_data();
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn instantiate_macro_counter(file_path: Option<String>) -> MacroCounter {
        let good_data_path = String::from(
            "/home/vallen/Workspace/rust_macro_counter/test_data/good_data/data_1.txt",
        );
        let file_path = match file_path {
            Some(file_path) => file_path,
            None => good_data_path,
        };

        let test_data = MacroCounter {
            file_path,
            calorie: Vec::new(),
            fat: Vec::new(),
            carb: Vec::new(),
            protein: Vec::new(),
            totals: Vec::new(),
        };

        return test_data;
    }

    #[test]
    fn check_data_fields_eq() {
        let mut test_data = instantiate_macro_counter(None);

        MacroCounter::compile_data(&mut test_data, false);

        let test_cal: Vec<f32> = Vec::from([180.0, 180.0, 280.0, 280.0]);
        let test_fat: Vec<f32> = Vec::from([7.7, 0.1, 11.0, 2.0]);
        let test_carb: Vec<f32> = Vec::from([21.0, 21.0, 55.0, 55.0]);
        let test_protein: Vec<f32> = Vec::from([42.0, 22.8, 10.1, 62.1]);

        assert_eq!(&test_data.calorie, &test_cal);
        assert_ne!(&test_data.fat, &test_fat);
        assert_eq!(&test_data.carb, &test_carb);
        assert_ne!(&test_data.protein, &test_protein);
    }

    #[test]
    #[should_panic]
    fn check_data_fields_ne() {
        let mut test_data = instantiate_macro_counter(None);

        MacroCounter::compile_data(&mut test_data, false);

        let test_cal: Vec<f32> = Vec::from([180.0, 180.0, 280.0, 280.0]);
        let test_fat: Vec<f32> = Vec::from([7.7, 0.1, 11.0, 2.0]);
        let test_carb: Vec<f32> = Vec::from([21.0, 21.0, 55.0, 55.0]);
        let test_protein: Vec<f32> = Vec::from([42.0, 22.8, 10.1, 62.1]);

        assert_ne!(&test_data.calorie, &test_cal);
        assert_eq!(&test_data.fat, &test_fat);
        assert_ne!(&test_data.carb, &test_carb);
        assert_eq!(&test_data.protein, &test_protein);
    }

    #[test]
    #[should_panic]
    fn compile_bad_data() {
        let file_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/test_data/bad_data/data.txt");
        let mut test_data = instantiate_macro_counter(Some(file_path));
        MacroCounter::compile_data(&mut test_data, false);
    }

    #[test]
    fn test_remove_data() {
        let mut test_data = instantiate_macro_counter(None);
        let operation = String::from("rlq2");

        let expected_cal: Vec<f32> = Vec::from([180.0, 180.0]);
        let expected_fat: Vec<f32> = Vec::from([6.0, 6.0]);

        MacroCounter::compile_data(&mut test_data, false);
        MacroCounter::remove_data(&mut test_data, operation);

        let resultant_cal: Vec<f32> = test_data.calorie.clone();
        let resultant_fat: Vec<f32> = test_data.fat.clone();

        assert_eq!(expected_cal, resultant_cal);
        assert_eq!(expected_fat, resultant_fat);
    }
}
