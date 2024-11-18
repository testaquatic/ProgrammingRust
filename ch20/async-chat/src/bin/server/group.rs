use std::sync::Arc;

use async_chat::FromServer;
use tokio::{net::tcp::OwnedWriteHalf, sync::broadcast::{self, error::RecvError}, task::JoinHandle};

use crate::connection::Outbound;

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn new(name: Arc<String>) -> Self {
        let (sender, _receiver) = broadcast::channel(1000);
        Group { name, sender }
    }

    pub async fn join<'a>(&self, outbound: Arc<Outbound<OwnedWriteHalf>>) -> JoinHandle<()>  {
        let receiver = self.sender.subscribe();
        let group_name = self.name.clone();
        tokio::spawn(async move {
            handle_subscriver(group_name, receiver, outbound).await;
        })
    }

    pub fn post(&self, message: Arc<String>) {
        // 구독자가 없는 경우에만 오류를 반환한다.
        let _ignored = self.sender.send(message);
    }
}

async fn handle_subscriver(
    group_name: Arc<String>,
    mut receiver: broadcast::Receiver<Arc<String>>,
    outbound: Arc<Outbound<OwnedWriteHalf>>,
) {
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => FromServer::Message {
                group_name: group_name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => {
                FromServer::Error(format!("Dropped {} messages from {}.", n, group_name))
            }
            Err(RecvError::Closed) => break,
        };

        // dbg!(&packet);

        if outbound.send(packet).await.is_err() {
            break;
        }
    }
}
