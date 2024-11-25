use std::ffi::{c_int, CStr};

use super::raw;

#[derive(Debug, thiserror::Error)]
#[error("{message}")] // libgit2의 메시지를 출력한다.
pub struct Error {
    code: i32,
    message: String,
    class: i32,
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error {
            code: -1,
            message: value,
            class: 0,
        }
    }
}

// `NulError`는 `CString::new`가 문자열에 0바이트가 들어 있을 때 반환하는 타입이다.
impl From<std::ffi::NulError> for Error {
    fn from(value: std::ffi::NulError) -> Self {
        Error {
            code: -1,
            message: value.to_string(),
            class: 0,
        }
    }
}

pub fn check(code: c_int) -> Result<c_int, Error> {
    if code >= 0 {
        return Ok(code);
    }

    unsafe {
        let error = raw::funcs::giterr_last();

        let message = CStr::from_ptr((*error).message)
            .to_string_lossy()
            .into_owned();

        Err(Error {
            #[allow(clippy::unnecessary_cast)]
            code: code as i32,
            message,
            class: (*error).klass as i32,
        })
    }
}
