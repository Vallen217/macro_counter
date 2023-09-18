use rmc::{MacroCounter, Pathing};
use rust_macro_counter as rmc;
use std::{self, io, process};

fn main() {
    let dir_path = String::from("/home/vallen/Workspace/rust_macro_counter/foo");
    let file_path = String::from("/home/vallen/Workspace/rust_macro_counter/foo/foo.txt");

    let test_pathing = rmc::Pathing {
        dir_path,
        file_path: file_path.clone(),
    };
    Pathing::check_file_exists(test_pathing);

    let mut macro_counter = MacroCounter {
        file_path: file_path.clone(),
        calorie: Vec::new(),
        fat: Vec::new(),
        carb: Vec::new(),
        protein: Vec::new(),
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

    // TODO: turn into callable function.
    println!("-");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Error: failed to read stdin.");

    if operation.contains("q") {
        process::exit(0);
    }
    if operation.contains("mf") {
        MacroCounter::compile_data(&mut macro_counter);

        println!(
            "\n(rl#)  - Removes the last n file entry lines\
                \n(rlq#) - Removes the last n file entry lines and quit\
                \n(q)    - Quit the loop\
                \nPress any key to continue"
        );

        // MacroCounter::get_operation(&mut macro_counter);
    }
    MacroCounter::compile_data(&mut macro_counter);
    MacroCounter::write_file(&mut macro_counter);
}
