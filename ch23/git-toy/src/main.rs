use git_toy::{git_libgit2_init, git_libgit2_shutdown};

fn main() {
    unsafe {
        git_libgit2_init();
        git_libgit2_shutdown();
    }
}
