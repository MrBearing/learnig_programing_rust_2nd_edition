struct Appellation {
    name : String,
    nicknames : Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA) {}",self.nicknames.join(","));
        }
        println!("");
    }
}

fn main() {
    let mut _a = Appellation {
        name: "Zeuse".to_string(),
        nicknames: vec!["cloud collector".to_string(),
                        "cking of the gods".to_string()]
    };

    println!("before assingment");
    _a = Appellation {name: "Hera".to_string(), nicknames : vec![]};
    println!("at end of block")
}


/*
 実行結果
 
```
     Running `target/debug/section_13_1_drop`
before assingment
Dropping Zeuse(AKA) cloud collector,cking of the gods
at end of block
Dropping Hera
```
*/

