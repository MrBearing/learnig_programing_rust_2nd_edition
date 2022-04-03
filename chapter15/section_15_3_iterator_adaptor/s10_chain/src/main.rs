fn main() {
    let v : Vec<i32> = (1..4).chain([20,30,40]).collect();
    assert_eq!(v,[1,2,3,20,30,40]);
    let v : Vec<i32> = (1..4).chain([20,30,40]).rev().collect();
    assert_eq!(v,[40,30,20,3,2,1]);
}
