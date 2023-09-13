use regex::Regex;
use std::fs;

pub fn pad_word(word: &str) -> String {
    let num_of_spaces = 12 - word.len();
    let padding = " ".repeat(num_of_spaces);
    return padding;
}

pub struct Pathing {
    pub dir_path: String,
    pub file_path: String,
}

impl Pathing {
    pub fn check_file_exists(self) {
        fs::create_dir_all(self.dir_path).expect("Error creating directory");

        let _ = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(self.file_path);
    }
}

pub struct MacroCounter {
    pub file_path: String,
    pub calories: Vec<f32>,
    pub fat: Vec<f32>,
    pub carb: Vec<f32>,
    pub protein: Vec<f32>,
    pub totals: Vec<f32>,
}

impl MacroCounter {
    pub fn compile_data(&mut self) {
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
                        0 => self.calories.push(datum),
                        1 => self.fat.push(datum),
                        2 => self.carb.push(datum),
                        3 => self.protein.push(datum),
                        _ => panic!("Error: file iteration out of bounds."),
                    };
                }
            }
        }
    }
}
