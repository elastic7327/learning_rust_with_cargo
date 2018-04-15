
use std::collections::HashMap;

#[test]
fn test_hashmap() {

    let mut marks = HashMap::new();

    marks.insert("Rust Programming", 96);
    marks.insert("Web Development", 82);
    marks.insert("Ux Design", 75);
    marks.insert("Professional Computing Studies", 45);

    println!("How many subjects have you studied? {}", marks.len());

    match marks.get("Web Development") {
         Some(mark) => println!("You got {} for Web dev!", mark),
         None => println!("You didn't study Web!")
    }

    marks.remove("Ux Design");

    for (k, v) in marks.iter() {
        println!("{}, {}", k, v);
    }

    println!("{:#?}", marks);
    // check for value
    println!("Did you study C++?m {}", marks.contains_key("C++ Programming"));

}

mod test_rand {

    extern crate rand;
    use self::rand::Rng;

    #[test]
    fn test_random_shit() {
        let random_number = rand::thread_rng().gen_range(1, 11);
        println!("Random number: {}", random_number);

        // Flip a coin
        let random_bool = rand::thread_rng().gen_weighted_bool(2);
        println!("Random bool! {}", random_bool);
    }
}


#[test]
fn test_lets_replace_string() {
   
    {
        let my_string = String::from("Rust is fantastic!");
        println!("After replace: {}", my_string.replace("fantastic", "great"));
    }

    {
        let my_string = String::from("The weather is \nnice\noutside mate!");
        for line in my_string.lines() {
            println!("[{}]", line);
        }
    }

    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed!");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        println!("At index 2: {}", tokens[2]);
    }

    {
        let my_string = String::from("    My name is Daniel    \n\r");

        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
    }


    {
        let my_string = String::from("dcode on YouTube");
        println!("{}", my_string);

        match my_string.chars().nth(4) {
             Some(c) => println!("Charactor at index 4: {}", c),
             None => println!("No Charactor at index 4.")
        }
    }


}
