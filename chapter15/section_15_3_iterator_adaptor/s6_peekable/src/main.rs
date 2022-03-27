

fn main() {
}

#[cfg(test)]
mod test {
    use std::iter::Peekable;

    fn parse_number<I>(tokens: &mut Peekable<I>) -> u32 
        where I: Iterator<Item=char>
    {
        let mut n = 0;
        loop {
            match tokens.peek(){
                Some(r) if r.is_digit(10) => { 
                    n = n * 10 + r.to_digit(10).unwrap();
                    // 1回前のループ時にインクリした値を10倍して桁上げ
                }
                _ => return n //数字以外がきたらnを返す
            }
            tokens.next();
        }
    }
    #[test]
    fn test_parse_number(){
        let mut chars = "226153980,1766319049".chars().peekable();
        assert_eq!(parse_number(&mut chars), 226153980 );
        assert_eq!(chars.next(), Some(',')); 
        assert_eq!(parse_number(&mut chars), 1766319049 );
        assert_eq!(chars.next(), None);
    }
}