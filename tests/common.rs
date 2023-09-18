#[cfg(test)]
mod tests {
    use rust_macro_counter::MacroCounter;
    use rust_macro_counter::Pathing;

    fn instantiate_macro_counter(file_path: Option<String>) -> MacroCounter {
        let good_data_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo/good_data.txt");
        let file_path = match file_path {
            Some(file_path) => file_path,
            None => good_data_path,
        };

        let test_data = MacroCounter {
            file_path,
            calorie: Vec::new(),
            fat: Vec::new(),
            carb: Vec::new(),
            protein: Vec::new(),
            totals: Vec::new(),
        };

        return test_data;
    }

    #[test]
    fn check_padding() {
        let word: &str = "test";
        let padding: String = rust_macro_counter::pad_word(word);
        let padded_word = String::from("        ");
        assert_eq!(padded_word, padding);
    }

    #[test]
    fn file_exists() {
        let dir_path = String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo");
        let file_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo/good_data.txt");
        let test_pathing = Pathing {
            dir_path,
            file_path,
        };

        Pathing::check_file_exists(test_pathing);
    }

    #[test]
    fn compile_data_eq_fields() {
        let mut test_data = instantiate_macro_counter(None);

        MacroCounter::compile_data(&mut test_data);

        let test_cal: Vec<f32> = Vec::from([180.0, 180.0, 280.0, 280.0]);
        let test_fat: Vec<f32> = Vec::from([7.7, 0.1, 11.0, 2.0]);
        let test_carb: Vec<f32> = Vec::from([21.0, 21.0, 55.0, 55.0]);
        let test_protein: Vec<f32> = Vec::from([42.0, 22.8, 10.1, 62.1]);

        assert_eq!(&test_data.calorie, &test_cal);
        assert_ne!(&test_data.fat, &test_fat);
        assert_eq!(&test_data.carb, &test_carb);
        assert_ne!(&test_data.protein, &test_protein);
    }

    #[test]
    #[should_panic]
    fn compile_data_ne_fields() {
        let mut test_data = instantiate_macro_counter(None);

        MacroCounter::compile_data(&mut test_data);

        let test_cal: Vec<f32> = Vec::from([180.0, 180.0, 280.0, 280.0]);
        let test_fat: Vec<f32> = Vec::from([7.7, 0.1, 11.0, 2.0]);
        let test_carb: Vec<f32> = Vec::from([21.0, 21.0, 55.0, 55.0]);
        let test_protein: Vec<f32> = Vec::from([42.0, 22.8, 10.1, 62.1]);

        assert_ne!(&test_data.calorie, &test_cal);
        assert_eq!(&test_data.fat, &test_fat);
        assert_ne!(&test_data.carb, &test_carb);
        assert_eq!(&test_data.protein, &test_protein);
    }

    #[test]
    #[should_panic]
    fn compile_bad_data() {
        let file_path =
            String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo/bad_data.txt");
        let mut test_data = instantiate_macro_counter(Some(file_path));
        MacroCounter::compile_data(&mut test_data);
    }

    #[test]
    fn remove_data() {
        let mut test_data = instantiate_macro_counter(None);
        let operation = String::from("rlq2");

        // let initial_cal: Vec<f32> = Vec::from([180.0, 180.0, 280.0, 280.0]);
        // let initial_fat: Vec<f32> = Vec::from([6.0, 6.0, 2.0, 2.0]);
        // let initial_carb: Vec<f32> = Vec::from([21.0, 21.0, 55.0, 55.0]);
        // let initial_protein: Vec<f32> = Vec::from([12.0, 12.8, 10.0, 10.0]);

        let expected_cal: Vec<f32> = Vec::from([180.0, 180.0]);
        let expected_fat: Vec<f32> = Vec::from([6.0, 6.0]);

        MacroCounter::compile_data(&mut test_data);
        MacroCounter::remove_data(&mut test_data, operation);

        let resultant_cal: Vec<f32> = test_data.calorie.clone();
        let resultant_fat: Vec<f32> = test_data.fat.clone();

        assert_eq!(expected_cal, resultant_cal);
        assert_eq!(expected_fat, resultant_fat);
    }

    #[test]
    fn data_totals() {
        let mut test_data = instantiate_macro_counter(None);
        let expected_values: Vec<f32> = Vec::from([920.0, 16.0, 152.0, 44.0]);

        MacroCounter::compile_data(&mut test_data);
        MacroCounter::write_file(&mut test_data);

        assert_eq!(test_data.totals, expected_values);
    }

    #[test]
    fn write_file() {
        let mut test_data = instantiate_macro_counter(None);

        MacroCounter::write_file(&mut test_data);
        MacroCounter::compile_data(&mut test_data);
    }
}
