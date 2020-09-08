extern crate page_size;

use std::fs::{self, File};
use std::io::prelude::*;

pub fn add(path1: &String, path2: &String) -> bool {
    let bytes_per_block = page_size::get();

    let mut f1 = File::open(path1).unwrap();
    let mut f2 = File::open(path2).unwrap();

    let meta1 = f1.metadata().unwrap();
    let meta2 = f2.metadata().unwrap();

    if meta1.len() != meta2.len() {
        return false;
    }

    let size = meta1.len();

    let mut buf1 = vec![0u8; bytes_per_block];
    let mut buf2 = vec![0u8; bytes_per_block];

    for i in 0..(size as usize / bytes_per_block) {
        f1.read_exact(&mut buf1);
        f2.read_exact(&mut buf2);

        if buf1 != buf2 {
            return false;
        }
    }

    f1.read_exact(&mut buf1);
    f2.read_exact(&mut buf2);

    return buf1 == buf2;
}
