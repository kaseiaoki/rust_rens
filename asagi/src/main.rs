use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::windows;
use std::path::Path;



fn main() {
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword);

    let vec: Vec<&str> = keyword.split_whitespace().collect();
    let p = Path::new(vec[0]);

    if let Ok(entries) = fs::read_dir("") {
        for entry in entries {
            if let Ok(entry) = entry {
                let e = entry.path();
                if e == p {
                    println!("gotya");
                }
                println!("{:?}", entry.path());
            }
        }
    }
}
