use rmc::pathing::Pathing;
use rust_macro_counter as rmc;

fn main() {
    let dir_path = String::from("/home/vallen/Workspace/rust_macro_counter/foo");
    let file_path = String::from("/home/vallen/Workspace/rust_macro_counter/foo/foo.txt");

    let test_pathing = Pathing {
        dir_path,
        file_path: file_path.clone(),
    };
    Pathing::check_file_exists(test_pathing);

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
    rmc::run(file_path.clone());
}
