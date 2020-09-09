extern crate wyhash;
use wyhash::wyhash;

pub mod comp_files;
use crate::comp_files::add;

use queue::Queue;
use std::collections::HashMap;

use std::fs::{self, File};
use std::io::prelude::*;

use std::path::PathBuf;

fn remove_verbose(path: &PathBuf) {
    //println!("Removing {:?}", path);
    fs::remove_file(path).expect("Problem with file deleting.");
}

fn find_files(root_path: &str) -> (Vec<PathBuf>, std::collections::HashMap<u64, u64>) {
    let root_path = PathBuf::from(root_path);

    let mut q = Queue::new();
    let mut res_files: Vec<PathBuf> = Vec::new();
    let mut files_sizes = HashMap::new();

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
                let file_size = fs::metadata(&new_path).unwrap().len();
                if files_sizes.contains_key(&file_size) {
                    files_sizes.insert(file_size, 2);
                } else {
                    files_sizes.insert(file_size, 1);
                }
                res_files.push(new_path);
            }
        }
    }

    (res_files, files_sizes)
}

fn main() {
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();

    let del: bool = {
        let arg = arguments.get::<String>("action");
        if arg != None && arg.unwrap() == "delete" {
            true
        } else {
            false
        }
    };

    let mut data_path = String::from("data");

    if arguments.get::<String>("path") != None {
        data_path = arguments.get::<String>("path").unwrap().clone();
    }

    let (res_files, files_sizes) = find_files(&data_path);

    let mut duplicit_helper: std::collections::HashMap<u64, &std::path::PathBuf> = HashMap::new();
    let mut contents = Vec::new();

    let mut files_mod = HashMap::new();

    for path in res_files.iter() {
        contents.clear();
        let mut f = File::open(path).unwrap();
        f.read_to_end(&mut contents).unwrap();

        let metadata = f.metadata().unwrap();
        let modif_time = metadata.modified().unwrap();
        let file_size = metadata.len();

        if files_sizes[&file_size] == 1 {
            continue;
        }

        let checksum = wyhash(&contents, 3);

        if duplicit_helper.contains_key(&checksum.clone()) {
            let modified_prev = files_mod[&checksum];
            let path_prev = duplicit_helper[&checksum];
            let to_remove;

            if modif_time < modified_prev
                || (modif_time == modified_prev
                    && path.file_name().unwrap() < path_prev.file_name().unwrap())
            {
                files_mod.insert(checksum, modif_time);
                duplicit_helper.insert(checksum, path);
                to_remove = path_prev;
                println!("{} {}", path.to_string_lossy(), path_prev.to_string_lossy());
            } else {
                to_remove = path;
                println!("{} {}", path_prev.to_string_lossy(), path.to_string_lossy());
            }

            if del {
                remove_verbose(to_remove);
            }
            continue;
        }

        files_mod.insert(checksum, modif_time);
        duplicit_helper.insert(checksum, path);
    }
}
