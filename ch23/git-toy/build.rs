use cmake::Config;

fn main() {
    let dst = Config::new("libgit2").build();
    println!("cargo::rustc-link-search=native={}/build", dst.display());
    println!("cargo::rustc-link-lib=git2");
    println!("cargo::rerun-if-changed=libgit2/package.json")
}
