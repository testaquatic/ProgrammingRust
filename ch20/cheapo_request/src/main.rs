use cheapo_request::many_requests;

fn main() -> Result<(), std::io::Error> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let requests = vec![
        ("example.com".to_string(), 80, "/".to_string()),
        ("www.red-bean.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
    ];
    let results = rt.block_on(many_requests(requests));
    results.into_iter().for_each(|result| match result {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("error: {}", e),
    });

    Ok(())
}
