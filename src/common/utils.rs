use crate::*;
use std::fs;

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

// TODO: Get current date, decrement 1 from 'day', ignore non-existent files,
// set counter to 31 and subtract 1 from 'month' if 0 is reached.

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
