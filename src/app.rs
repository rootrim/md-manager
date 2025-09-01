use crate::types::{FileExplorer, Preview, Search};

pub enum AppState {
    Search,
    File,
    Preview,
}

pub struct App {
    pub state: AppState,
    pub explorer_state: FileExplorer,
    pub preview_state: Option<Preview>,
    pub search_state: Search,
}
