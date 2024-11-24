use std::ffi::{c_int, CStr};

use crate::raw::{self, funcs};

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
