mod common;
pub mod macro_counter;

use common::display_data::DisplayData;
use common::pathing::{self, Pathing};
use common::predefined;
use common::utils::*;
use macro_counter::MacroCounter;
use regex::Regex;
use std::{io, process::exit};

// TODO: swith rel_per from gram to calorie

fn main() {
    let pathing = Pathing::generate_file_path(&Date::current_date(), true);
    let mut macro_counter = instantiate_macro_counter(pathing.day_path.clone());
    let mut display_data =
        instantiate_display_data(pathing.day_path.clone(), pathing.month_path.clone());
    let user_dir = pathing::user_path();

    println!(
        "\n\n(mf)  - Modify file\
        \n(dr)  - Display the most recent, non-current file\
        \n(dpf) - Display previous files\
        \n(dpm) - Display previous month's aggregated data\
        \n(dp#) - Display aggregated data from the previous # files\
        \n(df)  - Display the current file\
        \n(dm)  - Display the current month's aggregated data\
        \n(pd)  - Predefined meals\
        \n(m#)  - Append predefined meal # to the current file\
        \n(q)   - Quit the program\
        \n\nOperation:"
    );

    loop {
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();

        if operation.contains("mf") {
            println!(
                "\n\n(rl#)  - Remove the last # file entry\
                \n(rlq#) - Remove the last # file entry and quit\
                \n(.)    - Repeat the last file entry\
                \n(q)    - Quit the loop\
                \nPress any key to continue"
            );

            return MacroCounter::get_operation(&mut macro_counter, false);
        }
        if operation.contains("dr") {
            let parent_dir = format!("{}/Documents/Health/Macronutritional_Intake", user_dir);
            let mut current_date = Date::current_date();
            let latest_date = Date::decrement_date(&mut current_date, 0);
            let latest_file = Pathing::generate_file_path(latest_date, false);
            let display_data = instantiate_display_data(latest_file.day_path, parent_dir);
            DisplayData::display_file(&display_data, None);
        }

        if operation.contains("dpf") {
            let parent_dir = format!("{}/Documents/Health/Macronutritional_Intake", user_dir);
            DisplayData::display_previous_file(&mut display_data, parent_dir, false, false);
            println!("\nOperation:");
        }

        if operation.contains("dpm") {
            let parent_dir = format!("{}/Documents/Health/Macronutritional_Intake", user_dir);
            DisplayData::display_previous_file(&mut display_data, parent_dir, true, false);
            println!("\n\nOperation:");
        }

        if operation.contains("df") {
            DisplayData::display_file(&display_data, None);
            println!("\nOperation:");
        }

        if operation.contains("dm") {
            DisplayData::display_dir_data(&mut display_data, false);
            println!("\n\nOperation:");
        }

        let re_p = Regex::new(r"dp[0-9]+").unwrap();
        if re_p.is_match(&operation) {
            let rf_count = &operation.trim()[2..];
            let rf_count: i16 = match rf_count.parse() {
                Ok(count) => count,
                Err(err) => {
                    println!("Error: parsing operation: '{}'.", operation);
                    dbg!(err);
                    return main();
                }
            };

            aggregate_recent_files(rf_count);
            let temp_dir = format!(
                "{}/Documents/Health/Macronutritional_Intake/mctr_temp",
                user_dir
            );

            let mut display_recent = instantiate_display_data(String::new(), temp_dir.clone());
            DisplayData::display_dir_data(&mut display_recent, true);
        }

        if operation.contains("pd") {
            println!(
                "\n\n(cf)  - Create a new predefined meal file\
                \n(mf)  - Modify predefined meal files\
                \n(df)  - Display predefined meal files\
                \n(rf)  - Remove the latest predefined file\
                \n(q)   - Quit the loop"
            );
            return predefined::predefined();
        }

        let re_m = Regex::new(r"m[0-9]+").unwrap();
        if re_m.is_match(&operation) {
            // The 1st call to MacroCounter::compile_data() is to save data already in the file.
            MacroCounter::compile_data(&mut macro_counter, true);

            let predefined_file = format!(
                "{}/Documents/Health/Predefined_Meals/{}.txt",
                user_dir,
                &operation.to_string()[0..operation.len() - 1]
            );
            macro_counter.file_path.clear();
            macro_counter.file_path.push_str(&predefined_file);

            // The 2nd call to MacroCounter::compile_data() is to append data read
            // from the predefined_file and aggregate it to the MacroCounter struct fields.
            MacroCounter::compile_data(&mut macro_counter, false);

            macro_counter.file_path.clear();
            macro_counter.file_path.push_str(&pathing.day_path.clone());
            MacroCounter::write_file(&mut macro_counter);

            DisplayData::display_file(&mut display_data, None);

            println!("\nOperation:");
        }

        if operation.contains("q") {
            println!("");
            exit(0);
        }
    }
}
