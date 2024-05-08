mod common;
pub mod macro_counter;

use common::display_data::DisplayData;
use common::pathing::{self, Date, Pathing};
use common::predefined;
use common::utils::*;
use macro_counter::MacroCounter;
use regex::Regex;
use std::{io, process::exit};

fn main() {
    let pathing = Pathing::generate_file_path(Date::current_date());
    let mut macro_counter = instantiate_macro_counter(pathing.day_path.clone());
    let mut display_data =
        instantiate_display_data(pathing.day_path.clone(), pathing.month_path.clone());
    let user_dir = pathing::user_path();

    println!(
        "\n\n(mf)  - Modify file\
        \n(dpf) - Display previous files\
        \n(dpm) - Display previous monthly data\
        \n(df)  - Display file\
        \n(dm)  - Display monthly data\
        \n(pd)  - Predefined meals\
        \n(m#)  - Append predefined meal m#\
        \n(q)   - Quit the program\
        \n\nOperation:"
    );

    loop {
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();

        if operation.contains("mf") {
            println!(
                "\n\n(rl#)  - Removes the last n file entry lines\
                \n(rlq#) - Removes the last n file entry lines and quit\
                \n(.)    - Repeat the last data entry line\
                \n(q)    - Quit the loop\
                \nPress any key to continue"
            );

            return MacroCounter::get_operation(&mut macro_counter, false);
        }

        // TODO: Allow for an operation of -1 to view the most recent non-current file.
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
            DisplayData::display_monthly_data(&mut display_data);
            println!("\n\nOperation:");
        }

        // TODO: Write a function to create a temporary dir and copy the last 'n' number of files into the dir.
        // Then call display_monthly_data on that dir.

        if operation.contains("pd") {
            println!(
                "\n\n(cf)  - Create new predefined meal\
                \n(mf)  - Modify predefined meal\
                \n(df)  - Display predefined meals\
                \n(q)   - Quit the loop"
            );
            return predefined::predefined();
        }

        let re = Regex::new(r"m[0-9]+").unwrap();
        if re.is_match(&operation) {
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
