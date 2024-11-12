use fingertips::{
    parse_args::{self}, run::run
};

fn main() {
    let args = parse_args::parse_args();
    run(args).unwrap_or_else(|e| println!("error: {}", e));
}
