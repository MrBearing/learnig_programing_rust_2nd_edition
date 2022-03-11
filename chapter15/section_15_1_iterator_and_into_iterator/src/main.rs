fn main() {
    println!("There's: ");
    let v = vec!["antimony","arsenic","aluminum","selenium"];

    for element in &v {
        println!("{}", element);
    }
    
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}",element);
    }

}
