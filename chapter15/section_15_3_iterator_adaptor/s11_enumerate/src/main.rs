fn main() {
    let vector = vec![0 , 2,  4,  6, 8];
    let _v = vector.into_iter()
        .enumerate()
        .map(|(n, v)| println!("#{}, value:{}",n,v))
        .collect::<Vec<_>>();
}
