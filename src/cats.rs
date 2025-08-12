pub fn load_arts() -> Vec<String> {
    let file_contents = include_str!("art.txt"); // Load the art file

    file_contents
        .split("---") // Split ASCII arts with "---""
        .map(|s| s.trim()) // Trim whitespace from the ASCII art
        .filter(|s| !s.is_empty()) // Filter empty slices
        .map(|s| s.to_string()) // Convert the slices
        .collect() // Collet the arts in a vector
}