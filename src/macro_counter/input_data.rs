use super::*;
use std::io;

impl MacroCounter {
    pub fn collect_data(&mut self, predefined: bool) {
        self.push_data(String::from("Fat: "), MacroType::Fat);
        self.push_data(String::from("Carb: "), MacroType::Carb);
        self.push_data(String::from("Protein: "), MacroType::Protein);
        let last_i = self.calorie.len();
        self.calorie.push(
            (self.fat[last_i].clone() * 9.0)
                + (self.carb[last_i].clone() * 4.0)
                + (self.protein[last_i].clone() * 4.0),
        );
        // save newly gathered data to the file,
        self.write_file();

        println!("\n(Press enter to continue)\nOperation:");
        self.get_operation(predefined)
    }

    fn push_data(&mut self, macro_stdin: String, macro_type: MacroType) {
        println!("\n{}", macro_stdin);
        let mut macro_data = String::new();
        io::stdin().read_line(&mut macro_data).unwrap();

        let float_data: f32 = match macro_data.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };

        match macro_type {
            MacroType::Fat => self.fat.push(float_data),
            MacroType::Carb => self.carb.push(float_data),
            MacroType::Protein => self.protein.push(float_data),
            MacroType::Calorie => (),
        };
    }
}
