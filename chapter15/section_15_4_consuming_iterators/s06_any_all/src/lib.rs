#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let id = "Iterator";
        assert!(id.chars().any(char::is_uppercase));
        let id = "ITEARTOR";
        assert!(id.chars().all(char::is_uppercase));
    }
}
