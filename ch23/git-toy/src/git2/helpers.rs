use std::{
    ffi::{c_char, c_int, CStr, CString},
    path::Path,
};

use crate::git2::{
    errors::Error,
    raw::{self, funcs},
};

pub fn check(activity: &'static str, status: c_int) -> c_int {
    if status < 0 {
        unsafe {
            let error = &*funcs::giterr_last();
            println!(
                "error while {}: {} ({})",
                activity,
                CStr::from_ptr(error.message).to_string_lossy(),
                error.klass
            );
            std::process::exit(1);
        }
    }

    status
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn show_commit(commit: *const raw::funcs::git_commit) {
    let author = raw::funcs::git_commit_author(commit);

    let name = CStr::from_ptr((*author).name).to_string_lossy();
    let email = CStr::from_ptr((*author).email).to_string_lossy();
    println!("{} <{}>\n", name, email);

    let message = raw::funcs::git_commit_message(commit);
    println!("{}", CStr::from_ptr(message).to_string_lossy());
}

#[cfg(unix)]
pub fn path_to_string(path: &Path) -> Result<CString, Error> {
    use std::os::unix::ffi::OsStrExt;

    Ok(CString::new(path.as_os_str().as_bytes())?)
}

#[cfg(windows)]
pub fn path_to_string(path: &Path) -> Result<CString, Error> {
    match path.to_str() {
        Some(s) => Ok(CString::new(s)?),
        None => {
            let message = format!("Couldn't convert path '{}' to UTF-8", path.display());
            Err(message.into())
        }
    }
}

/// # Safety
/// `ptr`이 null이 아닌 때는 null로 끝나는 C문자열을 가리켜야 하며,
/// 이 문자열은 적어도 `_owner`의 수명동안 안전하게 접근할 수 있어야 한다.
pub unsafe fn char_ptr_to_str<T>(_owner: &T, ptr: *const c_char) -> Option<&str> {
    if ptr.is_null() {
        None
    } else {
        CStr::from_ptr(ptr).to_str().ok()
    }
}
