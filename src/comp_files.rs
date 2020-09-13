extern crate page_size;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn check_file_eq(path1: &PathBuf, path2: &PathBuf) -> bool {
    let bytes_per_block = page_size::get();

    let mut f1 = File::open(path1).expect("Can't open File 1 for eq check.");
    let mut f2 = File::open(path2).expect("Can't open File 2 for eq check.");

    let meta1 = f1.metadata().expect("Can't read File 1 metadata.");
    let meta2 = f2.metadata().expect("Can't read File 2 metadata.");

    if meta1.len() != meta2.len() {
        return false;
    }

    let size = meta1.len();

    let mut buf1 = vec![0u8; bytes_per_block];
    let mut buf2 = vec![0u8; bytes_per_block];

    for _i in 0..(size as usize / bytes_per_block) {
        f1.read_exact(&mut buf1)
            .expect("Failed to read from the file during eq check.");
        f2.read_exact(&mut buf2)
            .expect("Failed to read from the file during eq check.");

        if buf1 != buf2 {
            return false;
        }
    }

    f1.read_to_end(&mut buf1)
        .expect("Failed to read the end of file during eq check.");
    f2.read_to_end(&mut buf2)
        .expect("Failed to read the end of file during eq check.");

    return buf1 == buf2;
}
