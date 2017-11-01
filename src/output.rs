use std::path::{Path, PathBuf};
use std::io::{Result, Write, Read};
use std::fs::{File, create_dir_all};
use tempdir::TempDir;

pub trait Output {
    fn get_writer(&self, &Path) -> Result<Box<Write>>;
}

pub struct FsOutput {
    pub root: PathBuf,
}

impl Output for FsOutput {
    fn get_writer(&self, path: &Path) -> Result<Box<Write>> {
        assert!(path.is_relative());
        let file_path = self.root.join(&path);
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }
        let file = File::create(file_path.as_path())?;
        Ok(Box::new(file))
    }
}

#[test]
fn test_fs_output(){
    let dir = TempDir::new("yorick-test-output").unwrap();
    let out = FsOutput{ root: dir.path().to_path_buf() };

    // writing to the root of the directory
    let mut writer = out.get_writer(Path::new("foo")).unwrap();
    writer.write(b"FOO").unwrap();
    let mut file = File::open(dir.path().join("foo")).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    assert_eq!(contents, b"FOO");

    // writing to a subdirectory
    let mut writer = out.get_writer(Path::new("bar/baz")).unwrap();
    writer.write(b"QUX").unwrap();
    let mut file = File::open(dir.path().join("bar/baz")).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    assert_eq!(contents, b"QUX");
}
