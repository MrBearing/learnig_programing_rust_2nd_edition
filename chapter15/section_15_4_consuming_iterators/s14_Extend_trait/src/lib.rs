#[cfg(test)]
mod tests {
    #[test]
    fn test_extend_trait() {
        let mut v: Vec<u32> = (0..5).map(|i| 1 << i ).collect();
        assert_eq!(v,[1,2,4,8,16]);
        v.extend([31,57,99,163]);
        assert_eq!(v,[1,2,4,8,16,31,57,99,163])
    }
}
