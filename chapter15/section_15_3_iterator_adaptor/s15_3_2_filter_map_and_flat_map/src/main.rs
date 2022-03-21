use std::str::FromStr;

fn main() {
    let text = " 1 \nfrond .25 289 \n 3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok()) // Optionを返す関数ならOK
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
}

