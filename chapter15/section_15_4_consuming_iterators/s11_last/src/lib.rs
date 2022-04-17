#[cfg(test)]
mod tests {
    #[test]
    fn test_last() {
        let squares = (0..10).map(|i| i*i);
        assert_eq!(squares.last(),Some(81));
    }
}
