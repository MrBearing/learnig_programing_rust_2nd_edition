
fn main() {
    println!("try use cargo test");
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;

    #[test]
    fn test_02_max_and_min() {
        assert_eq!([-2,0,1,0,-2,-5].iter().max(),Some(&1));
        assert_eq!([-2,0,1,0,-2,-5].iter().min(),Some(&-5));
    }

    fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
        lhs.partial_cmp(rhs).unwrap() // NAN値が引数に入るとpanicする
    }

    #[test]
    fn test_03_max_by_and_min_by() {
        let numbers = [1.0, 4.0, 2.0];
        assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
        assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));
    }

    // panicするかのテスト
    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_03_max_by_and_min_shoud_panic() {
        let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
        assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0)); 
    }
    
    #[test]
    fn test_04_max_by_key_and_min_by_key() {
        let mut populations = HashMap::new();
        populations.insert("Portland", 583_776 );
        populations.insert("Fossil",449 );
        populations.insert("Greenhorn",2 );
        populations.insert("Boring",7_762 );
        populations.insert("The Dalles", 15_340 );
        assert_eq!(populations.iter().max_by_key(|&(_name, pop)| pop), 
                    Some((&"Portland", & 583_776 )));
        // 当たり前だけどこういう↓書き方はできない(max_byは2引数を受け取って大きい方を返すクロージャが必要)    
        // assert_eq!(populations.iter().max_by(|&(_name, pop)| pop), 
        //             Some((&"Portland", & 583_776 )));

        assert_eq!(populations.iter().min_by_key(|&(_name, pop)| pop),
                    Some((&"Greenhorn", & 2 )));
    }

}