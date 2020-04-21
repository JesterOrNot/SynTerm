use synterm::calculate_whitespace;

#[test]
pub fn test_cw() {
    assert_eq!(
        calculate_whitespace("\x01\x1b[1;33m\x02>>> \x01\x1b[m\x02"),
        4
    )
}
