use std::fs::File;
use std::path::PathBuf;

pub struct Preview {
    file_path: PathBuf,
    file: File,
}
