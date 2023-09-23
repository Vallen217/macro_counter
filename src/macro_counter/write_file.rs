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
        append_file.write(string_totals.as_bytes()).unwrap();
        // NOTE: disabled for testing.
        // return self.get_operation();
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
    fn test_write_file() {
        let mut test_data = instantiate_macro_counter(None);

        MacroCounter::compile_data(&mut test_data, false);
        MacroCounter::write_file(&mut test_data);
        dbg!(&test_data.calorie);
        MacroCounter::compile_data(&mut test_data, true);
        dbg!(&test_data.calorie);
    }

    #[test]
    fn check_macro_counter_totals() {
        let mut test_data = instantiate_macro_counter(None);
        let expected_values: Vec<f32> = vec![920.0, 16.0, 152.0, 44.0];

        MacroCounter::compile_data(&mut test_data, false);
        MacroCounter::compile_totals(&mut test_data);
        MacroCounter::compile_data(&mut test_data, true);

        assert_eq!(test_data.totals, expected_values);
    }

    #[test]
    fn check_macro_percents() {
        let mut test_data = instantiate_macro_counter(None);
        let expected_values: Vec<&str> = vec!["7.5%        ", "71.7%       ", "20.8%       "];

        MacroCounter::compile_data(&mut test_data, false);
        let resultant_values: Vec<String> = MacroCounter::compile_totals(&mut test_data);
        MacroCounter::compile_data(&mut test_data, true);

        assert_eq!(resultant_values, expected_values);
    }
}
