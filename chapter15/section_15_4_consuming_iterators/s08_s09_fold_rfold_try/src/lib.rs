// 15.4.8のサンプル
#[cfg(test)]
mod tests {
    #[test]
    fn test_fold() {
        let a = [5, 6, 7, 8, 9, 10];
        assert_eq!(a.iter().fold(0, |n, _ | n + 1), 6); // count
        assert_eq!(a.iter().fold(0, |n, i | n + i), 45); // sum
        assert_eq!(a.iter().fold(1, |n, i | n * i), 151200); // product
    }
    #[test]
    fn test_fold_string() {
        let a = ["Pack", "my", "box", "with","five", "dozen", "liquor", "jugs"];
        let pangram = a.iter().fold(String::new(), |s,w| s+w+" ");
        assert_eq!(pangram,"Pack my box with five dozen liquor jugs ");
        let weird_pangram = a.iter().rfold(String::new(), |s,w| s+w+" ");
        assert_eq!(weird_pangram,"jugs liquor dozen five with box my Pack ");
    }

    
}
