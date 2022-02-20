struct Selector<T>{
    elements: Vec<T>,
    current: usize,
}


use std::ops::{Deref, DerefMut};

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn show_it(thing: &str) {
    println!("{}", thing);
}

use std::fmt::Display;
fn show_it_generic<T: Display>(thing: T){
    println!("{}", thing);
}

fn main() {
    let mut s = Selector{
        elements: vec!['x','y','z'],
        current: 2
    };

    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());

    *s = 'w';
    assert_eq!(s.elements,['x','y','w']);

    //混乱する例
    let s = Selector{
        elements: vec! ["good","bad","ugly"],
        current: 2
    };
    show_it(&s);
    show_it_generic(&s as &str); // as &strを使うことで明示的に型強制が可能
    show_it_generic(&*s); // コンパイラの言うことに従った場合

}
