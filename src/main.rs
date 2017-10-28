extern crate walkdir;

use std::path::{Path, PathBuf};
use skeleton::{Skeleton, SkeletonFile};
use output::Output;

mod skeleton;
mod output;

fn main() {
    let skel = skeleton::FsSkeleton { root: PathBuf::from("..") };
    for file in skel.files() {
        println!("path : {}", file.path().display());
        let contents: &mut [u8] = &mut [0, 0, 0, 0, 0, 0];
        file.contents().unwrap().read(contents);
        println!("start: {:?}", contents);
    }

    let outp = output::FsOutput { root: PathBuf::from("./tmp") };
    let mut writer = outp.get_writer(Path::new("foo")).unwrap();
    writer.write(b"Hello!");
}
