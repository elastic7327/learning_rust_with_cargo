
#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_smoke_test() {

        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn test_option_some_none() {
        let result = divide(2.0, 3.0);

        match result {
            Some(x) => println!("Result: {}", x),
            None => println!("Cannot divide by 0")
        }
    }

    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

        #[test]
    fn test_is_none() {
        let x: Option<u32> = Some(2);
        assert_eq!(x.is_none(), false);

        let x: Option<u32> = None;
        assert_eq!(x.is_none(), true);
                                            
    }

        #[test]
    fn test_as_ref() {
        let num_as_str: Option<String> = Some("10".to_string());
        println!("print num as str -> {:?}", num_as_str);
        let num_as_int: Option<usize> = num_as_str.as_ref().map(|n| n.len());
        println!("print num as int -> {:?}", num_as_int);
                                            
    }

    #[test]
    fn test_expect() {
        let x = Some("value");
        assert_eq!(x.expect("the world is ending"), "value");


        let x: Option<&str> = None;

        x.expect("the world is ending");
        
    }

    #[test]
    fn test_unwarp() {
        let x = Some("in the air");
        assert_eq!(x.unwrap(), "in the air");

        let x: Option<&str> = None;
        assert_eq!(x.unwrap(), "in the air");
    }

    #[test]
    fn test_unwrap_or_else() {
        assert_eq!(Some("car").unwrap_or("bike"), "car");

        assert_eq!(None.unwrap_or("bike"), "bike");

    }

    #[test]
    fn test_unwrap_and_closure() {
        let k = 10;
        assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
        assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
    }

    #[test]
    fn test_map_and_some() {
        let maybe_some_string = Some(String::from("hello, world!"));

        let maybe_some_len = maybe_some_string.map(|s| s.len());

        assert_eq!(maybe_some_len, Some(13));
    }

    #[test]
    fn test_as_mut() {
        let mut x = Some(2);
        match x.as_mut() {
            Some(v) => *v = 42,
            None => {},
        }

        assert_eq!(x, Some(42));
    }
}

#[cfg(test)]
mod op_sm {

    #[test]
    fn test_option_and_some_more() {
        
        let x: Option<i32> = Some(2);
        assert_eq!(x.is_some(), true);

        println!("{:?}", x);

        let x: Option<u32> = None;
        assert_eq!(x.is_some(), false);

        println!("{:?}", x);
    }
}
