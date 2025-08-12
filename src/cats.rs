pub fn load_arts() -> Vec<String> {
    // Load the whole file at compile time
    let file_contents = include_str!("art.txt");

    // Split on our chosen separator
    file_contents
        .split("---") // separator between art blocks
        .map(|s| s.trim()) // remove extra whitespace/newlines
        .filter(|s| !s.is_empty()) // ignore empty sections
        .map(|s| s.to_string()) // convert to owned String
        .collect()
}