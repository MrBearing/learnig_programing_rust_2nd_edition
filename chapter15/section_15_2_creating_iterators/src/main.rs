use rand::random;
use std::iter::from_fn;

fn main() {

    let v = vec![4, 20, 12, 8, 6];
    let iterator = v.iter();
    println!("test example dump func");
    dump(iterator);
    println!("for 15.2.3");

    let length : Vec<f64> =
        from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(1000)
        .collect();
    dump(length.iter());


}

use std::fmt::Debug;

fn dump<T,U>(t: T)
    where T: IntoIterator<Item=U>,
            U: Debug
{
    for u in t{
        println!("{:?}", u);
    }
}




#[cfg(test)]
mod tests {

    #[test]
    fn test_15_2_4(){
        println!("for 15.2.4");
        let mut outer = "Earth".to_string();
        let inner = String::from_iter(outer.drain(1..4));
        assert_eq!(outer, "Eh"); // outerはここでドロップしてるはず
        assert_eq!(inner, "art");// drainで所有権が移ってるので、エラーにはならない
    }

    fn fibonacci() -> impl Iterator<Item=usize>{
        let mut state = (0,1);
        std::iter::from_fn(move || {
            state = (state.1,state.0 + state.1);
            Some(state.0)
        })
    }

    #[test]
    fn test_fibonacci(){
        let fib = fibonacci().take( 8 ).collect::<Vec<_>>();
        assert_eq!(fib,
            vec![ 1 , 1 , 2 , 3 , 5 , 8 , 13 , 21 ]);
        dbg!(fib);
    }


    #[test]
    fn test_into_iterator(){
        use std::collections::BTreeSet;
        let mut favorites = BTreeSet::new();
        favorites.insert("Lucy in the Sky With Diamonds".to_string());
        favorites.insert("Liebestra...".to_string());
        dbg!("favorites");
        let mut it = favorites.into_iter();
        assert_eq!(it.next(), Some("Liebestra...".to_string()));
        assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
        assert_eq!(it.next(), None);
        
    }

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
