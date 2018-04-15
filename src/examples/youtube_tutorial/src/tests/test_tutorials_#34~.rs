#[cfg(test)]
mod test_regex {

    extern crate regex;
    use self::regex::Regex;

    #[test]
    fn test_regex_test() {
        let re = Regex::new(r"(\w{5})").unwrap();
        let text = "hell";
        println!("Found match? {}", re.is_match(text));

        match re.captures(text) {
            Some(gotcha) => println!("Found match: {}", gotcha.get(0).unwrap().as_str()),
            None => println!("Oops!")
        }
    }
} /* test_regex */


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_option_enum() {
        let name = String::from("Daniel");
        println!("Character at index 8: {}", match name.chars().nth(8) {
            Some(c) => c.to_string(),
            None => "No character at index 8:!".to_string(),
        });
    }
}
