use super::*;
use std::{fs, io};

impl DisplayData {
    fn generate_previous_path(
        &mut self,
        parent_dir: String,
        monthly_data: bool,
        predefined: bool,
    ) -> String {
        let dir_path: String = if predefined {
            parent_dir
        } else {
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
            } else {
                dir_path
            }
        };

        if monthly_data && !predefined {
            return dir_path;
        }

        let directory = fs::read_dir(&dir_path).unwrap();
        for file in directory {
            println!("{}", file.unwrap().path().display());
        }

        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name).unwrap();

        let file_path: String = if file_name.contains(".txt") {
            format!("{}/{}", dir_path, &file_name[0..file_name.len() - 1])
        } else {
            format!("{}/{}.txt", dir_path, &file_name[0..file_name.len() - 1])
        };

        if !Pathing::file_exists(&file_path) {
            dbg!(&file_path);
            panic!("Error: invalid file");
        }
        return file_path;
    }

    pub fn display_previous_data(
        &mut self,
        parent_dir: String,
        monthly_data: bool,
        predefined: bool,
    ) {
        let path = self.generate_previous_path(parent_dir, monthly_data, predefined);

        if monthly_data {
            self.dir_path = path;
            DisplayData::display_monthly_data(self);
        } else {
            return self.display_data(Some(path));
        }
    }
}
