pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Ok(ok) => ok,
                Err(err) => panic!(err)
            }
        }
    }
}


#[test]
fn find_a_match() {
    let mut writer = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut writer);
    // `b` prefix makes the string a byte string literal `&[u8]` instead of `&str`
    assert_eq!(writer, b"lorem ipsum\n");
}
