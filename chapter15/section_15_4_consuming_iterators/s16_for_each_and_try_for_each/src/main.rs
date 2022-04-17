fn main() {
    ["doves","hens","birds"].iter()
        .zip(["turtle","french","calling"])
        .zip(2..5)
        .rev()
        .map(|((item ,kind),quantity)|{
            format!("{} {} {}",quantity,kind, item)
        })
        .for_each(|gift|{
            println!("you have received: {}",gift); 
        });
    // syntax sugar
    println!("syntax sugar");

    for gift in ["doves", "hens", "birds"].iter()
        .zip(["turtle","french","calling"])
        .zip(2..5)
        .rev()
        .map(|((item ,kind),quantity)|{
            format!("{} {} {}",quantity,kind, item)
        })
    {
        println!("you have received: {}",gift); 
    }
        

}
        