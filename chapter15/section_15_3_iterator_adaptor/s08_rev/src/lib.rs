#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_double_ended_iterator() {
        let bee_parts = ["head", "thorax","abdomen"];
        let mut iter = bee_parts.iter();

        assert_eq!(iter.next(), Some(&"head"));
        assert_eq!(iter.next_back(), Some(&"abdomen"));
        assert_eq!(iter.next(), Some(&"thorax"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_rev_adoptor() {
        let meals = ["breakfast","launch","dinner"];
        let mut iter = meals.iter().rev();
        assert_eq!(iter.next(), Some(&"dinner"));
        assert_eq!(iter.next(), Some(&"launch"));
        assert_eq!(iter.next(), Some(&"breakfast"));
        assert_eq!(iter.next(), None);
    }
}
