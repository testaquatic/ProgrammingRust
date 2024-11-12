use fingertips::{
    parse_args::{self},
    run::{self},
};

fn main() {
    let args = parse_args::parse_args();
    run::run(args).unwrap_or_else(|e| println!("error: {}", e));
}
