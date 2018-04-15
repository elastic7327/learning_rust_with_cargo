

#[test]
#[ignore]
fn test_iteration() {
    assert_eq!(1, 1);

    let a = vec!["a", "b", "c", "d"];
    
    for a in a.iter() {
        println!("{}", a);
    }

    for (idx, a) in a.iter().enumerate() {
        println!("{}: {}", idx, a);
    }
}

#[derive(Debug)]
struct Ractangle {
    width: u32,
    height: u32
}
impl Ractangle {
    fn print_description(&self) {
        println!("Ractangle :{} :{}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

#[derive(Debug)]
struct MyColor(u8, u8, u8);

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[test]
#[ignore]
fn test_enum_and_direction() {
    let player_direction:Direction = Direction::Up;

    match player_direction {
         Direction::Up => println!("We are Heading Up!"),
         Direction::Down => println!("We are Heading Down!"),
         Direction::Left => println!("We are Heading Left!"),
         Direction::Right => println!("We are Heading Right!"),
    }

    let player_direction:Direction = Direction::Down;

    match player_direction {
         Direction::Up => println!("We are Heading Up!"),
         Direction::Down => println!("We are Heading Down!"),
         Direction::Left => println!("We are Heading Left!"),
         Direction::Right => println!("We are Heading Right!"),
    }
}

#[test]
#[ignore]
fn tst_special_array() {
    let numbers = [2; 5];
    for i in numbers.iter() {
        println!("{:?}", i);
    }
}

#[test]
#[ignore]
fn test_tuple_struct() {
    let mut red = MyColor(255, 0, 0);
    println!("red 0 is {}", red.0);
    println!("{:#?}", red);
}

#[test]
fn test_impl_keyword() {
    let mut my_ract = Ractangle{width:10, height:5};
    my_ract.print_description();
    println!("is square? {}", my_ract.is_square());

    my_ract.height = 10;
    my_ract.print_description();
    println!("is square? {}", my_ract.is_square());

}

#[test]
fn test_tuple_what_the_hack() {
    let tuple = (20, "Rust", 3.4, false, (1, 2, 3, 4));
    // println!("{:#?}", tuple);
    println!("{:#?}", tuple.1);
    println!("{:#?}", tuple.4);
}
