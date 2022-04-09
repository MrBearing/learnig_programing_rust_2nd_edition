use std::iter::{once,repeat};

fn main() {
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    // -> ["", "", fizz,"",  "",   fizz,"", "", fizz ..... ]
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    // -> ["", "", "",  "", "buzz","",  "", "", "", "buzz",.....]
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes)
        .map(|tuple|
            match tuple { 
                (i,("","")) => i.to_string(),
                (_, (fizz, buzz)) => format!("{}{}",fizz,buzz)
            });
    for line in fizz_buzz{
        println!("{}",line);
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cycle(){
        let dirs = ["North", "East", "South", "West"];
        let mut spin = dirs.iter().cycle();
        assert_eq!(spin.next(), Some(&"North"));
        assert_eq!(spin.next(), Some(&"East"));
        assert_eq!(spin.next(), Some(&"South"));
        assert_eq!(spin.next(), Some(&"West"));
        assert_eq!(spin.next(), Some(&"North"));
        assert_eq!(spin.next(), Some(&"East"));
    }

    #[test]
    fn test_cycle_fuse(){
        let dirs = ["North", "East", "South", "West"];
        let mut spin = dirs.iter().fuse().cycle();
        assert_eq!(spin.next(), Some(&"North"));
        assert_eq!(spin.next(), Some(&"East"));
        assert_eq!(spin.next(), Some(&"South"));
        assert_eq!(spin.next(), Some(&"West"));
        assert_eq!(spin.next(), Some(&"North"));
        assert_eq!(spin.next(), Some(&"East"));
    }
}