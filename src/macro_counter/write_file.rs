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

        rel_percentage
    }

    fn generate_macro_string(&mut self, j: usize, i: usize) -> String {
        let macro_string: String = match j {
            0 => {
                let string_pad = self.calorie[i].clone().to_string();
                let temp_macro_string: String =
                    format!("{}{}", self.calorie[i], pad_word(&string_pad));
                temp_macro_string
            }
            1 => {
                let string_pad = self.fat[i].clone().to_string();
                let temp_macro_string: String =
                    format!("{}g{}", self.fat[i], pad_word(&string_pad));
                temp_macro_string
            }
            2 => {
                let string_pad = self.carb[i].clone().to_string();
                let temp_macro_string: String =
                    format!("{}g{}", self.carb[i], pad_word(&string_pad));
                temp_macro_string
            }
            3 => {
                let temp_macro_string: String = format!("{}g", self.protein[i]);
                temp_macro_string
            }
            _ => {
                dbg!(j);
                panic!("Error: iterator out of bounds while parsing file data");
            }
        };

        macro_string
    }

    pub fn write_file(&mut self) {
        let top_file_line = format!(
            "Cal:{}Fat:{}Carb:{}Protein:{}",
            pad_word("Cal:"),
            pad_word("Fat:"),
            pad_word("Carb:"),
            pad_word("Protein:")
        );
        fs::write(&self.file_path, top_file_line).expect("Error: unable to write to file.");

        let mut append_file = match OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.file_path)
        {
            Ok(append_file) => append_file,
            Err(err) => {
                dbg!(err);
                panic!("Error: opening '{}'", &self.file_path);
            }
        };

        for i in 0..self.calorie.len() {
            append_file
                .write("\n".as_bytes())
                .expect("Error: unable to write to file.");

            for j in 0..4 {
                let macro_string = self.generate_macro_string(j, i);
                match append_file.write(macro_string.as_bytes()) {
                    Ok(macro_string) => macro_string,
                    Err(err) => {
                        dbg!(err);
                        panic!("Error: writing data to '{}'", &self.file_path);
                    }
                };
            }
        }

        self.totals.clear();
        let rel_percentage = self.compile_totals();

        let total_calorie = format!("{}", self.totals[0]);
        let total_fat = format!("{}g", self.totals[1]);
        let total_carb = format!("{}g", self.totals[2]);
        let total_protein = format!("{}g", self.totals[3]);
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
        match append_file.write(string_totals.as_bytes()) {
            Ok(string_totals) => string_totals,
            Err(err) => {
                dbg!(err);
                panic!("Error: writing data to '{}'", &self.file_path);
            }
        };
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::common::utils;

    #[test]
    fn test_write_file() {
        let file_path = format!("{}/test_data/good_data/data_2.txt", utils::user_test_path());
        let mut test_data = utils::instantiate_macro_counter(file_path);

        MacroCounter::compile_data(&mut test_data, false);
        MacroCounter::write_file(&mut test_data);
    }

    #[test]
    fn check_macro_counter_totals() {
        let file_path = format!("{}/test_data/good_data/data_2.txt", utils::user_test_path());
        let mut test_data = utils::instantiate_macro_counter(file_path);
        let expected_values: Vec<f32> = vec![920.0, 16.0, 152.0, 44.0];

        MacroCounter::compile_data(&mut test_data, false);
        MacroCounter::compile_totals(&mut test_data);

        assert_eq!(test_data.totals, expected_values);
    }

    #[test]
    fn check_macro_percents() {
        let file_path = format!("{}/test_data/good_data/data_2.txt", utils::user_test_path());
        let mut test_data = utils::instantiate_macro_counter(file_path);
        let expected_values: Vec<&str> = vec!["7.5%        ", "71.7%       ", "20.8%       "];

        MacroCounter::compile_data(&mut test_data, false);
        let resultant_values: Vec<String> = MacroCounter::compile_totals(&mut test_data);

        assert_eq!(resultant_values, expected_values);
    }
}
