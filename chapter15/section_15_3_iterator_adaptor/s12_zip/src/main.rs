use std::iter::repeat;

fn main() {
    let _vector = vec![0 , 2,  4,  6, 8];
    let _v:Vec<_> = (0..).zip("ABCDEFGHIJ".chars())
        .map(|(n, v)| println!("#{}, value:{}",n,v))
        .collect();
    let endings = ["once","twice","chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going")
        .zip(endings)
        .collect();
    assert_eq!(rhyme,vec![("going","once"),
                          ("going","twice"),
                          ("going","chicken soup with rice") ]);
}

