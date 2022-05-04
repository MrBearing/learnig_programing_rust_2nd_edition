use std::error::Error;
use std::io::{self,BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;


fn _grep_str(target: &str) -> io::Result<()> {
    let stdin = io::stdin();
    for line_result in stdin.lock().lines(){
        let line = line_result?;
        if line.contains(target){
            println!("{}", line);
        }
    }
    
    Ok(())
}

fn grep<R>(target: &str, reader: R) -> io::Result<()> 
    where R: BufRead
{
    // for line_result in reader.lines() {
    //     let line = line_result?;
    //     if line.contains(target){
    //         println!(" {}", line);
    //     }
    // }
    
    // イテレータで書き直し
    reader.lines()
        .filter_map(|line_result| line_result.ok())
        .filter(|line| line.contains(target))
        .for_each(|line| println!(" {}", line));

    Ok(())
}

fn grep_main() -> Result<(),Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE...")?
    };
    let files : Vec<PathBuf> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    }else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }
    Ok(())
}


fn main() {
    // let stdin = io::stdin();
    // grep("test",stdin.lock());
    let result = grep_main();
    if let Err(err) = result {
        eprintln!("{}",err);
        std::process::exit(1);
    }
}
