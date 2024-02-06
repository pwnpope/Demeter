use walkdir::WalkDir;


pub fn dirsearch(directory: &str, extension: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(directory).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == extension) {
            files.push(entry_path.display().to_string());
        }
    }
    files
}
