use regex::Regex;
use std::{fs, io};

enum MacroType {
    Calorie,
    Fat,
    Carb,
    Protein,
}

pub struct MacroCounter {
    pub file_path: String,
    pub calorie: Vec<f32>,
    pub fat: Vec<f32>,
    pub carb: Vec<f32>,
    pub protein: Vec<f32>,
    pub totals: Vec<f32>,
}

pub mod write_file {
    use super::*;
    use crate::common::utils::pad_word;
    use std::fs::{self, OpenOptions};
    use std::io::Write;

    impl MacroCounter {
        fn compile_totals(&mut self) -> Vec<String> {
            self.totals.push(self.calorie.iter().sum());
            self.totals.push(self.fat.iter().sum());
            self.totals.push(self.carb.iter().sum());
            self.totals.push(self.protein.iter().sum());

            let ratio: f32 = 100.0 / (self.totals[1] + self.totals[2] + self.totals[3]);
            let mut rel_percentage: Vec<String> = Vec::new();
            for i in 1..4 {
                let percent_1 = format!("{:.1}%", ratio * self.totals[i]);
                let percent_2 = format!("{}{}", percent_1, pad_word(&percent_1));
                rel_percentage.push(percent_2.clone());
            }

            return rel_percentage;
        }

        fn generate_macro_string(&mut self, j: usize, i: usize) -> String {
            /* despite be of datatype: f32, if a field from `MacroCounter`
            is a whole number (i.e. end with a zero), the ".0" doesn't survive
            the conversion from f32 to String, needed for `macro_pad`.
            So it's added to the string manually. */
            let macro_string: String = match j {
                0 => {
                    let mut string_pad = self.calorie[i].clone().to_string();
                    string_pad.push_str(".0");
                    let temp_macro_string: String =
                        format!("{}.0{}", self.calorie[i], pad_word(&string_pad));
                    temp_macro_string
                }
                1 => {
                    let mut string_pad = self.fat[i].clone().to_string();
                    string_pad.push_str(".0g");
                    let temp_macro_string: String =
                        format!("{}.0g{}", self.fat[i], pad_word(&string_pad));
                    temp_macro_string
                }
                2 => {
                    let mut string_pad = self.carb[i].clone().to_string();
                    string_pad.push_str(".0g");
                    let temp_macro_string: String =
                        format!("{}.0g{}", self.carb[i], pad_word(&string_pad));
                    temp_macro_string
                }
                3 => {
                    let mut string_pad = self.protein[i].clone().to_string();
                    string_pad.push_str(".0g");
                    let temp_macro_string: String =
                        format!("{}.0g{}", self.protein[i], pad_word(&string_pad));
                    temp_macro_string
                }
                _ => panic!("4"),
            };

            return macro_string;
        }

        pub fn write_file(&mut self) {
            let top_file_line = format!(
                "Cal:{}Fat:{}Carb:{}Protein:{}",
                pad_word("Cal:"),
                pad_word("Fat:"),
                pad_word("Carb:"),
                pad_word("Protein:")
            );
            fs::write(&self.file_path, top_file_line).expect("Error: Reading file");

            let mut append_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&self.file_path)
                .unwrap();

            for i in 0..self.calorie.len() {
                append_file
                    .write("\n".as_bytes())
                    .expect("Error: Failed to write to file.");

                for j in 0..4 {
                    let macro_string = self.generate_macro_string(j, i);
                    append_file.write(macro_string.as_bytes()).unwrap();
                }
            }

            let rel_percentage = self.compile_totals();
            let total_calorie = format!("{}", self.totals[0]);
            let total_fat = format!("{}.0g", self.totals[1]);
            let total_carb = format!("{}.0g", self.totals[2]);
            let total_protein = format!("{}.0g", self.totals[3]);
            let string_totals = format!(
                "\n\nTotal Amounts & Relative Percentages:\
            \n{}{}{}{}{}{}{}\n{}{}{}{}",
                total_calorie,
                pad_word(&total_calorie),
                total_fat,
                pad_word(&total_fat),
                total_carb,
                pad_word(&total_carb),
                total_protein,
                " ".repeat(12),
                rel_percentage[0],
                rel_percentage[1],
                rel_percentage[2]
            );
            append_file.write(string_totals.as_bytes()).unwrap();
            // NOTE: disabled for testing.
            // return self.get_operation();
        }
    }
}

pub mod input_data {
    use super::*;
    use std::io;

    impl MacroCounter {
        // Having an additional function purely to call collect_data()
        // was the only way I thought of to keep it D.R.Y. and idiomatic.
        pub fn collect_data(&mut self) {
            self.push_data(String::from("Calorie: "), MacroType::Calorie);
            self.push_data(String::from("Carb: "), MacroType::Fat);
            self.push_data(String::from("Fat: "), MacroType::Carb);
            self.push_data(String::from("Protein: "), MacroType::Protein);
            self.get_operation();
            self.write_file();
        }

        fn push_data(&mut self, macro_stdin: String, macro_type: MacroType) {
            println!("{}", macro_stdin);
            let mut macro_data = String::new();
            io::stdin()
                .read_line(&mut macro_data)
                .expect("Error: failed to read stdin.");

            let float_data: f32 = match macro_data.trim().parse() {
                Ok(num) => num,
                Err(_) => 0.0,
            };

            match macro_type {
                MacroType::Calorie => self.calorie.push(float_data),
                MacroType::Fat => self.fat.push(float_data),
                MacroType::Carb => self.carb.push(float_data),
                MacroType::Protein => self.protein.push(float_data),
            };
        }
    }
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
            let re = Regex::new(r"\d+\.\dg?").unwrap();
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
    fn compile_data_eq_fields() {
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
    fn compile_data_ne_fields() {
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
    fn remove_data() {
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

    #[test]
    fn write_file() {
        let mut test_data = instantiate_macro_counter(None);

        MacroCounter::compile_data(&mut test_data, true);
        MacroCounter::write_file(&mut test_data);
        MacroCounter::compile_data(&mut test_data, false);
    }
}
