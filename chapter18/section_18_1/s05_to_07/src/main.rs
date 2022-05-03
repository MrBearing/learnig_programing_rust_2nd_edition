use std::io;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {

    let file = File::create("tmp.txt")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Hello world!");

   



    Ok(())
}
