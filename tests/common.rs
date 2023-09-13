#[cfg(test)]
mod tests {
    use rmc::MacroCounter;
    use rust_macro_counter as rmc;

    #[test]
    fn check_padding() {
        let word: &str = "test";
        let padding: String = rmc::pad_word(word);
        let padded_word = String::from("        ");
        assert_eq!(padded_word, padding);
    }

    #[test]
    fn file_exists() {
        let dir_path = String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo");
        let file_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo/good_data.txt");
        let test_pathing = rmc::Pathing {
            dir_path,
            file_path,
        };

        rmc::Pathing::check_file_exists(test_pathing);
    }

    #[test]
    fn compile_data_eq_fields() {
        let file_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo/good_data.txt");
        let mut test_data = rmc::MacroCounter {
            file_path,
            calories: Vec::new(),
            fat: Vec::new(),
            carb: Vec::new(),
            protein: Vec::new(),
            totals: Vec::new(),
        };
        MacroCounter::compile_data(&mut test_data);

        let test_cal: Vec<f32> = Vec::from([180.0, 180.0, 280.0, 280.0]);
        let test_fat: Vec<f32> = Vec::from([7.7, 0.1, 11.0, 2.0]);
        let test_carb: Vec<f32> = Vec::from([21.0, 21.0, 55.0, 55.0]);
        let test_protein: Vec<f32> = Vec::from([42.0, 22.8, 10.1, 62.1]);

        assert_eq!(&test_data.calories, &test_cal);
        assert_ne!(&test_data.fat, &test_fat);
        assert_eq!(&test_data.carb, &test_carb);
        assert_ne!(&test_data.protein, &test_protein);
    }

    #[test]
    #[should_panic]
    fn compile_data_ne_fields() {
        let file_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo/good_data.txt");
        let mut test_data = rmc::MacroCounter {
            file_path,
            calories: Vec::new(),
            fat: Vec::new(),
            carb: Vec::new(),
            protein: Vec::new(),
            totals: Vec::new(),
        };
        MacroCounter::compile_data(&mut test_data);

        let test_cal: Vec<f32> = Vec::from([180.0, 180.0, 280.0, 280.0]);
        let test_fat: Vec<f32> = Vec::from([7.7, 0.1, 11.0, 2.0]);
        let test_carb: Vec<f32> = Vec::from([21.0, 21.0, 55.0, 55.0]);
        let test_protein: Vec<f32> = Vec::from([42.0, 22.8, 10.1, 62.1]);

        assert_ne!(&test_data.calories, &test_cal);
        assert_eq!(&test_data.fat, &test_fat);
        assert_ne!(&test_data.carb, &test_carb);
        assert_eq!(&test_data.protein, &test_protein);
    }

    #[test]
    #[should_panic]
    fn compile_bad_data() {
        let file_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo/bad_data.txt");
        let mut test_data = rmc::MacroCounter {
            file_path,
            calories: Vec::new(),
            fat: Vec::new(),
            carb: Vec::new(),
            protein: Vec::new(),
            totals: Vec::new(),
        };
        MacroCounter::compile_data(&mut test_data);
    }
}
