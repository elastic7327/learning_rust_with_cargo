

#[test]
#[ignore]
fn test_strings() {
    let mut s = String::from("Hello my name is Daniel");

    println!("Len {}", s.len());

    println!("String is empty?{}", s.is_empty());

    for token in s.split_whitespace() {
        println!("{}", token);
    }

    println!("Does the string contain 'Daniel'? {}", s.contains("Daniel"));

    s.push_str("Welcome to my Tutorial world");

    println!("{}", s);

}

struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String{
        return format!("My name is {} and I am {}", self.name, self.age);
    }
}

#[test]
#[ignore]
fn test_struct_and_traits() {
    let dan = Person{name: String::from("Daniel"), age: 31};
    println!("{}", dan.to_string());
}

#[test]
#[ignore]
fn test_the_vector() {
    let mut my_vector = vec![1, 2, 3, 4];

    my_vector.push(49);
    my_vector.remove(0);

    for num in my_vector.iter() {
        println!("{}", num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;


    struct Person {
        name: String,
        age: u8,
    }

    trait HasVoiceBox {

        // Speak
        fn speak(&self);

        // Check if can speak
        fn can_speak(&self) -> bool;

    }

    impl HasVoiceBox for Person {

        fn speak(&self) {
            println!("Hello, my name is {}", self.name);
        }

        fn can_speak(&self) -> bool{
            if self.age > 0 {
                return true;
            } else {
                return false;
            }
        }

    }

    #[test]
    #[ignore]
    fn test_read_file_and_print_it() {
        let mut file = File::open("./info.txt").expect("Opps Can't Open it");

        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Oops! Can not read the file!");

        println!("File Contents: \n\n{}", contents)
    }

    #[test]
    #[ignore]
    fn test_write_file() {
        let mut file = File::create("output.txt").expect("Could not crate file!");

        file.write_all(b"Welcome to dcode!").expect("Cannot write to the file, sorry mate.");
    }

    #[test]
    fn test_trait() {
        let p = Person{
            name: String::from("dan"),
            age: 31
        };

        p.speak();
        println!("{}", p.can_speak());

    }

    #[test]
    fn test_patternmat() {

        let number = 15;
        match number {
           1  => println!("It is one"),
           2  => println!("There is two of them!"),
           2...20 => println!("It is greater than one!"),
           _  => println!("It doesn't match!"),
        }
    }

    #[test]
    fn test_patternmat_v2() {
        let name = "Daniel";

        match name {
            "Chris" => println!("Nice name, mate!"),
            "Daniel" => println!("Yeah good on you"),
            _ => println!("Don't know your name"),
        }
    }

    #[test]
    fn test_read_user_input() {
        let mut input = String::new();

        println!("Hey mate! Say something");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Success! You said: {}", input.to_uppercase());
            },
            Err(e) => {
                println!("Oops! something went wrong {}", e);
            }
        }
    }
}

