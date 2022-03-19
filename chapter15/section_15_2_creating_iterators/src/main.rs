

fn main() {
    println!("for run: cargo test ");
}

#[cfg(test)]
mod tests {
    
    // #[test]
    

    #[test]
    fn test_iter_example2(){
        use std::ffi::OsStr;
        use std::path::Path;

        let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");//Fedora入れる気だ。。。
        let mut iterator = path.iter();
        assert_eq!(iterator.next(), Some(OsStr::new("C:")));
        assert_eq!(iterator.next(), Some(OsStr::new("Users")));
        assert_eq!(iterator.next(), Some(OsStr::new("JimB")));
        assert_eq!(iterator.next(), Some(OsStr::new("Downloads")));
    }

    #[test]
    fn test_iter_example1(){
        let v = vec![4, 20, 12, 8, 6];
        let mut iterator = v.iter();

        assert_eq!(iterator.next(), Some(&4)); // &4 と比較する必要があるiter.nextが返すのはあくまで参照
        assert_eq!(iterator.next(), Some(&20));
        assert_eq!(iterator.next(), Some(&12));
        assert_eq!(iterator.next(), Some(&8));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
    }

}
