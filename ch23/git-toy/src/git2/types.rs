use std::{
    ffi::{c_char, CString},
    marker::PhantomData,
    mem,
    path::Path,
    ptr,
};

use super::{
    errors::{check, Error},
    helpers::{char_ptr_to_str, path_to_string},
    raw,
};

/// 깃 저장소
pub struct Repository {
    // 이 포인터는 늘 살아있는 `git_repository` 구조체를 가리켜야 한다.
    raw: *mut raw::funcs::git_repository,
}

/// 객체 실별자
/// 객체가 가진 내용의 와이드 해시이다.
pub struct Oid {
    pub raw: raw::funcs::git_oid,
}

pub struct Commit<'repo> {
    // 이 포인터는 살아있는 'git_commit'를 가리켜야 한다.
    raw: *mut raw::funcs::git_commit,
    _marker: PhantomData<&'repo Repository>,
}

impl Repository {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository, Error> {
        ensure_initialized();

        let path = path_to_string(path.as_ref())?;
        let mut repo = ptr::null_mut();
        unsafe {
            check(raw::funcs::git_repository_open(&mut repo, path.as_ptr()))?;
        }

        Ok(Repository { raw: repo })
    }

    pub fn reference_name_to_oid(&self, name: &str) -> Result<Oid, Error> {
        let name = CString::new(name)?;
        unsafe {
            let oid = {
                let mut oid = mem::MaybeUninit::uninit();
                check(raw::funcs::git_reference_name_to_id(
                    oid.as_mut_ptr(),
                    self.raw,
                    name.as_ptr() as *const c_char,
                ))?;
                oid.assume_init()
            };
            Ok(Oid { raw: oid })
        }
    }

    pub fn find_commit(&self, oid: &Oid) -> Result<Commit, Error> {
        let mut commit = ptr::null_mut();
        unsafe {
            check(raw::funcs::git_commit_lookup(
                &mut commit,
                self.raw,
                &oid.raw,
            ))?;
        }
        Ok(Commit {
            raw: commit,
            _marker: PhantomData,
        })
    }
}

pub struct Signature<'text> {
    raw: *const raw::funcs::git_signature,
    _marker: PhantomData<&'text str>,
}

impl<'repo> Commit<'repo> {
    pub fn author(&self) -> Signature {
        unsafe {
            Signature {
                raw: raw::funcs::git_commit_author(self.raw),
                _marker: PhantomData,
            }
        }
    }

    pub fn message(&self) -> Option<&str> {
        unsafe {
            let message = raw::funcs::git_commit_message(self.raw);
            char_ptr_to_str(self, message)
        }
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        unsafe {
            raw::funcs::git_repository_free(self.raw);
        }
    }
}

impl<'repo> Drop for Commit<'repo> {
    fn drop(&mut self) {
        unsafe {
            raw::funcs::git_commit_free(self.raw);
        }
    }
}

impl<'text> Signature<'text> {
    /// 적격한 UTF8이 아니면 `None`을 반환한다.
    pub fn name(&self) -> Option<&str> {
        unsafe { char_ptr_to_str(self, (*self.raw).name) }
    }

    /// 적격한 UTF8이 아니면 `None`을 반환한다.
    pub fn email(&self) -> Option<&str> {
        unsafe { char_ptr_to_str(self, (*self.raw).email) }
    }
}

fn ensure_initialized() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| unsafe {
        check(raw::funcs::git_libgit2_init()).expect("initializing libgit2 failed");
        assert_eq!(libc::atexit(shutdown), 0);
    });
}

extern "C" fn shutdown() {
    unsafe {
        if let Err(e) = check(raw::funcs::git_libgit2_shutdown()) {
            eprintln!("shutting down libgit2 failed: {}", e);
            std::process::abort();
        }
    }
}
