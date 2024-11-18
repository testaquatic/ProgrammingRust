use std::{borrow::BorrowMut, sync::Arc};

use async_chat::{args::parse_args, utils, FromClient, FromServer};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt},
    net::{self, tcp},
};
use tokio_stream::StreamExt;

async fn send_commands(mut to_server: net::tcp::OwnedWriteHalf) -> Result<(), tokio::io::Error> {
    println!(
        "\
    Commands:\n\
    \tjoin GROUP\n\
    \tpost GROUP MESSAGE...\n\
    \tTypoe CTRL+D(UNIX) or CTRL+Z(Windows) to close the connection.
    "
    );

    let mut command_liens = io::BufReader::new(io::stdin()).lines();
    while let Some(command) = command_liens.next_line().await? {
        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };
        utils::send_as_json(to_server.borrow_mut(), &request).await?;
        to_server.flush().await?;
    }

    Ok(())
}

fn parse_command(line: &str) -> Option<FromClient> {
    let (command, rest) = get_next_token(line)?;
    match command {
        "post" => {
            let (group, rest) = get_next_token(rest)?;
            let message = rest.trim_start().to_string();

            Some(FromClient::Post {
                group_name: Arc::new(group.to_string()),
                message: Arc::new(message),
            })
        }
        "join" => {
            let (group, rest) = get_next_token(rest)?;
            if !rest.trim_start().is_empty() {
                return None;
            }

            Some(FromClient::Join {
                group_name: Arc::new(group.to_string()),
            })
        }
        _ => {
            eprintln!("Unrecognized  command: {:?}", line);

            None
        }
    }
}

fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(space) => Some((&input[0..space], &input[space..])),
        None => Some((input, "")),
    }
}

async fn handle_replies(from_server: tcp::OwnedReadHalf) -> Result<(), io::Error> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);

    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("message posted to {}: {}", group_name, message);
            }
            FromServer::Error(message) => println!("error from server: {}", message),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let socket = parse_args()
        .ok_or(anyhow::anyhow!("주소를 입력하지 않았습니다."))?
        .get_stream()
        .await?;
    socket.set_nodelay(true)?;
    let (socket_reader, socket_writer) = socket.into_split();

    let to_server = send_commands(socket_writer);
    let from_server = handle_replies(socket_reader);

    tokio::try_join!(to_server, from_server)?;

    Ok(())
}
