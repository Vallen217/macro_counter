use std::fs;

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
