#[cfg(test)]
mod tests {
    #[test]
    fn test_partition() {
        let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];
        let (living, nonliving): (Vec<&str>, Vec<&str>) 
            = things.iter().partition(|name| name.as_bytes()[0] & 1 != 0);
        assert_eq!(living , vec!["mushroom", "giraffe", "grapefruit"]);
        assert_eq!(nonliving, vec!["doorknob", "noodle"]);
    }
}
