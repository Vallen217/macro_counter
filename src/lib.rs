pub mod macro_counter;
pub mod pathing;

use macro_counter::MacroCounter;
use std::io;
use std::process::exit;

pub fn run(file_path: String) {
    let mut macro_counter = MacroCounter {
        file_path,
        calorie: Vec::new(),
        fat: Vec::new(),
        carb: Vec::new(),
        protein: Vec::new(),
        totals: Vec::new(),
    };
    let mut operation = String::new();
    println!("-");
    io::stdin()
        .read_line(&mut operation)
        .expect("Error: failed to read stdin.");

    if operation.contains("q") {
        exit(0);
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

pub fn pad_word(word: &str) -> String {
    let num_of_spaces = 12 - word.len();
    let padding = " ".repeat(num_of_spaces);
    return padding;
}
