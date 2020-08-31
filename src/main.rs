use crc32fast::Hasher;

use queue::Queue;
use std::collections::HashMap;

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
    let res_files = find_files("../data");
    
    let mut duplicit_helper = HashMap::new();
    
    for path in res_files.iter() {
        let mut contents = String::new();
        File::open(path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

        let mut hasher = Hasher::new();
        hasher.update(contents.as_bytes());
        let checksum = hasher.finalize();
        
        println!("File {:?} with checksum {}", path, checksum);
        
        if duplicit_helper.contains_key(&checksum.clone()) {
            println!("Duplicit with {:?}", duplicit_helper[&checksum]);
        }
        
        duplicit_helper.insert(checksum, path);
    }
}
