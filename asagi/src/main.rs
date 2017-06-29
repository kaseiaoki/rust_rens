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

fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

fn check(d: Vec<String>, t: &'static str) -> bool {
    let mut catch: bool = false;
    let a: &'static str = &*t;
    for d in d.iter() {
        let r = d.contains(&a);
        if (r == true) {
            catch = true;
        }
    }
    return catch;
}

fn lop(p: &std::path::PathBuf, target: &'static str) {
    let p = p.clone();
    thread::spawn(move || { pd(&p, target); });
    thread::sleep_ms(1000);
}

fn tos(p: &std::path::PathBuf, target: &'static str) -> bool {
    let paths = fs::read_dir(p).unwrap();
    let names = paths
        .map(|entry| {
            let entry = entry.unwrap();

            let entry_path = entry.path();

            let file_name = entry_path.file_name().unwrap();

            let file_name_as_str = file_name.to_str().unwrap();

            let file_name_as_string = String::from(file_name_as_str);

            file_name_as_string
        })
        .collect::<Vec<String>>();
    // println!("{}", names[0]);
    let dir: Vec<String> = names;
    if (dir.len() == 0) {
        // sd(".");
        return false;
    }
    let t = String::from(target);
    let ans = check(dir, target);
    return ans;
    // return dir;
}

fn pd(p: &std::path::PathBuf, target: &'static str) {
    if let Ok(entries) = fs::read_dir(&p) {
        for entry in entries {
            if let Ok(entry) = entry {
                let e = entry.path();
                if (tos(&e, target)) {
                    println!("{:?}", metadata(entry.path()).unwrap());
                    process::exit(1);
                }
                let md = metadata(entry.path()).unwrap();
                if (md.is_dir()) {
                    let path_buf = entry.path();
                    lop(&path_buf, target);
                }
            }
        }
    }
}

fn sd(target: &'static str) {
    let t = target.clone();
    if let Ok(entries) = fs::read_dir("") {
        for entry in entries {
            if let Ok(entry) = entry {
                let e = entry.path();
                let md = metadata(entry.path()).unwrap();
                if (md.is_dir()) {
                    let path_buf = entry.path();
                    // println!("in sd");
                    lop(&path_buf, t);
                }
            }
        }
    }
}

fn main() {
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword);
    let vec: Vec<&str> = keyword.split_whitespace().collect();
    let t = String::from(vec[0]);
    let s: &'static str = string_to_static_str(t);
    sd(s);
}
