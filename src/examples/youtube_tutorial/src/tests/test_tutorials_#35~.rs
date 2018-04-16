
mod daniel {

    fn chicken() {
        println!("Chicken!");
    }

    pub fn print_message() {
        chicken();
        println!("How's it going!");
    }

    pub mod water {
        pub fn print_message() {
            println!("I'm water!");
        }
    } /* pub mod water */
} /* daniel */


#[test]
fn test_print_messge() {
    daniel::water::print_message();
    
}

mod test_with_enum_methods {

    enum Day {
        Mon, Tue, Wed,
        Turs, Fri, Sat, Sun
    }
} /* test_with_enum_methods */
