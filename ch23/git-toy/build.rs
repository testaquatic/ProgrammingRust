use std::{clone, fs};

use cmake::Config;

fn main() {
    let dst = Config::new("libgit2")
        // .define("BUILD_SHARED_LIBS", "OFF")
        // .define("USE_THREADS", "OFF")
        .build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    // println!("cargo::rustc-link-lib=static=git2");
}
