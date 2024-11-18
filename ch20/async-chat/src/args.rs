use clap::{Arg, Command};
use tokio::net;

pub struct Args {
    address: String,
}

impl Args {
    pub async fn get_stream(&self) -> Result<net::TcpStream, tokio::io::Error> {
        net::TcpStream::connect(&self.address).await
    }

    pub async fn get_listener(&self) -> Result<net::TcpListener, tokio::io::Error> {
        net::TcpListener::bind(&self.address).await
    }
}

pub fn parse_args() -> Option<Args> {
    let matches = Command::new("async_chat_client")
        .version("0.1")
        .arg(Arg::new("address").required(true))
        .get_matches();

    let args = Args {
        address: matches.get_one::<String>("address")?.to_owned(),
    };

    Some(args)
}
