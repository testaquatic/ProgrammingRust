use clap::{Arg, Command};

async fn http_get_main(url: &str) -> Result<(), anyhow::Error> {
    // HTTP 요청을 보내고 응답을 받는다.
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        anyhow::bail!("{}", response.status());
    }

    let response_bytes = response.bytes().await?;
    // 응답 본문을 읽어서 stdout에 쓴다.
    let mut stdout = tokio::io::stdout();
    tokio::io::copy(&mut response_bytes.as_ref(), &mut stdout).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Command::new("http_get")
        .version("0.1.0")
        .arg(Arg::new("url").num_args(1).required(true))
        .get_matches();

    let url: String = args.get_one::<String>("url").unwrap().to_owned();

    http_get_main(&url).await.expect("error: ");
}
