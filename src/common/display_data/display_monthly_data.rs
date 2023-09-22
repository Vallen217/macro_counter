use super::DisplayData;
use crate::common::utils::pad_word;
use std::fs;

impl DisplayData {
    pub fn compile_monthly_data(&mut self) -> (Vec<String>, Vec<String>) {
        let directory = fs::read_dir(&self.dir_path).unwrap();

        // generate initial `self.macro_totals` sub-vectors to deter index errors.
        for _ in 0..4 {
            self.macro_totals.push(vec![]);
        }

        for file in directory {
            let mut totals_line = String::new();
            let mut is_totals_line = false;

            for i in fs::read_to_string(file.unwrap().path()).unwrap().lines() {
                if is_totals_line {
                    totals_line.push_str(i);
                    is_totals_line = false;
                }

                // The next iteration will be the totals file line required.
                if i.starts_with("Total") {
                    is_totals_line = true;
                }
            }

            for (i, mut val) in totals_line.split_whitespace().enumerate() {
                if val.contains("g") {
                    val = &val[0..val.len() - 1];
                }
                self.macro_totals[i].push(val.parse().unwrap());
            }
        }

        return self.parse_monthly_data();
    }

    fn parse_monthly_data(&mut self) -> (Vec<String>, Vec<String>) {
        let dir = fs::read_dir(&self.dir_path).unwrap();
        let mut dir_len: f32 = 0.0;
        for _ in dir {
            dir_len += 1.0;
        }

        let mut monthly_means: Vec<String> = Vec::new();
        for i in 0..4 {
            self.totals.push(self.macro_totals[i].iter().sum::<f32>());
            let mean = format!("{:.1}", self.totals[i] / dir_len);
            monthly_means.push(mean);
        }

        let mut monthly_rel_percent: Vec<String> = Vec::new();
        for i in 1..4 {
            monthly_rel_percent.push(format!(
                "{:.1}%",
                (100.0 / self.totals[1..].iter().sum::<f32>()) * self.totals[i]
            ));
        }

        return (monthly_means, monthly_rel_percent);
    }

    pub fn write_monthly_data(&mut self, parsed_data: (Vec<String>, Vec<String>)) {
        println!(
            "\n\nCal:{}Fat:{}Carb:{}Protein:{}",
            pad_word("Cal:"),
            pad_word("Fat:"),
            pad_word("Carb:"),
            pad_word("Protein:")
        );

        let monthly_means: Vec<String> = parsed_data.0;
        let monthly_rel_percent: Vec<String> = parsed_data.1;

        print!(
            "\nContemporary monthly total amounts:\n{}{}",
            self.totals[0],
            pad_word(&self.totals[0].to_string())
        );
        for val in &self.totals[1..] {
            let totals_val = format!("{}g", val);
            print!("{}{}", totals_val, pad_word(&totals_val));
        }

        print!(
            "\n\nMean daily amounts:\n{}{}",
            monthly_means[0],
            pad_word(&monthly_means[0].to_string())
        );
        for val in monthly_means[1..].iter() {
            let means_val = format!("{}g", val);
            print!("{}{}", means_val, pad_word(&means_val));
        }

        print!("\n\nMean daily relative percentages:\n");
        for val in monthly_rel_percent.iter() {
            print!("{}{}", val, pad_word(&val));
        }

        println!("\n");
    }
}

#[cfg(test)]
pub mod unit_tests {
    use super::*;

    fn instantiate_display_data() -> DisplayData {
        let dir_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/test_data/good_data/");
        let test_data = DisplayData {
            file_path: format!("{}data_1.txt", dir_path),
            dir_path,
            macro_totals: vec![],
            totals: Vec::new(),
        };

        return test_data;
    }

    #[test]
    fn check_display_macro_totals() {
        let mut test_data = instantiate_display_data();
        DisplayData::compile_monthly_data(&mut test_data);

        let mut expected_value: Vec<f32> = Vec::new();
        expected_value.push(1795.0);
        expected_value.push(34.0);
        expected_value.push(280.0);
        expected_value.push(98.0);

        assert_eq!(test_data.totals, expected_value);
    }

    #[test]
    fn check_monthly_means() {
        let mut test_data = instantiate_display_data();
        let resultant_values = DisplayData::compile_monthly_data(&mut test_data);

        let mut expected_value: Vec<String> = Vec::new();
        expected_value.push("897.5".to_string());
        expected_value.push("17.0".to_string());
        expected_value.push("140.0".to_string());
        expected_value.push("49.0".to_string());

        assert_eq!(resultant_values.0, expected_value);
    }

    #[test]
    fn check_monthly_rel_percent() {
        let mut test_data = instantiate_display_data();
        let resultant_values = DisplayData::compile_monthly_data(&mut test_data);

        let mut expected_value: Vec<String> = Vec::new();
        expected_value.push("8.3%".to_string());
        expected_value.push("68.0%".to_string());
        expected_value.push("23.8%".to_string());

        assert_eq!(resultant_values.1, expected_value);
    }
}
