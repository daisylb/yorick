use std::path::{Path, PathBuf};
use std::io::{Result, Write};
use std::fs::{File, create_dir_all};

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
            create_dir_all(parent.as_os_str())?;
        }
        let file = File::create(file_path.as_os_str())?;
        Ok(Box::new(file))
    }
}
