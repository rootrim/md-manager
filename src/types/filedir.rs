use std::path::PathBuf;

pub enum FileSystemEntry {
    File(PathBuf),
    Directory(PathBuf),
}

impl FileSystemEntry {
    pub fn path(&self) -> &PathBuf {
        match self {
            FileSystemEntry::File(path) => path,
            FileSystemEntry::Directory(path) => path,
        }
    }
}
