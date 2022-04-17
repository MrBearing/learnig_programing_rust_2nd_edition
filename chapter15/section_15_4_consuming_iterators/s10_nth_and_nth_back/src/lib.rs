#[cfg(test)]
mod tests {
    #[test]
    fn test_nth() {
        let mut squares = (0..10).map(|i| i*i);
        assert_eq!(squares.nth(4), Some(16)); // 0 + 4 = 4
        assert_eq!(squares.nth(0), Some(25)); // 4 + 1 = 5
        assert_eq!(squares.nth(6), None); // 5 + 6 + 1 = 12( overflow)
        let mut squares = (0..10).map(|i| i*i);
        assert_eq!(squares.nth_back(2), Some(49)); // 10 - 2 - 1 = 7
        assert_eq!(squares.nth_back(0), Some(36)); // 7 - 1 = 6
        assert_eq!(squares.nth_back(6), None); // 6 - 1 -6 = -1( overflow)
    }
}
