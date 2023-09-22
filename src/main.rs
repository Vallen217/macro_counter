mod common;
pub mod macro_counter;

use common::display_data::DisplayData;
use common::pathing::Pathing;
use macro_counter::MacroCounter;
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
        "\n(mf)  - Modify file\
        \n(dpf) - Display previous files\
        \n(dpm) - Display previous monthly data\
        \n(df)  - Display file\
        \n(dm)  - Display monthly data\
        \n(pd)  - Predefined meals\
        \n(m#)  - Append predefined meal m#\
        \n(q)   - Quit the program"
    );

    loop {
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Error: failed to read stdin.");

        if operation.contains("mf") {
            println!(
                "\n(rl#)  - Removes the last n file entry lines\
                \n(rlq#) - Removes the last n file entry lines and quit\
                \n(q)    - Quit the loop\
                \nPress any key to continue"
            );
            MacroCounter::compile_data(&mut macro_counter, true);
            // FIX: recompile data before writing file
            MacroCounter::get_operation(&mut macro_counter);
            return main();
        }

        if operation.contains("dpf") {
            let parent_dir = String::from("/home/vallen/Workspace/rust_macro_counter/data_files");
            DisplayData::display_previous_data(&mut display_data, parent_dir, false);
        }
        if operation.contains("dpm") {
            let parent_dir = String::from("/home/vallen/Workspace/rust_macro_counter/data_files");
            DisplayData::display_previous_data(&mut display_data, parent_dir, true);
        }
        if operation.contains("df") {
            DisplayData::display_data(&display_data, None);
        }

        if operation.contains("dm") {
            let parsed_data: (Vec<String>, Vec<String>) =
                DisplayData::compile_monthly_data(&mut display_data);

            Pathing::create_file(&pathing, true);
            DisplayData::write_monthly_data(&mut display_data, parsed_data);
        }

        // TODO: predefined_meals
        // if operation.contains("pd") {}
        // let re = Regex::new(r"m[0-9]+").unwrap();
        // if re.is_match(operation) {}

        if operation.contains("q") {
            exit(0);
        }
    }
}
