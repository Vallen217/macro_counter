use super::*;
use std::{fs, io};

impl DisplayData {
    pub fn generate_previous_path(&mut self, parent_dir: String, monthly_data: bool) -> String {
        let parent_directory = fs::read_dir(&parent_dir).unwrap();

        println!("\nEnter a relative path from:");
        for dir in parent_directory {
            println!("{}", dir.unwrap().path().display())
        }

        let mut dir = String::new();
        io::stdin().read_line(&mut dir).unwrap();
        let dir_path = format!("{}/{}", parent_dir, &dir[0..dir.len() - 1]);

        if !Pathing::file_exists(&dir_path) {
            dbg!(&dir_path);
            panic!("Error: invalid directory");
        }

        if monthly_data {
            return dir_path;
        }

        let directory = fs::read_dir(&dir_path).unwrap();
        for file in directory {
            println!("{}", file.unwrap().path().display());
        }

        let mut file = String::new();
        io::stdin().read_line(&mut file).unwrap();
        let file_path = format!("{}/{}", dir_path, &file[0..file.len() - 1]);

        if !Pathing::file_exists(&file_path) {
            dbg!(&file_path);
            panic!("Error: invalid file");
        }
        return file_path;
    }

    pub fn display_previous_data(&mut self, parent_dir: String, monthly_data: bool) {
        let path = self.generate_previous_path(parent_dir, monthly_data);

        if monthly_data {
            self.dir_path = path;
            dbg!(&self.dir_path);
            let parsed_data: (Vec<String>, Vec<String>) = DisplayData::compile_monthly_data(self);
            DisplayData::write_monthly_data(self, parsed_data);
        } else {
            self.display_data(Some(path));
        }
    }
}
