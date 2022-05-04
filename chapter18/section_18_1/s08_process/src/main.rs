use std::io::{self, Write};
use std::process::{Command,Stdio};

fn main() -> io::Result<()> {
    println!("hello ");
    let mut child = Command::new("grep")
        .arg("-e")
        .arg("a.*e.*i.*o.*u")
        .stdin(Stdio::piped())
        .spawn()?;
    let mut to_child = child.stdin.take().unwrap();
    let my_words = ["this","is","Some","test"].iter();
    for word in my_words {
        writeln!(to_child,"{}", word)?;
    }
    drop(to_child);
    child.wait()?;
    Ok(())
}
