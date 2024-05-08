use super::*;
use crate::pathing;

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
            let year_path = pathing::user_input_pathing(parent_dir.clone(), "year");
            year_path
        };

        let month_path = pathing::user_input_pathing(dir_path.clone(), "month");

        if monthly_data && !predefined {
            return month_path;
        }

        let day_path = pathing::user_input_pathing(month_path.clone(), "day");

        day_path
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
