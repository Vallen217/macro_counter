use crate::*;
use chrono::Local;
use std::fs;

pub struct Date {
    pub year: i16,
    pub month: i16,
    pub day: i16,
}

impl Date {
    pub fn current_date() -> Date {
        let chrono_date = format!("{}", Local::now().date_naive());
        let mut date_segments = vec![];

        for val in chrono_date.split("-") {
            let val: i16 = match val.parse() {
                Ok(data) => data,
                Err(error) => {
                    dbg!(error);
                    panic!("Error: compiling current date '{}'", val);
                }
            };
            date_segments.push(val);
        }

        let date = Date {
            year: date_segments[0],
            month: date_segments[1],
            day: date_segments[2],
        };

        date
    }

    pub fn decrement_date(&mut self, mut void_files: i8) -> &mut Self {
        if self.day != 1 {
            self.day -= 1;
        } else {
            if self.month != 1 {
                self.month -= 1;
                self.day = 31;
            } else {
                self.year -= 1;
                self.month = 12;
                self.day = 31;
            }
        }

        let current_path = pathing::Pathing::generate_file_path(&self, false);
        // If a number greater than the total number of files (dates) is passed,
        // the loop will result in a stack overflow.
        if void_files > 100 {
            println!(
                "Error: '{}' excceds the number of files available",
                void_files
            );
            crate::main()
        }

        if !pathing::file_exists(&current_path.day_path) {
            void_files += 1;
            return self.decrement_date(void_files);
        }
        self
    }
}

// Creates a temporary directory and copies 'n' files into it for display.
pub fn aggregate_recent_files(mut count: i16) {
    let user_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => panic!("Error: unable to determine $HOME directory"),
    };
    let temp_dir = format!(
        "{}/Documents/Health/Macronutritional_Intake/mctr_temp",
        user_dir.to_str().unwrap()
    );

    match fs::create_dir_all(&temp_dir) {
        Ok(_) => (),
        Err(err) => {
            dbg!(err);
            panic!("Error: creating: '{}'", temp_dir);
        }
    };

    let mut initial_date = Date::current_date();

    while count > 0 {
        let temp_date = Date::decrement_date(&mut initial_date, 0);
        let temp_file = format!("{}/{}.txt", temp_dir, temp_date.day);
        let real_pathing = Pathing::generate_file_path(temp_date, false);
        let temp_pathing = Pathing {
            year_path: String::new(),
            month_path: temp_dir.clone(),
            day_path: temp_file,
        };

        Pathing::create_file(&temp_pathing);
        let _ = fs::copy(real_pathing.day_path, temp_pathing.day_path);
        count -= 1
    }
}

// used to synchronize the indentation levels in .txt files.
pub fn pad_word(word: &str) -> String {
    let num_of_spaces = 12 - word.len();
    let padding = " ".repeat(num_of_spaces);
    padding
}

pub fn instantiate_macro_counter(file_path: String) -> MacroCounter {
    let macro_counter = MacroCounter {
        file_path,
        calorie: Vec::new(),
        fat: Vec::new(),
        carb: Vec::new(),
        protein: Vec::new(),
        totals: Vec::new(),
    };

    macro_counter
}

pub fn instantiate_display_data(file_path: String, dir_path: String) -> DisplayData {
    let display_data = DisplayData {
        file_path,
        dir_path,
        macro_totals: vec![],
        totals: Vec::new(),
    };

    display_data
}

#[allow(dead_code)]
pub fn user_test_path() -> String {
    let dir_path = match fs::canonicalize("..") {
        Ok(path) => path,
        Err(err) => {
            dbg!(err);
            panic!();
        }
    };

    format!("{}/macro_counter/", dir_path.to_str().unwrap().to_string())
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_pad_word() {
        let word: &str = "test";
        let padding: String = pad_word(word);
        let padded_word = String::from("        ");
        assert_eq!(padded_word, padding);
    }
}
