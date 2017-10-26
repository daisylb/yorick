use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use std::io::{BufReader, Read, Result};
use std::fs::File;
use std::iter::Iterator;

pub trait Skeleton {
    type File: SkeletonFile;
    fn files(&self) -> Box<Iterator<Item=Self::File>>;
}

pub trait SkeletonFile {
    fn path(&self) -> Box<&Path>;
    fn contents(&self) -> Result<Box<Read>>;
}

pub struct FsSkeleton {
    pub root: PathBuf,
}

impl Skeleton for FsSkeleton {
    type File = FsFile;
    fn files(&self) -> Box<Iterator<Item=FsFile>> {
        return Box::new(WalkDir::new(self.root.as_path()).into_iter().map(|entry| FsFile {
            path: Box::new(entry.unwrap().path().to_path_buf())
        }))
    }
}

pub struct FsFile {
    path: Box<PathBuf>,
}

impl SkeletonFile for FsFile {
    fn path(&self) -> Box<&Path> {
        Box::new(self.path.as_path())
    }
    fn contents(&self) -> Result<Box<Read>> {
        let file = File::open(self.path.as_path())?;
        Ok(Box::new(BufReader::new(file)))
    }
}
