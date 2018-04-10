
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
