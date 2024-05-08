use chrono::Local;
use dirs;
use std::{fs, io, path, process};

pub struct Date {
    pub year: i16,
    pub month: i16,
    pub day: i16,
}

pub struct Pathing {
    pub year_path: String,
    pub month_path: String,
    pub day_path: String,
}

impl Date {
    pub fn current_date() -> Date {
        let chrono_date = format!("{}", Local::now().date_naive());
        let mut date_segments = vec![];

        for val in chrono_date.split("-") {
            let val: i16 = match val.parse() {
                Ok(data) => data,
                Err(error) => {
                    dbg!(error);
                    panic!("Error: compiling current date '{}'", val);
                }
            };
            date_segments.push(val);
        }

        let date = Date {
            year: date_segments[0],
            month: date_segments[1],
            day: date_segments[2],
        };

        date
    }
}

impl Pathing {
    pub fn generate_file_path(date: Date) -> Pathing {
        let user_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => panic!("Error: unable to determine $HOME directory"),
        };
        let parent_dir = format!(
            "{}/Documents/Health/Macronutritional_Intake",
            user_dir.to_str().unwrap()
        );

        let pathing = Pathing {
            year_path: format!("{parent_dir}/{}", date.year),
            month_path: format!("{parent_dir}/{}/{}", date.year, date.month),
            day_path: format!("{parent_dir}/{}/{}/{}.txt", date.year, date.month, date.day),
        };

        Pathing::create_file(&pathing);
        pathing
    }

    pub fn create_file(&self) {
        match fs::create_dir_all(&self.month_path) {
            Ok(_) => (),
            Err(err) => {
                dbg!(err);
                panic!("Error: creating '{}'", self.month_path);
            }
        };

        let _ = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.day_path);
    }
}

pub fn file_exists(path: &String) -> bool {
    path::Path::new(&path).exists()
}

pub fn user_path() -> String {
    let dir_path = match dirs::home_dir() {
        Some(dir) => dir,
        None => panic!("Error: unable to determine $HOME directory"),
    };

    dir_path.to_str().unwrap().to_string()
}

pub fn user_input_pathing(parent_directory: String, date_type: &str) -> String {
    let parent_dir = match fs::read_dir(&parent_directory) {
        Ok(dir) => dir,
        Err(err) => {
            dbg!(err);
            panic!("Error: unable to read '{}'", parent_directory);
        }
    };

    println!("\nEnter a {} from:", date_type);
    for path in parent_dir {
        println!("{}", path.unwrap().path().display());
    }

    let mut path = String::new();
    match io::stdin().read_line(&mut path) {
        Ok(path) => path,
        Err(err) => {
            dbg!(err);
            panic!("Error: unable to read '{}'", path);
        }
    };

    // This should be in an 'else' block, but the compiler can't find the variable in scope
    // unless it's declared outright like this. And I dont care enough to fix it right now.
    let mut formatted_path = format!("{}/{}", parent_directory, &path[0..path.len() - 1]);

    if date_type.contains("day") || date_type.contains("pd file") {
        if path.contains(".txt") {
            formatted_path = format!("{}/{}", parent_directory, &path[0..path.len() - 1]);
        } else {
            formatted_path = format!("{}/{}.txt", parent_directory, &path[0..path.len() - 1]);
        }
    }

    // for user to quit early
    if formatted_path.contains("q") {
        process::exit(0);
    }

    if !file_exists(&formatted_path) {
        println!("\nError: Invalid selection");
        return user_input_pathing(parent_directory, date_type);
    }

    formatted_path
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn instantiate_test_paths() -> Pathing {
        let month_path = match fs::canonicalize("./test_data/good_data") {
            Ok(path) => path,
            Err(err) => {
                dbg!(err);
                panic!();
            }
        };
        let month_path = month_path.to_str().unwrap().to_string();

        let day_path = format!("{}/data_1.txt", month_path);
        let test_pathing = Pathing {
            year_path: "none".to_string(),
            month_path,
            day_path,
        };

        test_pathing
    }

    #[test]
    fn test_create_file() {
        let test_pathing = instantiate_test_paths();

        Pathing::create_file(&test_pathing);
    }

    #[test]
    fn test_file_exits() {
        let test_pathing = instantiate_test_paths();

        assert!(file_exists(&test_pathing.day_path));
    }
}
