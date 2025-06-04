use super::DisplayData;
use crate::common::utils::pad_word;
use std::fs;

impl DisplayData {
    fn compile_dir_data(&mut self) -> (Vec<String>, Vec<String>) {
        // if DisplayData::compile_dir_data() is called repeatedly within
        // a single program runtime, these fields will continually aggregate.
        self.macro_totals.clear();
        self.totals.clear();

        let directory = match fs::read_dir(&self.dir_path) {
            Ok(dir) => dir,
            Err(err) => {
                dbg!(err);
                panic!("Error: unable to read '{}'", self.dir_path);
            }
        };

        // generate initial `self.macro_totals` sub-vectors to deter index errors.
        for _ in 0..4 {
            self.macro_totals.push(vec![]);
        }

        for file in directory {
            let mut totals_line = String::new();
            // the 2nd to last line of the file, which has the data we want to compile.
            let mut is_totals_line = false;

            for i in fs::read_to_string(file.as_ref().unwrap().path())
                .unwrap()
                .lines()
            {
                if is_totals_line {
                    totals_line.push_str(i);
                    is_totals_line = false;
                }

                // The next iteration will be the totals_file_line required.
                if i.starts_with("Total") {
                    is_totals_line = true;
                }
            }

            for (i, mut val) in totals_line.split_whitespace().enumerate() {
                if val.contains("g") {
                    val = &val[0..val.len() - 1];
                }
                // delete the file to prevent scewed dir data
                // if it exists but contains no real data.
                if val == 0.to_string() {
                    match fs::remove_file(file.as_ref().unwrap().path()) {
                        Ok(_) => {
                            return self.compile_dir_data();
                        }
                        Err(error) => {
                            dbg!(error);
                        }
                    }
                }
                self.macro_totals[i].push(val.parse().unwrap());
            }
        }

        self.parse_dir_data()
    }

    // parses data gathered from DisplayData::compile_data()
    // into pieces of data we want to disply and returns them as Strings.
    fn parse_dir_data(&mut self) -> (Vec<String>, Vec<String>) {
        let dir = match fs::read_dir(&self.dir_path) {
            Ok(dir) => dir,
            Err(err) => {
                dbg!(err);
                panic!("Error: unable to read '{}'", self.dir_path);
            }
        };
        // get the number of files in the dir to calculate various dir means.
        let mut dir_len: f32 = 0.0;
        for _ in dir {
            dir_len += 1.0;
        }

        let mut dir_means: Vec<String> = Vec::new();
        for i in 0..4 {
            self.totals.push(self.macro_totals[i].iter().sum::<f32>());
            let mean = format!("{:.1}", self.totals[i] / dir_len);
            dir_means.push(mean);
        }
        let mut dir_rel_percent: Vec<String> = Vec::new();
        for i in 1..4 {
            dir_rel_percent.push(format!(
                "{:.1}%",
                (100.0 / self.totals[1..].iter().sum::<f32>()) * self.totals[i]
            ));
        }

        (dir_means, dir_rel_percent)
    }

    pub fn display_dir_data(&mut self, temp_dir: bool) {
        println!(
            "\n\nCal:{}Fat:{}Carb:{}Protein:{}",
            pad_word("Cal:", 12),
            pad_word("Fat:", 12),
            pad_word("Carb:", 12),
            pad_word("Protein:", 12)
        );

        let parsed_data = self.compile_dir_data();
        let dir_means: Vec<String> = parsed_data.0;
        let dir_rel_percent: Vec<String> = parsed_data.1;

        print!(
            "\nContemporary directory total amounts:\n{}{}",
            self.totals[0],
            pad_word(&self.totals[0].to_string(), 12)
        );
        for val in &self.totals[1..] {
            let totals_val = format!("{}g", val);
            print!("{}{}", totals_val, pad_word(&totals_val, 12));
        }

        print!(
            "\n\nMean daily amounts:\n{}{}",
            dir_means[0],
            pad_word(&dir_means[0].to_string(), 12)
        );
        for val in dir_means[1..].iter() {
            let means_val = format!("{}g", val);
            print!("{}{}", means_val, pad_word(&means_val, 12));
        }

        print!("\n\nMean daily relative percentages:\n");
        print!("{}", pad_word("", 12));
        for val in dir_rel_percent.iter() {
            print!("{}{}", val, pad_word(&val, 12));
        }

        if temp_dir {
            match std::fs::remove_dir_all(&self.dir_path) {
                Ok(ok) => ok,
                Err(err) => {
                    println!("Error removing temporary directory: {}", temp_dir);
                    dbg!(err);
                }
            };
        }

        crate::main();
    }
}

#[cfg(test)]
pub mod unit_tests {
    use super::*;
    use crate::common::utils;

    fn instantiate_display_data() -> DisplayData {
        let dir_path = format!("{}/test_data/good_data/", utils::user_test_path());
        let test_data = DisplayData {
            file_path: format!("{}data_1.txt", dir_path),
            dir_path,
            macro_totals: vec![],
            totals: Vec::new(),
        };

        test_data
    }

    #[test]
    fn check_display_macro_totals() {
        let mut test_data = instantiate_display_data();
        DisplayData::compile_dir_data(&mut test_data);

        let mut expected_value: Vec<f32> = Vec::new();
        expected_value.push(2555.0);
        expected_value.push(63.0);
        expected_value.push(357.0);
        expected_value.push(147.0);

        assert_eq!(test_data.totals, expected_value);
    }

    #[test]
    fn check_dir_means() {
        let mut test_data = instantiate_display_data();
        let resultant_values = DisplayData::compile_dir_data(&mut test_data);

        let mut expected_value: Vec<String> = Vec::new();
        expected_value.push("851.7".to_string());
        expected_value.push("21.0".to_string());
        expected_value.push("119.0".to_string());
        expected_value.push("49.0".to_string());

        assert_eq!(resultant_values.0, expected_value);
    }

    #[test]
    fn check_dir_rel_percent() {
        let mut test_data = instantiate_display_data();
        let resultant_values = DisplayData::compile_dir_data(&mut test_data);

        let mut expected_value: Vec<String> = Vec::new();
        expected_value.push("11.1%".to_string());
        expected_value.push("63.0%".to_string());
        expected_value.push("25.9%".to_string());

        assert_eq!(resultant_values.1, expected_value);
    }
}
