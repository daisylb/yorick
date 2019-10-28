#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate maplit;

use crate::output::Output;
use crate::skeleton::{Skeleton, SkeletonFile};
use std::path::{Path, PathBuf};

mod config;
mod output;
mod skeleton;

fn main() {
    let skel = skeleton::FsSkeleton {
        root: PathBuf::from(".."),
    };
    for file in skel.files() {
        println!("path : {}", file.path().display());
        let contents: &mut [u8] = &mut [0, 0, 0, 0, 0, 0];
        file.contents().unwrap().read(contents).unwrap();
        println!("start: {:?}", contents);
    }

    let outp = output::FsOutput {
        root: PathBuf::from("./tmp"),
    };
    let mut writer = outp.get_writer(Path::new("foo")).unwrap();
    writer.write(b"Hello!").unwrap();
}
