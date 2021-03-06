extern crate page_size;
extern crate wyhash;

use core::hash::Hasher;
use walkdir::WalkDir;
use wyhash::WyHash;

pub mod comp_files;
use crate::comp_files::check_file_eq;

use std::collections::HashMap;

use std::fs::{self, File};
use std::io::prelude::*;

use std::time::SystemTime;

use std::path::PathBuf;

fn remove_verbose(path: &PathBuf) {
    //println!("Removing {:?}", path);
    fs::remove_file(path).expect("Problem with file deleting.");
}

fn print_duplicate(
    remove_orig: bool,
    path_orig: &PathBuf,
    path_dup: &PathBuf,
    dup_result: &mut Vec<Vec<PathBuf>>,
    dup_res_index: &mut HashMap<PathBuf, u64>,
) {
    if !dup_res_index.contains_key(path_orig) {
        dup_res_index.insert(path_orig.to_path_buf(), dup_result.len() as u64);
        dup_result.push(vec![path_orig.clone()]);
    }
    let index: usize = dup_res_index[path_orig] as usize;
    dup_result[index].push(path_dup.clone());
    if remove_orig {
        let last: usize = dup_result[index].len();
        dup_result[index].swap(0, last - 1);
    }
}

fn find_files(root_path: &str) -> (Vec<PathBuf>, HashMap<u64, u64>) {
    let root_path = PathBuf::from(root_path);

    let mut res_files: Vec<PathBuf> = Vec::new();
    let mut files_sizes = HashMap::new();

    for entry in WalkDir::new(&root_path) {
        let path = entry.expect("Get entry path.").path().to_path_buf();
        if !path.is_dir() {
            let file_size = fs::metadata(&path).unwrap().len();
            if files_sizes.contains_key(&file_size) {
                files_sizes.insert(file_size, 2);
            } else {
                files_sizes.insert(file_size, 1);
            }
            res_files.push(path);
        }
    }

    (res_files, files_sizes)
}

fn get_hash(f: &mut File) -> u64 {
    let mut hasher = WyHash::with_seed(3);
    let meta = f.metadata().unwrap();
    let size = meta.len();

    let bytes_per_block = page_size::get();

    let mut buf = vec![0u8; bytes_per_block];

    for _i in 0..(size as usize / bytes_per_block) {
        f.read_exact(&mut buf)
            .expect("Failed to read from the file during hashing.");
        hasher.write(&buf);
    }

    f.read_to_end(&mut buf)
        .expect("Failed to read the end of file during hashing.");
    hasher.write(&buf);

    hasher.finish()
}

fn main() {
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).expect("Unable to parse arguments.");

    let del: bool = arguments.get::<bool>("delete").unwrap_or(false);

    let data_path: String = arguments
        .get::<String>("path")
        .unwrap_or("data".to_string());

    let hash_fun: String = arguments
        .get::<String>("hash_fun")
        .unwrap_or("wyhash".to_string());

    let print_dup: bool = arguments.get::<bool>("print").unwrap_or(false);

    let (res_files, files_sizes) = find_files(&data_path);

    let mut duplicit_helper: HashMap<u64, Vec<&PathBuf>> = HashMap::new();
    let mut files_mod: HashMap<PathBuf, std::time::SystemTime> = HashMap::new();
    let mut dup_result: Vec<Vec<PathBuf>> = Vec::new();
    let mut dup_res_index: HashMap<PathBuf, u64> = HashMap::new();

    for path in res_files.iter() {
        let mut f = File::open(path).expect("Unable to open file during deduplication.");

        let metadata: std::fs::Metadata = f.metadata().expect("Can't read metadata.");
        let modif_time: SystemTime = metadata
            .modified()
            .expect("Can't read mod time from file metadata.");
        let file_size: u64 = metadata.len();

        if files_sizes[&file_size] == 1 {
            continue;
        }

        let checksum = {
            if hash_fun == "dummy" {
                (file_size as u64) % 3
            } else {
                get_hash(&mut f)
            }
        };

        drop(f);

        let mut is_duplicate: bool = false;

        for i in 0..duplicit_helper.entry(checksum).or_default().len() {
            if !check_file_eq(duplicit_helper[&checksum][i], path) {
                continue;
            }
            is_duplicate = true;
            let path_prev = duplicit_helper[&checksum][i];
            let modified_prev = files_mod[path_prev];
            let to_remove;

            if modif_time < modified_prev
                || (modif_time == modified_prev
                    && path.file_name().expect("Can't get file's name.")
                        < path_prev.file_name().expect("Can't get prev file's name."))
            {
                files_mod.insert(path.to_path_buf(), modif_time);
                duplicit_helper.entry(checksum).or_default()[i] = path;
                to_remove = path_prev;
                if print_dup {
                    print_duplicate(true, path_prev, path, &mut dup_result, &mut dup_res_index);
                }
            } else {
                to_remove = path;
                if print_dup {
                    print_duplicate(false, path_prev, path, &mut dup_result, &mut dup_res_index);
                }
            }

            if del {
                remove_verbose(to_remove);
            }
            break;
        }

        if !is_duplicate {
            files_mod.insert(path.to_path_buf(), modif_time);
            duplicit_helper.entry(checksum).or_default().push(path);
        }
    }

    if print_dup {
        for i in dup_result.iter() {
            print!("{}", i[0].to_string_lossy());
            for j in 1..(i.len()) {
                print!(" {}", i[j].to_string_lossy());
            }
            println!();
        }
    }
}
