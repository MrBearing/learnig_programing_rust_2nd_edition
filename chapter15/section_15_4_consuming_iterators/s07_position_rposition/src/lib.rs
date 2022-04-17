#[cfg(test)]
mod tests {
    #[test]
    fn test_position() {
        let text = "Xerxes";
        assert_eq!(text.chars().position(|c| c =='e'),Some(1));
        assert_eq!(text.chars().position(|c| c =='z'),None);
    }
    #[test]
    fn test_rposition(){
        let bytes = b"Xerxes";
        assert_eq!(bytes.iter().rposition(|&c| c ==b'e'),Some(4));
        assert_eq!(bytes.iter().rposition(|&c| c ==b'X'),Some(0));
    }
}
