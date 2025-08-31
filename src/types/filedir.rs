use std::path::Path;

pub enum FileSystemEntry {
    File(Box<Path>),
    Directory(Box<Path>),
}
