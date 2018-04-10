
#[cfg(test)]
mod tests {
    #[test]
    fn test_smoke_test() {
        assert_eq!(1, 1);
    }

    #[test]
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
}
