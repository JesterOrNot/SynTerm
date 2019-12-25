extern crate synterm;
#[cfg(test)]
mod tests {
    #[test]
    fn test_split_tokens() {
        assert_eq!(synterm::split_tokens("Hello World"), vec!["Hello", "World"]);
    }
}
