use chrono::Local;
use dirs;
use std::{fs, path};

pub struct Pathing {
    pub dir_path: String,
    pub file_path: String,
}

impl Pathing {
    pub fn create_file(&self) {
        match fs::create_dir_all(&self.dir_path) {
            Ok(_) => (),
            Err(err) => {
                dbg!(err);
                panic!("Error: creating '{}'", self.dir_path);
            }
        };

        let _ = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.file_path);
    }

    pub fn generate_file_path() -> Pathing {
        // NOTE: it may become necessary to further separate files into {year}/{month}/{day}
        let user_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => panic!("Error: unable to determine $HOME directory"),
        };
        let parent_dir = format!(
            "{}/Documents/Health/Macronutritional_Intake",
            user_dir.to_str().unwrap()
        );
        let full_path = format!("{}/{}.txt", parent_dir, Local::now().date_naive());
        let mut file_path = String::new();
        let mut dir_path = String::new();

        for (i, val) in full_path.split("-").enumerate() {
            match i {
                // parent dir + year
                0 => {
                    let tmp = format!("{}-", val);
                    dir_path.push_str(&tmp);
                }
                // month
                1 => {
                    dir_path.push_str(val);
                    let tmp = format!("{}/", &dir_path);
                    file_path.push_str(&tmp);
                }
                // day
                2 => file_path.push_str(val),
                _ => {
                    dbg!(i, val);
                    panic!("Error: generating file path");
                }
            }
        }

        let pathing = Pathing {
            file_path,
            dir_path,
        };

        Pathing::create_file(&pathing);
        return pathing;
    }
}

pub fn file_exists(path: &String) -> bool {
    return path::Path::new(&path).exists();
}

// generates the user's absolute path of `macro_counter`
pub fn user_path() -> String {
    // let dir_path = match fs::canonicalize("..") {
    //     Ok(path) => path,
    //     Err(err) => {
    //         dbg!(err);
    //         panic!();
    //     }
    // };
    let dir_path = match dirs::home_dir() {
        Some(dir) => dir,
        None => panic!("Error: unable to determine $HOME directory"),
    };

    return dir_path.to_str().unwrap().to_string();
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn instantiate_test_paths() -> Pathing {
        let dir_path = match fs::canonicalize("./test_data/good_data") {
            Ok(path) => path,
            Err(err) => {
                dbg!(err);
                panic!();
            }
        };
        let dir_path = dir_path.to_str().unwrap().to_string();

        let file_path = format!("{}/data_1.txt", dir_path);
        let test_pathing = Pathing {
            file_path,
            dir_path,
        };

        return test_pathing;
    }

    #[test]
    fn test_create_file() {
        let test_pathing = instantiate_test_paths();

        Pathing::create_file(&test_pathing);
    }

    #[test]
    fn test_file_exits() {
        let test_pathing = instantiate_test_paths();

        assert!(file_exists(&test_pathing.file_path));
    }
}
