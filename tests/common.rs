#[cfg(test)]
mod tests {
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
        let file_path = String::from("/home/vallen/Workspace/rust_macro_counter/tests/foo/foo.txt");
        let test_pathing = rmc::Pathing {
            dir_path,
            file_path,
        };

        rmc::Pathing::check_file_exists(test_pathing);
    }
}
