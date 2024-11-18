use std::sync::Arc;

use async_chat::args::parse_args;
use connection::serve;
use group_table::GroupTable;

mod connection;
mod group;
mod group_table;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let chat_group_table = Arc::new(GroupTable::new());

    let listener = parse_args()
        .ok_or(anyhow::anyhow!("Address not entered."))?
        .get_listener()
        .await?;

    loop {
        let (socket, _) = listener.accept().await?;
        // println!("{} connected", socket.peer_addr().unwrap());
        let groups = chat_group_table.clone();
        tokio::spawn(async move {
            if let Err(e) = serve(socket, groups).await {
                eprintln!("Error: {e:?}");
            }
        });
    }
}
