extern crate wyhash;
use wyhash::wyhash;

use std::env;

use queue::Queue;
use std::collections::HashMap;

use std::fs::{self, File};
use std::io::prelude::*;

use std::path::PathBuf;

fn remove_verbose(path: &PathBuf) {
    println!("Removing {:?}", path);
    fs::remove_file(path).expect("Problem with file deleting.");
}

fn find_files(root_path: &str) -> Vec<PathBuf> {
    let root_path = PathBuf::from(root_path);

    let mut q = Queue::new();
    let mut res_files: Vec<PathBuf> = Vec::new();

    q.queue(root_path)
        .expect("Error during queue push of root.");

    while q.len() != 0 {
        let path = q.dequeue().expect("Error during queue pop.");
        for entry in fs::read_dir(path).expect("Error during listing files.") {
            let entry = entry.expect("Bad thing.");
            let new_path = entry.path();
            if new_path.is_dir() {
                q.queue(new_path).expect("Error during queue push.");
            } else {
                res_files.push(new_path)
            }
        }
    }

    res_files
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let del: bool = {
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

        /*let checksum = md5::compute(&contents);
        println!(
            "File {:?} with checksum {:x} and {:?}",
            path, checksum, modified
        );*/

        if duplicit_helper.contains_key(&checksum.clone()) {
            println!("Duplicit with {:?}", duplicit_helper[&checksum]);
            let modified_prev = files_mod[&checksum];
            let path_prev = duplicit_helper[&checksum];
            let to_remove;

            if modified < modified_prev {
                files_mod.insert(checksum, modified);
                duplicit_helper.insert(checksum, path);
                to_remove = path_prev;
            } else {
                to_remove = path;
            }

            if del {
                remove_verbose(to_remove);
            }
            continue;
        }

        files_mod.insert(checksum, modified);
        duplicit_helper.insert(checksum, path);
    }
}
