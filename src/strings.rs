pub struct AString(String);
pub struct DString(String);

impl AString {
    pub fn is_valid_byte(b: u8) -> bool {
        match b {
            b'A'...b'Z' => true,
            b'0'...b'9' => true,
            b'_' => true,
            _ => false
        }
    }
}



impl DString {
    pub fn is_valid_byte(b: u8) -> bool {
        if AString::is_valid_byte(b) {
            true
        } else {
            match b {
                b'!' | b'"' | b'%' | b'&' | b'\'' | b'(' | b')' |
                b'*' | b'+' | b',' | b'-' | b'.' | b'/' | b':' |
                b';' | b'<' | b'=' | b'>' | b'?' => true,
                _ => false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn astring_valid_byte() {
        assert!(AString::is_valid_byte(b'A'));
    }

    #[test]
    #[should_panic]
    fn astring_invalid_byte() {
        assert!(AString::is_valid_byte(b'!'));
    }

    #[test]
    fn dstring_valid_byte() {
        assert!(DString::is_valid_byte(b'!'));
    }

    #[test]
    #[should_panic]
    fn dstring_invalid_byte() {
        assert!(DString::is_valid_byte(b'a'));
    }
}