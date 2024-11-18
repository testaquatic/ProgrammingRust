async  fn many_requests(urls: &[String]) -> Vec<Result<String, reqwest::Error>> {
    let client = reqwest::Client::new();
    let mut responses = Vec::with_capacity(urls.len());
    for url in urls {
        let response = client.get(url).send().await;
        responses.push(response);
    }
    
    let mut results = Vec::with_capacity(responses.len());
    for handle in responses { 
        let response = match handle {
            Ok(reponse) => reponse,
            Err(e) => {results.push(Err(e)); continue;},
        };
        results.push(response.text().await);
    }

    results
}

#[tokio::main]
async fn main() {
    let requests = vec![
        "example.com".to_string(),
        "www.red-bean.com".to_string(),
        "en.wikipedia.org".to_string(),
    ];
    let results = many_requests(&requests).await;
    results.into_iter().for_each(|result| match result {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("error: {}", e),
    });
}
