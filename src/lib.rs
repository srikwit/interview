pub fn factorial(n: u32) -> u32{

    fn inner_factorial(n: u32, agg: u32) -> u32{
        match n.checked_mul(agg) {
            None => 0,
            _ => match n { n if n >= 2 => inner_factorial(n-1,n*agg), _ => agg}
        }
    }

    match n {
        n if n ==1 || n==0 => 1,
        n if n > 1 => inner_factorial(n, 1) ,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factorial_small_positive_integers() {
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(5), 120);
    }
    #[test]
    fn test_factorial_zero_and_one(){
        assert_eq!(factorial(1), 1);                                                                                                                                                                    assert_eq!(factorial(0), 1);
    }
    #[test]
    fn test_large_numbers(){
        assert_eq!(factorial(u32::max_value()-1),0);
    }
}
