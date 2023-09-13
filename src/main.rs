use rmc::{MacroCounter, Pathing};
use rust_macro_counter as rmc;

fn main() {
    let dir_path = String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo");
    let file_path =
        String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo/good_data.txt");

    let test_pathing = rmc::Pathing {
        dir_path,
        file_path: file_path.clone(),
    };
    Pathing::check_file_exists(test_pathing);

    let mut macro_counter = MacroCounter {
        file_path: file_path.clone(),
        calories: Vec::new(),
        fat: Vec::new(),
        carb: Vec::new(),
        protein: Vec::new(),
        totals: Vec::new(),
    };

    MacroCounter::compile_data(&mut macro_counter);
    println!("{}", macro_counter.calories[2]);
}
