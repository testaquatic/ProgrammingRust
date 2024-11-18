use serde::{de::DeserializeOwned, Serialize};
use tokio::io::AsyncBufReadExt;
use tokio_stream::{wrappers::LinesStream, Stream, StreamExt};

pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> Result<(), tokio::io::Error>
where
    S: tokio::io::AsyncWriteExt + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;

    Ok(())
}

pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = Result<P, tokio::io::Error>>
where
    S: AsyncBufReadExt + Unpin,
    P: DeserializeOwned,
{
    LinesStream::new(inbound.lines()).map(|line_result| {
        let line = line_result?;
        let parsed = serde_json::from_str::<P>(&line)?;
        Ok(parsed)
    })
}
