mod common;
pub mod macro_counter;

use common::display_data::DisplayData;
use common::pathing::Pathing;
use common::predefined;
use macro_counter::MacroCounter;
use regex::Regex;
use std::io;
use std::process::exit;

fn main() {
    let pathing = Pathing::generate_file_path();
    let mut macro_counter = MacroCounter {
        file_path: pathing.file_path.clone(),
        calorie: Vec::new(),
        fat: Vec::new(),
        carb: Vec::new(),
        protein: Vec::new(),
        totals: Vec::new(),
    };
    let mut display_data = DisplayData {
        file_path: pathing.file_path.clone(),
        dir_path: pathing.dir_path.clone(),
        macro_totals: vec![],
        totals: Vec::new(),
    };

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
        io::stdin()
            .read_line(&mut operation)
            .expect("Error: failed to read stdin.");

        if operation.contains("mf") {
            println!(
                "\n\n(rl#)  - Removes the last n file entry lines\
                \n(rlq#) - Removes the last n file entry lines and quit\
                \n(q)    - Quit the loop\
                \nPress any key to continue"
            );

            // macro_counter fields are re-instantiated every program call,
            // so compile_data() must be called to read data from files,
            // and push it to macro_counter_fields first,
            // inorder to modify file without loosing prexisting data.
            MacroCounter::compile_data(&mut macro_counter, false);
            MacroCounter::get_operation(&mut macro_counter);
        }

        if operation.contains("dpf") {
            let parent_dir = String::from("/home/vallen/Workspace/rust_macro_counter/data_files");
            DisplayData::display_previous_data(&mut display_data, parent_dir, false, false);
        }

        if operation.contains("dpm") {
            let parent_dir = String::from("/home/vallen/Workspace/rust_macro_counter/data_files");
            DisplayData::display_previous_data(&mut display_data, parent_dir, true, false);
        }

        if operation.contains("df") {
            DisplayData::display_data(&display_data, None);
        }

        if operation.contains("dm") {
            DisplayData::display_monthly_data(&mut display_data);
        }

        if operation.contains("pd") {
            println!(
                "\n\n(cf)  - Create new predefined meal\
                \n(mf)  - Modify predefined meal\
                \n(df)  - Display predefined meals\
                \n(q)   - Quit the loop"
            );
            predefined::predefined();
        }

        let re = Regex::new(r"m[0-9]+").unwrap();
        if re.is_match(&operation) {
            // The 1st call to MacroCounter::compile_data() is to save data
            // already in the file.
            MacroCounter::compile_data(&mut macro_counter, true);

            let predefined_file = format!(
                "/home/vallen/Workspace/rust_macro_counter/predefined_meals/{}.txt",
                &operation.to_string()[0..operation.len() - 1]
            );
            macro_counter.file_path.clear();
            macro_counter.file_path.push_str(&predefined_file);
            // The 2nd call to MacroCounter::compile_data() is to append data read
            // from the predefined_file and aggregate it to the MacroCounter struct fields.
            MacroCounter::compile_data(&mut macro_counter, false);

            macro_counter.file_path.clear();
            macro_counter.file_path.push_str(&pathing.file_path.clone());
            MacroCounter::write_file(&mut macro_counter);
        }

        if operation.contains("q") {
            exit(0);
        }
    }
}
