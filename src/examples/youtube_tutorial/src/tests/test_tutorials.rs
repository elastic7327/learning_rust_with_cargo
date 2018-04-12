

#[test]
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


enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[test]
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

