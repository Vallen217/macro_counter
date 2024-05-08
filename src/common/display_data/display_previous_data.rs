use super::*;
use crate::pathing;

impl DisplayData {
    pub fn display_previous_file(
        &mut self,
        parent_dir: String,
        monthly_data: bool,
        predefined: bool,
    ) {
        if predefined {
            let pd_path = pathing::user_input_pathing(parent_dir, "pd file");
            return self.display_file(Some(pd_path));
        };

        let year_path = pathing::user_input_pathing(parent_dir, "year");
        let month_path = pathing::user_input_pathing(year_path, "month");

        if monthly_data && !predefined {
            self.dir_path = month_path;
            self.display_monthly_data();
        } else {
            let day_path = pathing::user_input_pathing(month_path.clone(), "day");
            self.display_file(Some(day_path));
        }
    }
}
