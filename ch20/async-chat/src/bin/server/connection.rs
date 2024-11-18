use std::sync::Arc;

use async_chat::{utils, FromClient, FromServer};
use tokio::{
    io::{self, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::Mutex,
};
use tokio_stream::StreamExt;

use crate::group_table::GroupTable;

pub async fn serve(socket: TcpStream, groups: Arc<GroupTable>) -> Result<(), io::Error> {
    let (socket_reader, socket_writer) = socket.into_split();

    let outbound = Arc::new(Outbound::new(socket_writer));
    let buffered = BufReader::new(socket_reader);
    let mut from_client = utils::receive_as_json(buffered);

    while let Some(request_result) = from_client.next().await {
        let request = request_result?;

        let result = match request {
            FromClient::Join { group_name } => {
                // println!("{} joined", &group_name);
                let group = groups.get_or_create(group_name);
                group.join(outbound.clone()).await;
                Ok(())
            }
            FromClient::Post {
                group_name,
                message,
            } => match groups.get(&group_name) {
                Some(group) => {
                    group.post(message);
                    Ok(())
                }
                None => Err(format!("Group '{}' does not exist", group_name)),
            },
        };
        if let Err(message) = result {
            let report = FromServer::Error(message);
            outbound.send(report).await?;
        }
    }

    Ok(())
}

pub struct Outbound<T>(Mutex<T>) where T: AsyncWriteExt;

impl<T> Outbound<T> where  T: AsyncWriteExt + Unpin {
    pub fn new(to_client: T) -> Self {
        Outbound(Mutex::new(to_client))
    }

    pub async fn send(&self, packet: FromServer) -> Result<(), io::Error> {
        let mut guard = self.0.lock().await;
        utils::send_as_json(&mut *guard, &packet).await?;
        guard.flush().await?;
        Ok(())
    }
}
