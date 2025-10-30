#[macro_export]
macro_rules! file_path {
    ($base:expr $(, $sub:expr)+) => {{
        use ::std::path::PathBuf;
        use ::std::fs;

        let mut full_path = PathBuf::from($base);

        $(
            full_path.push($sub);
        )*

        let parent_dir = full_path.parent().expect("File has no parent directory.");
        fs::create_dir_all(parent_dir).expect("Failed to create parent directory.");

        full_path
    }};
}
