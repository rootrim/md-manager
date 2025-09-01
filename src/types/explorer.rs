use crate::types::filedir::FileSystemEntry;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

pub struct FileExplorer {
    current_path: PathBuf,
    entries: Vec<FileSystemEntry>,
    index: Option<u8>,
}

impl FileExplorer {
    pub fn create(path: PathBuf) -> Result<Self> {
        Ok(Self {
            entries: create_entries(&path)?,
            current_path: path,
            index: None,
        })
    }
}

fn create_entries(path: &PathBuf) -> Result<Vec<FileSystemEntry>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_dir() {
                    Some(FileSystemEntry::Directory(path))
                } else if path.is_file() {
                    Some(FileSystemEntry::File(path))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<FileSystemEntry>>())
}
