use std::fs;

// TODO: instantiate MacorCounter

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
    pub calories: i64,
    pub fat: Vec<f32>,
    pub carb: Vec<f32>,
    pub protein: Vec<f32>,
    pub totals: Vec<f32>,
}
