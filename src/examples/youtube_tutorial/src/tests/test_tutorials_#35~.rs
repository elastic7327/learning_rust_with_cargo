
#[allow(unused_variable)]
#[allow(dead_code)]


struct Rectangle {
    width: u8,
    height: u8,
}

impl Rectangle {

    fn is_square(&self) -> bool {
        self.width  == self.height
    }
}


mod daniel {
    #[allow(unused_variable)]
    #[allow(dead_code)]

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
    #[allow(unused_variable)]
    #[allow(dead_code)]

    use super::*;

    enum Day {
        Mon, Tue, Wed,
        Turs, Fri, Sat, Sun
    }

    impl Day {
        fn is_weekday(&self) -> bool {
            match self {
                &Day::Sat | &Day::Sun => return false,
                _ => return true
            }
        }
    }

    #[test]
    fn test_with_enum_meth() {

        let l = Day::Sat;

        println!("is weekday ? {}", l.is_weekday());

        daniel::water::print_message();

        let r = super::Rectangle{width: 32, height: 32};

        println!("Is Square?! {}", r.is_square());

    }

} /* test_with_enum_methods */
