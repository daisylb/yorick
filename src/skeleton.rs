use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::io::{BufReader, Read, Result, Write};
use std::fs::{File, create_dir};
use std::iter::Iterator;
use std::collections::HashMap;
use tempdir::TempDir;

pub trait Skeleton {
    type File: SkeletonFile;
    fn files(&self) -> Box<Iterator<Item = Self::File>>;
}

pub trait SkeletonFile: Sized {
    fn path(&self) -> &Path;
    fn contents(&self) -> Result<Box<Read>>;
}

#[derive(Debug)]
pub struct FsSkeleton {
    pub root: PathBuf,
}

impl Skeleton for FsSkeleton {
    type File = FsFile;
    fn files(&self) -> Box<Iterator<Item = FsFile>> {
        let root = self.root.clone();
        return Box::new(
            WalkDir::new(self.root.as_path())
                .into_iter()
                .map(|entry| entry.unwrap())
                .filter(|entry| entry.file_type().is_file())
                .map(move |entry| {
                    println!("{:?}", root);
                    FsFile {
                        full_path: entry.path().to_path_buf(),
                        path: entry.path().strip_prefix(&root).unwrap().to_path_buf(),
                    }
                }),
        );
    }
}

pub struct FsFile {
    path: PathBuf,
    full_path: PathBuf,
}

impl SkeletonFile for FsFile {
    fn path(&self) -> &Path {
        self.path.as_path()
    }
    fn contents(&self) -> Result<Box<Read>> {
        let file = File::open(self.full_path.as_path())?;
        Ok(Box::new(BufReader::new(file)))
    }
}

#[test]
fn test_fs_skeleton_reader() {
    let dir = TempDir::new("yorick-test-skeleton-reader").unwrap();

    // write content
    let mut file1 = File::create(dir.path().join("foo")).unwrap();
    file1.write(b"FOO").unwrap();
    create_dir(dir.path().join("bar")).unwrap();
    let mut file1 = File::create(dir.path().join("bar/baz")).unwrap();
    file1.write(b"QUX").unwrap();

    // test skeleton
    let skel = FsSkeleton { root: dir.path().to_path_buf() };
    let mut file_map = HashMap::new();
    for file in skel.files() {
        let mut contents = Vec::new();
        file.contents().unwrap().read_to_end(&mut contents).unwrap();
        let path = file.path().to_path_buf();
        file_map.insert(path, contents);
    }
    assert_eq!(file_map.len(), 2);
    println!("files read: {:?}", file_map);
    assert_eq!(file_map.get(Path::new("foo")).unwrap().as_slice(), b"FOO");
    assert_eq!(
        file_map.get(Path::new("bar/baz")).unwrap().as_slice(),
        b"QUX"
    );
}
