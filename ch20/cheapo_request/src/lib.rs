use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net, task,
};

pub async fn cheapo_request(host: &str, port: u16, path: &str) -> Result<String, std::io::Error> {
    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown().await?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

pub async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<Result<String, io::Error>> {
    let mut handles = Vec::with_capacity(requests.len());

    for (host, port, path) in requests {
        let handle = task::spawn(async move { cheapo_request(&host, port, &path).await });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        let result = task::spawn(handle).await.unwrap().unwrap();
        results.push(result);
    }

    results
}
