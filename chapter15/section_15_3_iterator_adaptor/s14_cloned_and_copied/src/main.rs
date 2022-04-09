fn main() {
    let a = ['1', '2', '3', '∞'];
    let mut a_iter = a.iter();
    assert_eq!(a_iter.next(), Some(&'1'));
    assert_eq!(a_iter.next(), Some(&'2'));
    let mut cloned_a_iter = a.iter().cloned();
    assert_eq!(cloned_a_iter.next(), Some('1'));
    assert_eq!(cloned_a_iter.next(), Some('2'));
}
