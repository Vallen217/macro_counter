use super::*;
use crate::pathing;
use std::{fs, io, process};

impl DisplayData {
    fn generate_previous_path(
        &mut self,
        parent_dir: String,
        monthly_data: bool,
        predefined: bool,
    ) -> String {
        let dir_path: String = if predefined {
            parent_dir.clone()
        } else {
            let parent_directory = match fs::read_dir(&parent_dir) {
                Ok(dir) => dir,
                Err(err) => {
                    dbg!(err);
                    panic!("Error: unable to read '{}'", parent_dir);
                }
            };

            println!("\nEnter a relative path from:");
            for dir in parent_directory {
                println!("{}", dir.unwrap().path().display())
            }

            let mut dir = String::new();
            match io::stdin().read_line(&mut dir) {
                Ok(dir) => dir,
                Err(err) => {
                    dbg!(err);
                    panic!("Error: unable to read '{}'", dir);
                }
            };

            // for user to quit early
            if dir.contains("q") {
                process::exit(0);
            }

            // to view the latest non-current file
            // if dir.contains("-1") {
            //     println!()
            // }

            let dir_path = format!("{}/{}", parent_dir, &dir[0..dir.len() - 1]);

            if !pathing::file_exists(&dir_path) {
                println!("\nError: invalid directory");
                return self.generate_previous_path(parent_dir, monthly_data, predefined);
            }
            dir_path
        };

        if monthly_data && !predefined {
            return dir_path;
        }

        // returns the last directory in the path as the new parent directory
        // if it is a yearly directory.
        //
        // Why? Because I don't want restructure this entire function
        // to refit the new temporal file separation method.
        let last_path_dir = dir_path.rsplit_once("/").unwrap().1;
        if last_path_dir.len() == 4 {
            return self.generate_previous_path(dir_path, monthly_data, predefined);
        }

        let directory = match fs::read_dir(&dir_path) {
            Ok(dir) => dir,
            Err(err) => {
                dbg!(err);
                panic!("Error: unable to read '{}'", parent_dir);
            }
        };

        println!("");
        for file in directory {
            println!("{}", file.unwrap().path().display());
        }

        let mut file_name = String::new();
        match io::stdin().read_line(&mut file_name) {
            Ok(file_name) => file_name,
            Err(_) => {
                println!("Error: unable to read '{}'", file_name);
                return self.generate_previous_path(parent_dir, monthly_data, predefined);
            }
        };

        // for user to quit early
        if file_name.contains("q") {
            process::exit(0);
        }

        let file_path: String = if file_name.contains(".txt") {
            format!("{}/{}", dir_path, &file_name[0..file_name.len() - 1])
        } else {
            format!("{}/{}.txt", dir_path, &file_name[0..file_name.len() - 1])
        };

        if !pathing::file_exists(&file_path) {
            println!("\nError: invalid file");
            return self.generate_previous_path(parent_dir, monthly_data, predefined);
        }
        file_path
    }

    pub fn display_previous_file(
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
            self.display_file(Some(path))
        }
    }
}
