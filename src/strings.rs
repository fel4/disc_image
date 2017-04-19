use std::str;

#[derive(Debug, PartialEq)]
pub struct AString(String);

#[derive(Debug, PartialEq)]
pub struct DString(String);

#[derive(Debug, PartialEq)]
pub struct Error(&'static str);

impl AString {
    pub fn is_valid_byte(b: u8) -> bool {
        match b {
            b'A'...b'Z' => true,
            b'0'...b'9' => true,
            b'_' => true,
            _ => false
        }
    }

    pub fn parse(i: &[u8]) -> Result<AString,Error> {
        if i.iter().all(|&x| AString::is_valid_byte(x)) {
            match str::from_utf8(i) {
                Ok(s) => Ok(AString(s.to_string())),
                Err(_) => Err(Error("Invalid AString"))
            }
        } else {
            Err(Error("Invalid AString"))
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
    fn astring_valid_string() {
        assert!(AString::parse(b"TEST").is_ok());
    }

    #[test]
    fn astring_invalid_string() {
        assert!(AString::parse(b"test").is_err());
    }

    #[test]
    fn astring_invalid_string_with_correct_error() {
        let r = AString::parse(b"foo?");
        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), Error("Invalid AString"));
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