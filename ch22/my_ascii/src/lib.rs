#[derive(Debug, PartialEq, Eq)]
pub struct Ascii(Vec<u8>);

impl Ascii {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
        if bytes.iter().any(|&byte| !byte.is_ascii()) {
            return Err(NotAsciiError(bytes));
        }
        Ok(Ascii(bytes))
    }

    ///# 주의사항
    /// `bytes`에 아스키 문자만 있어야 한다.
    pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
        Ascii(bytes)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotAsciiError(pub Vec<u8>);

impl From<Ascii> for String {
    fn from(value: Ascii) -> Self {
        unsafe {
            String::from_utf8_unchecked(value.0)
        }
    }
}

#[test]
fn test_ascii_to_string() {
    let bytes= b"ASCII and ye shall receive".to_vec();
    let ascii = Ascii::from_bytes(bytes).unwrap();
    let string = String::from(ascii);

    assert_eq!(string, "ASCII and ye shall receive");
}