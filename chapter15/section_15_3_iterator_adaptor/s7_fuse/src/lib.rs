struct Flanky(bool);

impl Iterator for Flanky {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("totally last item")
        } else {
            self.0 = true; // Noneを返しているのにself.0の値を更新してる
            // Noneを返すと
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flanky() {
        let mut f = Flanky(true);
        assert_eq!(f.next(), Some("totally last item"));
        assert_eq!(f.next(), None);
        assert_eq!(f.next(), Some("totally last item"));
        assert_eq!(f.next(), None);
        
        let mut fused_f = Flanky(true).fuse();
        assert_eq!(fused_f.next(), Some("totally last item"));
        assert_eq!(fused_f.next(), None);
        assert_eq!(fused_f.next(), None);
        assert_eq!(fused_f.next(), None);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
