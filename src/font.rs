use std::ffi::OsStr;

/// An acceptable font name.
///
/// This exists to ensure the name is:
/// 1. UTF-8
/// 2. Does not contain file-breaking characters, such as:
///    - Null (U+0000)
///    - Quotes (`"` or `'`)
///
/// This is restrictive with the intention to become lenient as more research is
/// done.
#[derive(Clone, Copy, Debug)]
pub struct Font<'a>(&'a str);

impl<'a> Font<'a> {
    /// Parses the string as a valid font name.
    pub fn from_os_str(name: &'a OsStr) -> Option<Self> {
        let name = name.to_str()?;
        if Self::is_valid_str(name) {
            Some(Self(name))
        } else {
            None
        }
    }

    fn is_valid_str(s: &str) -> bool {
        s.chars().all(Self::is_valid_char)
    }

    fn is_valid_char(ch: char) -> bool {
        match ch {
            '\n' | '\r' | '\t' | '"' | '\'' => false,
            ' ' => true,
            _ => ch.is_ascii_graphic() || ch.is_alphanumeric(),
        }
    }

    /// Returns the name as a UTF-8 string.
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}
