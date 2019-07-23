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

pub fn pascal(c: u32, r: u32) -> u32{

    fn inner_checked_addition(a: u32, b: u32) -> u32 {
        match a.checked_add(b){
            Some(v) => v,
            None => 0
        }
    }

    match (c,r) {
    (c,r) if c == 0 || c == r => 1,
    (c,r) if c == r - 1 => inner_checked_addition(1 , pascal(c - 1 , r - 1)),
     _ => match inner_checked_addition(pascal(c, r -1) , pascal(c - 1, r - 1)) { x  if x > 0 => x, _ => 0 }
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
         assert_eq!(factorial(1), 1);
         assert_eq!(factorial(0), 1);
    }
    #[test]
    fn test_factorial_large_numbers(){
        assert_eq!(factorial(u32::max_value()-1),0);
    }

    #[test]
    fn test_pascal_small_positive_integers(){
        assert_eq!(pascal(1,3),3);
        assert_eq!(pascal(1,2),2);
        assert_eq!(pascal(0,2),1);
    }
   
    //Failed attempt at preventing overflow for large values in pascal triangle. TODO: Handle
    //overflow gracefully
    #[test]
    fn test_pascal_large_numbers(){
        assert_eq!(pascal(u32::max_value()/4,u32::max_value()/2),0);
    }
}
