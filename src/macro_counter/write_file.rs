use super::*;
use crate::common::utils::pad_word;
use std::fs::{self, OpenOptions};
use std::io::Write;

impl MacroCounter {
    // Macro ratios by grams
    fn compile_gram_totals(&mut self) -> Vec<String> {
        self.totals.push(self.calorie.iter().sum());
        self.totals.push(self.fat.iter().sum());
        self.totals.push(self.carb.iter().sum());
        self.totals.push(self.protein.iter().sum());

        let ratio: f32 = 100.0 / (self.totals[1] + self.totals[2] + self.totals[3]);
        let mut rel_per: Vec<String> = Vec::new();
        for i in 1..4 {
            let percent = format!("{:.1}%", ratio * self.totals[i]);
            let format_per = format!("{}{}", percent, pad_word(&percent, 12));
            rel_per.push(format_per.clone());
        }

        rel_per
    }

    // Macro ratios by calories
    fn compile_cal_totals(&mut self) -> Vec<String> {
        self.totals.push(self.calorie.iter().sum());
        self.totals.push(self.fat.iter().sum());
        self.totals.push(self.carb.iter().sum());
        self.totals.push(self.protein.iter().sum());

        let ratio: f32 = 100.0 / self.totals[0];
        let mut rel_per: Vec<String> = Vec::new();

        let fat_percent = format!("{:.1}%", &ratio * (self.totals[1] * 9.0));
        let format_per = format!("{}{}", fat_percent, pad_word(&fat_percent, 12));
        rel_per.push(format_per.clone());

        // Carb and protein macros
        for i in 2..4 {
            let percent = format!("{:.1}%", &ratio * (self.totals[i] * 4.0));
            let format_per = format!("{}{}", percent, pad_word(&percent, 12));
            rel_per.push(format_per.clone());
        }

        rel_per
    }

    fn generate_macro_string(&mut self, j: usize, i: usize) -> String {
        let macro_string: String = match j {
            0 => {
                let string_pad = self.calorie[i].clone().to_string();
                let temp_macro_string: String =
                    format!("{}{}", self.calorie[i], pad_word(&string_pad, 12));
                temp_macro_string
            }
            1 => {
                let string_pad = self.fat[i].clone().to_string();
                let temp_macro_string: String =
                    format!("{}g{}", self.fat[i], pad_word(&string_pad, 11));
                temp_macro_string
            }
            2 => {
                let string_pad = self.carb[i].clone().to_string();
                let temp_macro_string: String =
                    format!("{}g{}", self.carb[i], pad_word(&string_pad, 11));
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
            pad_word("Cal:", 12),
            pad_word("Fat:", 12),
            pad_word("Carb:", 12),
            pad_word("Protein:", 12)
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
        let rel_cal_per = self.compile_cal_totals();
        let rel_gram_per = self.compile_gram_totals();

        let total_calorie = format!("{}", self.totals[0]);
        let total_fat = format!("{}g", self.totals[1]);
        let total_carb = format!("{}g", self.totals[2]);
        let total_protein = format!("{}g", self.totals[3]);
        let string_totals = format!(
            "\n\nTotal Amounts & Macro Ratios by Calories & Grams:\
            \n{}{}{}{}{}{}{}\n\nCalories:{}{}{}{}\nGrams:{}{}{}{}",
            total_calorie,
            pad_word(&total_calorie, 12),
            total_fat,
            pad_word(&total_fat, 12),
            total_carb,
            pad_word(&total_carb, 12),
            total_protein,
            pad_word("Calories:", 12),
            rel_cal_per[0],
            rel_cal_per[1],
            rel_cal_per[2],
            pad_word("Grams:", 12),
            rel_gram_per[0],
            rel_gram_per[1],
            rel_gram_per[2]
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
        MacroCounter::compile_gram_totals(&mut test_data);

        assert_eq!(test_data.totals, expected_values);
    }

    #[test]
    fn check_macro_percents() {
        let file_path = format!("{}/test_data/good_data/data_2.txt", utils::user_test_path());
        let mut test_data = utils::instantiate_macro_counter(file_path);
        let expected_values: Vec<&str> = vec!["7.5%        ", "71.7%       ", "20.8%       "];

        MacroCounter::compile_data(&mut test_data, false);
        let resultant_values: Vec<String> = MacroCounter::compile_gram_totals(&mut test_data);

        assert_eq!(resultant_values, expected_values);
    }
}
