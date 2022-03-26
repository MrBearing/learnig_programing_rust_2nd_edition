use std::str::FromStr;
use std::collections::HashMap;
fn main() {

    println!("**filter_map function sample**");
    let text = " 1 \nfrond .25 289 \n 3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok()) // Optionを返す関数ならなんでも使える
    {
        println!("{:4.2 }", number.sqrt());
    }
    // 同じことをmapとFilterで実装した場合、ちょっと冗長
    let text = " 1 \nfrond .25 289 \n 3.1415 estuary\n";
    for number in text.split_whitespace()
        .map(|w| f64::from_str(w))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
    {
        println!("{:4.2 }", number.sqrt());
    }
    println!("**flat_map fucntion sample**");
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
    let countries = ["Japan", "Brazil", "Kenya"];
    
    for city in countries.iter().flat_map(|c| &major_cities[c]){
        println!("{}",city);
    }

    //　すべての都市を出すならこう書く
    println!("**alls cities **");
    for city in major_cities.iter().flat_map(|entry| entry.1 ){
       println!("{}",city); 
    }
    println!("**alls cities 2**");
    for city in major_cities.values().flatten(){
        println!("{}",city); 
    }
 

}

