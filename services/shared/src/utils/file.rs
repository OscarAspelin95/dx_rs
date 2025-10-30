use std::path::PathBuf;

/// This is the stupidest function I've ever seen...
/// There are many fancy .and_then().map() stuff we can do
/// but this function is so stupid I'll let it be for now.
pub fn file_name(file: PathBuf) -> String {
    file.file_name()
        .unwrap()
        .to_os_string()
        .to_str()
        .unwrap()
        .to_string()
}
