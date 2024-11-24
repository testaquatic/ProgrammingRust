use std::{
    ffi::{c_char, CString},
    mem, ptr,
};

use clap::{Arg, Command};
use git_toy::{
    helpers::{check, show_commit},
    raw,
};

struct Args {
    path: String,
}

fn parse_args() -> Args {
    let matches = Command::new("git-toy")
        .version("v0.1")
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .num_args(1)
                .required(true),
        )
        .get_matches();

    if let Some(path) = matches.get_one::<String>("path") {
        return Args { path: path.clone() };
    }
    unreachable!();
}

fn main() {
    let path = parse_args().path;
    let path = CString::new(path).expect("path contains null characters");

    unsafe {
        check("initializing library", raw::funcs::git_libgit2_init());

        let mut repo = ptr::null_mut();
        check(
            "opening repository",
            raw::funcs::git_repository_open(&mut repo, path.as_ptr()),
        );

        let c_name = b"HEAD\0".as_ptr() as *const c_char;
        let oid = {
            let mut oid = mem::MaybeUninit::uninit();
            check(
                "looking up HEAD",
                raw::funcs::git_reference_name_to_id(oid.as_mut_ptr(), repo, c_name),
            );
            oid.assume_init()
        };

        let mut commit = ptr::null_mut();
        check(
            "looking up commit",
            raw::funcs::git_commit_lookup(&mut commit, repo, &oid),
        );

        show_commit(commit);

        raw::funcs::git_commit_free(commit);

        raw::funcs::git_repository_free(repo);

        check("shutting down library", raw::funcs::git_libgit2_shutdown());
    }
}
