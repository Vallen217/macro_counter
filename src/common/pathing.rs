use chrono::Local;
use std::{fs, path};

pub struct Pathing {
    pub dir_path: String,
    pub file_path: String,
}

impl Pathing {
    pub fn create_file(&self, monthly_data: bool) {
        fs::create_dir_all(&self.dir_path).unwrap();

        let file_path: String = if monthly_data {
            format!("{}.txt", self.dir_path.clone())
        } else {
            self.file_path.clone()
        };

        let _ = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path);
    }

    pub fn generate_file_path() -> Pathing {
        // NOTE: it may become necessary to further separate files into {year}/{month}/{day}
        let parent_dir = String::from("/home/vallen/Workspace/rust_macro_counter/data_files");
        let full_path = format!("{}/{}.txt", parent_dir, Local::now().date_naive());
        let mut file_path = String::new();
        let mut dir_path = String::new();

        for (i, val) in full_path.split("-").enumerate() {
            match i {
                0 => {
                    let tmp = format!("{}-", val);
                    dir_path.push_str(&tmp);
                }
                1 => {
                    dir_path.push_str(val);
                    let tmp = format!("{}/", &dir_path);
                    file_path.push_str(&tmp);
                }
                2 => file_path.push_str(val),
                _ => panic!("3"),
            }
        }

        let pathing = Pathing {
            file_path,
            dir_path,
        };

        Pathing::create_file(&pathing, false);
        return pathing;
    }

    pub fn file_exists(path: &String) -> bool {
        return path::Path::new(&path).exists();
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_file_exists() {
        let dir_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/test_data/good_data");
        let file_path = String::from(
            "/home/vallen/Workspace/rust_macro_counter/test_data/good_data/data_1.txt",
        );
        let test_pathing = Pathing {
            dir_path,
            file_path,
        };

        Pathing::create_file(&test_pathing, false);
    }
}
