// 15.4.9のサンプル
use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let sum = stdin.lock()
                .lines()
                .take(5) // take とか入れないとに終わらない。。
                .try_fold(0, |sum, line| ->Result<u64, Box<dyn Error>> {
                    Ok(sum+ u64::from_str(&line?.trim())?)
                })?;
    println!("sum: {}", sum);
    Ok(())
}