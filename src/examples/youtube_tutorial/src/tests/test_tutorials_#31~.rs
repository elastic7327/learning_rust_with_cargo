
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
    //
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
        //
        let random_bool = rand::thread_rng().gen_weighted_bool(2);
        println!("Random bool! {}", random_bool);
    }
}
