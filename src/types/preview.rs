use std::fs::File;
use std::path::Path;

pub struct Preview {
    file_path: Box<Path>,
    file: File,
}
