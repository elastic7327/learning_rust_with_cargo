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
    #[test]
    fn test_option_enum() {
        let name = String::from("Daniel");
        println!("Character at index 1: {}", match name.chars().nth(1) {
            Some(c) => c.to_string(),
            None => "No character at index 8:!".to_string(),
        });
    }
}

#[cfg(test)]
mod tests_for_occupation {

    fn get_occupation(name: &str) -> Option<&str> {
        match name {
            "Daniel" => Some("Somftware Developer"),
            "Michael" => Some("Dentist"),
            _ => None,
        }
    }

    #[test]
    fn test_something_related_with_occupation() {
        println!("Occupation is {}", match get_occupation("Freddy") {
             Some(o) => o, 
             None => "No occupation!!",
        });
    }
}

#[cfg(test)]
mod tests_for_reqwest {
    extern crate reqwest;
    // use self::reqwest;
    #[test]
    #[ignore]
    fn test_reqwest() {

        match reqwest::get("https://www.naver.com") {
            Ok(mut response) => {
                if response.status()  == reqwest::StatusCode::Ok {
                    match response.text() {
                         Ok(text) => println!("Response Text: {}", text),
                         Err(_) => println!("Could not read response msg")
                    }

                } else {
                    println!("Response was not 200 Ok");
                }
            }

            Err(_) => println!("Could not make the request!")
            }
        }
}

#[cfg(test)]
mod test_requests_more_easy_way {

    extern crate reqwest;
    // use self::reqwest;
    #[test]
    fn test_reqwest() {
        let response_text = reqwest::get("https://www.naver.com").expect("Could't make request").text().expect("Could not read response text!");
        println!("{}", response_text);
    }
}


