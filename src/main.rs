extern crate wyhash;
use wyhash::wyhash;

use std::env;

use queue::Queue;
use std::collections::HashMap;

use std::fs::File;
use std::fs::{self, DirEntry};
use std::io::prelude::*;

use std::path::{Path, PathBuf};

fn find_files(root_path: &str) -> Vec<PathBuf> {
    let res_files: Vec<PathBuf> = vec![];

    let root_path = PathBuf::from(root_path);

    let mut q = Queue::new();
    let mut res_files = Vec::new();

    q.queue(root_path);

    while q.len() != 0 {
        let path = q.dequeue().expect("Bad thing.");
        for entry in fs::read_dir(path).expect("Bad thing.") {
            let entry = entry.expect("Bad thing.");
            let new_path = entry.path();
            if new_path.is_dir() {
                q.queue(new_path);
            } else {
                res_files.push(new_path)
            }
        }
    }

    res_files
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let del = {
        if args.len() >= 2 && &args[1] == "delete" {
            true
        } else {
            false
        }
    };

    let res_files = find_files("data");

    let mut duplicit_helper = HashMap::new();
    let mut contents = Vec::new();

    let mut files_mod = HashMap::new();

    for path in res_files.iter() {
        contents.clear();
        let mut f = File::open(path).unwrap();
        f.read_to_end(&mut contents).unwrap();

        let modified = f.metadata().unwrap().modified().unwrap();

        let checksum = wyhash(&contents, 3);

        println!(
            "File {:?} with checksum {} and {:?}",
            path, checksum, modified
        );

        if duplicit_helper.contains_key(&checksum.clone()) {
            println!("Duplicit with {:?}", duplicit_helper[&checksum]);
            let modified2 = files_mod[&checksum];
            let path2 = duplicit_helper[&checksum];

            if modified < modified2 {
                files_mod.insert(checksum, modified);
                duplicit_helper.insert(checksum, path);

                if del {
                    fs::remove_file(path2);
                    println!("Removing {:?}", path2);
                }
            } else {
                if del {
                    fs::remove_file(path);
                    println!("Removing {:?}", path);
                }
            }
            continue;
        }

        files_mod.insert(checksum, modified);
        duplicit_helper.insert(checksum, path);
    }
}
