use regex::Regex;
use std::{fs, io, process::exit};

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

enum MacroType {
    Calorie,
    Fat,
    Carb,
    Protein,
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
    // compile prexisting data, read from a file.
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

    pub fn remove_data(&mut self, operation: String) {
        loop {
            let iter: i8 = if operation.contains("q") {
                operation.clone().trim()[3..].parse().unwrap()
            } else {
                println!("{}", operation.clone().trim());
                operation.clone().trim()[2..].parse().unwrap()
            };

            for _ in 0..iter {
                self.calories.pop();
                self.fat.pop();
                self.carb.pop();
                self.protein.pop();
            }

            if operation.trim().contains("q") {
                break;
            } else {
                return self.call_collect_data();
            }
        }
    }

    pub fn get_operation(&mut self) {
        println!("-");
        let mut operation = String::from("");
        io::stdin()
            .read_line(&mut operation)
            .expect("Error: failed to read stdin.");
        if operation.trim() == "q" {
            // TODO: return main(); loop && make case insensitive.
            exit(0);
        }

        let re = Regex::new(r"rlq?[0-9]*").unwrap();
        if re.is_match(&operation) {
            return self.remove_data(operation);
        }
        if operation.contains("q") {
            // TODO: return main(); loop && make case insensitive.
            exit(0);
        } else {
            return self.call_collect_data();
        }
    }

    // Making an additional function purely to call collect_data()
    // was the only way I thought of to keep it D.R.Y. and idiomatic.
    fn call_collect_data(&mut self) {
        self.collect_data(String::from("Calorie: "), MacroType::Calorie);
        self.collect_data(String::from("Carb: "), MacroType::Fat);
        self.collect_data(String::from("Fat: "), MacroType::Carb);
        self.collect_data(String::from("Protein: "), MacroType::Protein);
        self.get_operation();
    }

    fn collect_data(&mut self, macro_stdin: String, macro_type: MacroType) {
        println!("{}", macro_stdin);
        let mut macro_data = String::new();
        io::stdin()
            .read_line(&mut macro_data)
            .expect("Error: failed to read stdin.");
        let float_data: f32 = macro_data.trim().parse().unwrap();

        match macro_type {
            MacroType::Calorie => self.calories.push(float_data),
            MacroType::Fat => self.fat.push(float_data),
            MacroType::Carb => self.carb.push(float_data),
            MacroType::Protein => self.protein.push(float_data),
        };
        dbg!(&self.calories);
    }
}
