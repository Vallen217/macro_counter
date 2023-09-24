use super::*;
use std::io;

impl MacroCounter {
    pub fn collect_data(&mut self) {
        self.push_data(String::from("Calorie: "), MacroType::Calorie);
        self.push_data(String::from("Carb: "), MacroType::Fat);
        self.push_data(String::from("Fat: "), MacroType::Carb);
        self.push_data(String::from("Protein: "), MacroType::Protein);
        // save newly gathered data to the file,
        self.write_file();

        println!("\n(Press enter to continue)\nOperation:");
        return self.get_operation();
    }

    fn push_data(&mut self, macro_stdin: String, macro_type: MacroType) {
        println!("{}", macro_stdin);
        let mut macro_data = String::new();
        io::stdin().read_line(&mut macro_data).unwrap();

        let float_data: f32 = match macro_data.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };

        match macro_type {
            MacroType::Calorie => self.calorie.push(float_data),
            MacroType::Fat => self.fat.push(float_data),
            MacroType::Carb => self.carb.push(float_data),
            MacroType::Protein => self.protein.push(float_data),
        };
    }
}
