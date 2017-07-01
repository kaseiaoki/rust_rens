use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::string::String;
use std::io::prelude::*;
use std::os::windows;
use std::path::Path;
use std::fs::metadata;
use std::env;
use std::vec::Vec;
use std::thread;
use std::process;
use std::mem;


fn main() {

    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword);
    let vec: Vec<&str> = keyword.split_whitespace().collect();
    let t = String::from(vec[0]);
}
