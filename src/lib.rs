struct HighlightingPair<'T> {
    item: &'T str,
    color: &'T str,
}

fn split_tokens(current_line: &str) -> Vec<&str> {
    return current_line.split(" ").collect();
}
