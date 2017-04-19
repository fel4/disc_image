use std::fmt;
use std::str;
use std::str::FromStr;
use std::string::ToString;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct AString(String);

#[derive(Debug, PartialEq)]
pub struct DString(String);

#[derive(Debug, PartialEq)]
pub struct Error(&'static str);

pub trait ISOString : From<String> {
    fn is_valid_byte(b: u8) -> bool;

    fn parse(i: &[u8]) -> Result<Self,Error> {
        if i.iter().take_while(|&&x| x != b' ').all(|&x| Self::is_valid_byte(x)) {
            match str::from_utf8(i) {
                Ok(s) => Ok(Self::from(s.to_string())),
                Err(_) => Err(Error("Invalid string"))
            }
        } else {
            Err(Error("Invalid string"))
        }
    }

    fn as_str(&self) -> &str;
    fn as_string(&self) -> String;
}

pub trait FromISOString : FromStr {
    fn from_isostring<S: ISOString>(s: &S) -> Result<Self, <Self as FromStr>::Err>;
}

impl<T> FromISOString for T where T: FromStr {
    fn from_isostring<S: ISOString>(s: &S) -> Result<Self, <T as FromStr>::Err> {
        Self::from_str(&s.as_string())
    }
}

impl fmt::Display for AString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for DString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AString {
    fn from(s: String) -> Self {
        AString(s)
    }
}

impl From<String> for DString {
    fn from(s: String) -> Self {
        DString(s)
    }
}


impl ISOString for AString {
    fn is_valid_byte(b: u8) -> bool {
        match b {
            b'A'...b'Z' => true,
            b'0'...b'9' => true,
            b'_' => true,
            _ => false
        }
    }

    fn as_str(&self) -> &str { &self.0 }
    fn as_string(&self) -> String { self.0.clone() }
}

impl ISOString for DString {
    fn is_valid_byte(b: u8) -> bool {
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

    fn as_str(&self) -> &str { &self.0 }
    fn as_string(&self) -> String { self.0.clone() }
}

named_args!(pub astring(len: usize) <AString>, map_res!(take!(len), AString::parse));
named_args!(pub dstring(len: usize) <DString>, map_res!(take!(len), DString::parse));

#[cfg(test)]
mod tests {
    use nom::IResult;
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
    fn astring_valid_string_with_spaces() {
        assert!(AString::parse(b"TEST    ").is_ok());
    }

    #[test]
    fn astring_invalid_string() {
        assert!(AString::parse(b"test").is_err());
    }

    #[test]
    fn astring_invalid_string_with_correct_error() {
        let r = AString::parse(b"foo?");
        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), Error("Invalid string"));
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

    #[test]
    fn dstring_valid_string() {
        assert!(DString::parse(b"TEST").is_ok());
    }

    #[test]
    fn dstring_valid_string_with_spaces() {
        assert!(DString::parse(b"TEST   ").is_ok());
    }

    #[test]
    fn dstring_invalid_string() {
        assert!(DString::parse(b"test").is_err());
    }

    #[test]
    fn dstring_invalid_string_with_correct_error() {
        let r = DString::parse(b"foo?");
        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), Error("Invalid string"));
    }

    #[test]
    fn nom_astring() {
        assert_eq!(astring(&b"FOO"[..], 3), IResult::Done(&b""[..], AString("FOO".to_string())));
    }

    #[test]
    fn nom_dstring() {
        assert_eq!(dstring(&b"FOO?"[..], 4), IResult::Done(&b""[..], DString("FOO?".to_string())));
    }
}