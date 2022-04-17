#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn test_find() {
        let mut populations = HashMap::new();
        populations.insert("Portland", 583_776 );
        populations.insert("Fossil",449 );
        populations.insert("Greenhorn",2 );
        populations.insert("Boring",7_762 );
        populations.insert("The Dalles", 15_340 );
        assert_eq!(populations.iter().find(|&(_name , &pop)| pop > 1_000_000), None);
        assert_eq!(populations.iter().find(|&(_name , &pop)| pop > 500_000), Some((&"Portland", &583_776)));
    }
}
