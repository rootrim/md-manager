use crate::types::filedir::FileSystemEntry;
use std::path::PathBuf;

pub struct FileExplorer {
    current_path: PathBuf,
    entries: Vec<FileSystemEntry>,
    index: Option<u8>,
}
