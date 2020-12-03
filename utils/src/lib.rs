pub fn split_lines(contents: &str) -> impl Iterator<Item = &str> {
        contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
}
