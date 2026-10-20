//! Temporary Android stand-in for `rfd`: same names, no dialogs yet.
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileHandle(PathBuf);
impl FileHandle {
    pub fn path(&self) -> &Path {
        &self.0
    }
}

macro_rules! builder {
    ($t:ident) => {
        #[derive(Debug, Default, Clone)]
        pub struct $t;
        impl $t {
            pub fn new() -> Self { Self }
            pub fn parent<W: ?Sized>(self, _w: &W) -> Self { self }
            pub fn set_parent<W: ?Sized>(self, _w: &W) -> Self { self }
            pub fn set_title(self, _t: impl Into<String>) -> Self { self }
            pub fn add_filter(self, _n: impl Into<String>, _e: &[impl ToString]) -> Self { self }
            pub fn set_directory<P: AsRef<Path>>(self, _p: P) -> Self { self }
            pub fn set_file_name(self, _n: impl Into<String>) -> Self { self }
        }
    };
}
builder!(FileDialog);
builder!(AsyncFileDialog);

impl FileDialog {
    pub fn pick_file(self) -> Option<PathBuf> { None }
    pub fn pick_files(self) -> Option<Vec<PathBuf>> { None }
    pub fn save_file(self) -> Option<PathBuf> { None }
    pub fn pick_folder(self) -> Option<PathBuf> { None }
}
impl AsyncFileDialog {
    pub async fn pick_file(self) -> Option<FileHandle> { None }
    pub async fn pick_files(self) -> Option<Vec<FileHandle>> { None }
    pub async fn save_file(self) -> Option<FileHandle> { None }
    pub async fn pick_folder(self) -> Option<FileHandle> { None }
}
