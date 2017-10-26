extern crate walkdir;

use std::path::PathBuf;
use skeleton::{Skeleton, SkeletonFile};

mod skeleton;

fn main() {
    let skel = skeleton::FsSkeleton {root: PathBuf::from("..")};
    for file in skel.files() {
        println!("path : {}", file.path().display());
        let contents: &mut [u8] = &mut [0, 0, 0, 0, 0, 0];
        file.contents().unwrap().read(contents);
        println!("start: {:?}", contents);
    }
}
