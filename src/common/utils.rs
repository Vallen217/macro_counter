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

    fn decrement_date(&mut self) -> String {
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
        format!("{}", current_path.day_path)
    }
}

// TODO: Create a temp dir, cp files into it,
// call display_monthly_data on temp dir if the number of files > 1 else call display_file,
// delete temp dir.
pub fn aggregate_recent_files(mut count: i16) {
    let mut initial_iter = true;
    let mut initial_date = Date::current_date();

    // If a number greater than the total number of files (dates) is passed,
    // the loop won't end.
    let mut void_files: i8 = 0;

    while count > 0 {
        let file: String = if initial_iter {
            let current_path = pathing::Pathing::generate_file_path(&initial_date, false);
            format!("{}", current_path.day_path)
        } else {
            Date::decrement_date(&mut initial_date)
        };
        initial_iter = false;

        if pathing::file_exists(&file) {
            dbg!("{:?}", &file);
            count -= 1;
        } else {
            void_files += 1;
            if void_files > 5 {
                println!("Error: the number received excceds the number of files available");
                break;
            }
        }
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
