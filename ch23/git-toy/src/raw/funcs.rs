use std::ffi::{c_char, c_int, c_uchar};

use super::errors::git_error;

#[link(name = "git2")]
extern "C" {
    pub fn git_libgit2_init() -> c_int;
    pub fn git_libgit2_shutdown() -> c_int;
    pub fn giterr_last() -> *const git_error;
    pub fn git_repository_open(out: *mut *mut git_repository, path: *const c_char) -> c_int;
    pub fn git_repository_free(repo: *mut git_repository);
    pub fn git_reference_name_to_id(
        out: *mut git_oid,
        repo: *mut git_repository,
        reference: *const c_char,
    ) -> c_int;
    pub fn git_commit_lookup(
        out: *mut *mut git_commit,
        repo: *mut git_repository,
        id: *const git_oid,
    ) -> c_int;
    pub fn git_commit_author(commit: *const git_commit) -> *const git_signature;
    pub fn git_commit_message(commit: *const git_commit) -> *const c_char;
    pub fn git_commit_free(commit: *mut git_commit);

}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct git_repository {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct git_commit {
    _private: [u8; 0],
}

pub const GIT_OID_RAWZ: usize = 20_usize;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct git_oid {
    pub id: [c_uchar; GIT_OID_RAWZ],
}

#[allow(non_camel_case_types)]
pub type git_time_t = i64;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct git_time {
    pub time: git_time_t,
    pub offset: c_int,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct git_signature {
    pub name: *const c_char,
    pub email: *const c_char,
    pub when: git_time,
}
