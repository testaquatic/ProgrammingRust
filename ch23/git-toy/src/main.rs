use clap::{Arg, Command};
use git_toy::git2::types::Repository;

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

    let repo = Repository::open(&path).expect("openning repository");

    let commit_oid = repo
        .reference_name_to_oid("HEAD")
        .expect("looking up 'HEAD' reference");

    let commit = repo.find_commit(&commit_oid).expect("looking up commit");

    let author = commit.author();
    println!(
        "{} <{}>\n",
        author.name().unwrap_or("(none)"),
        author.email().unwrap_or("none")
    );
    println!("{}", commit.message().unwrap_or("(none)"));
}
