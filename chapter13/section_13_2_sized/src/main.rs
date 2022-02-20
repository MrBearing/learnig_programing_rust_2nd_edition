use std::fmt::Display;
struct RcBox<T: ?Sized> {
    _ref_cont: usize,
    value: T,
}
fn display(boxed: &RcBox<dyn Display>){
    println!("For your enjoyment : {}",&boxed.value);
}

fn main() {
    let boxed_lunch: RcBox<String>  = RcBox{
        _ref_cont: 1,
        value: "lunch".to_string()
    };
    
    let boxed_displayable: &RcBox<dyn Display> =&boxed_lunch; // これしなくてもいいの？
   
    display(&boxed_lunch);
    display(&boxed_displayable);
    /*
    実行結果は同じ
            Running `target/debug/section_13_2_sized`
        For your enjoyment : lunch
        For your enjoyment : lunch
    */
}
