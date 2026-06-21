use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RawFile {
    pub path: PathBuf,
    pub content: String,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct ProcessedFile {
    pub path: PathBuf,
    pub content: String,
    pub token_count: usize,
}

#[derive(Debug, Clone)]
pub struct SkippedFileInfo {
    pub path: PathBuf,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct SuspiciousFileResult {
    pub path: PathBuf,
    pub line: usize,
    pub message: String,
    pub rule_id: String,
}

#[derive(Debug, Clone)]
pub struct FileSearchResult {
    pub file_paths: Vec<PathBuf>,
    pub empty_dir_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct FileCollectResult {
    pub raw_files: Vec<RawFile>,
    pub skipped_files: Vec<SkippedFileInfo>,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub suspicious: Vec<SuspiciousFileResult>,
    pub safe_paths: Vec<PathBuf>,
}
