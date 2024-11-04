use std::path::PathBuf;

pub fn default_path(path: Option<PathBuf>, default: PathBuf, ext: &str) -> PathBuf {
    path.unwrap_or(
        PathBuf::from(".").join(
            default
                .with_extension(ext)
                .file_name()
                .expect("Failed to get file name"),
        ),
    )
}
