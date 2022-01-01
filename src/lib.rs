use core::str;
use std::panic::panic_any;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Ok(ok) => ok,
                Err(err) => panic_any(err),
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

#[test]
fn unfound_pattern() {
    let mut buffer = Vec::new();
    find_matches("foobar", "\n", &mut buffer);
    let lhs = str::from_utf8(&buffer).unwrap();
    assert_eq!(lhs, "");
}

#[test]
fn empty_string_pattern() {
    let mut buffer = Vec::new();
    find_matches("foobar", "", &mut buffer);
    let lhs = str::from_utf8(&buffer).unwrap();
    assert_eq!(lhs, "foobar\n");
}
