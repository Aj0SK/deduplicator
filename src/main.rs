use crc32fast::Hasher;
use queue::Queue;

use std::fs::{self, DirEntry};

use std::fs::File;
use std::io::prelude::*;

use std::path::{Path, PathBuf};

fn find_files(root_path : &str) -> Vec<PathBuf> {
    let res_files : Vec<PathBuf> = vec![];

    let root_path = PathBuf::from(root_path);
    
    let mut q = Queue::new();
    let mut res_files = Vec::new();
    
    q.queue(root_path);
    
    while q.len() != 0
    {
        let path = q.dequeue().expect("Bad thing.");
        for entry in fs::read_dir(path).expect("Bad thing.")
        {
            let entry = entry.expect("Bad thing.");
            let new_path = entry.path();
            if new_path.is_dir() {
                q.queue(new_path);
            }
            else {
                res_files.push(new_path)
            }
        }
    }
    
    res_files
}

fn main() {
    /*let mut hasher = Hasher::new();
    hasher.update(b"foo bar baz");
    let checksum = hasher.finalize();
    
    println!("Hello, world! {}", checksum);
    
    let path = PathBuf::from("../data");
    let x = count_files_walking(&path);
    println!("Count is {}", x);*/
    
    let res_files = find_files("../data");
    
    for x in res_files.iter() {
        println!("In {:?} is:", x);
        
        let mut contents = String::new();
        File::open(x)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        println!("{}", contents);
        
        let mut hasher = Hasher::new();
        hasher.update(contents.as_bytes());
        let checksum = hasher.finalize();
        
        println!("and checksum is {}", checksum);
    }
}
