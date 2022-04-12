use std::io::prelude::*;
// cargo run すると止まらないので注意
fn main() {
    let stdin = std::io::stdin();
    println!("{}", stdin.lock().lines().count());//止め方わからぬ。。。
}

#[cfg(test)]
mod tests {
    fn triangle(n:u64) -> u64 {
        (1..=n).sum()
    }
    
    #[test]
    fn test_triangle() {
        assert_eq!(triangle(20),210)
    }

    fn factorial(n:u64) -> u64 {
        (1..=n).product()
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(20),2432902008176640000);
    }

    
}