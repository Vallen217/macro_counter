pub mod display_monthly_data;
pub mod display_previous_data;

use std::fs;

pub struct DisplayData {
    pub file_path: String,
    pub dir_path: String,
    pub macro_totals: Vec<Vec<f32>>,
    pub totals: Vec<f32>,
}

impl DisplayData {
    pub fn display_file(&self, previous_data: Option<String>) {
        let file_path = match previous_data {
            Some(file_path) => file_path,
            None => self.file_path.clone(),
        };

        match fs::read_to_string(&file_path) {
            Ok(data) => println!("\n{}", data),
            Err(_) => {
                println!("Error: unable to read '{}'", file_path);
                return crate::main();
            }
        };
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::instantiate_display_data;
    use crate::pathing;

    #[test]
    #[ignore]
    fn test_display_data() {
        let dir_path = format!(
            "{}/Workspace/rust_macro_counter/test_data/good_data/",
            pathing::user_path()
        );
        let file_path = format!("{}data_1.txt", dir_path);
        let test_data = instantiate_display_data(file_path, dir_path);

        DisplayData::display_file(&test_data, None);
    }
}
