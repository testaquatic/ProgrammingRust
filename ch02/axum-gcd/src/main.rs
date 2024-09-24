use axum::{
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use tokio::net::TcpListener;

#[derive(serde::Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn get_index() -> impl IntoResponse {
    let body = include_str!("body.html");

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body(body.into_response())
        .unwrap()
}

async fn post_gcd(Form(form): Form<GcdParameters>) -> impl IntoResponse {
    if form.n == 0 || form.m == 0 {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Computing the GCD with zero is boring.".into_response())
            .unwrap();
    }

    let response = format!(
        "The greatest common divisor of the number {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    )
    .into_response();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body(response)
        .unwrap()
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            (m, n) = (n, m);
        }
        m = m % n;
    }
    n
}

fn main() {
    let app = Router::new()
        .route("/", get(get_index))
        .route("/gcd", post(post_gcd));
    let task = async {
        println!("Serving on http://localhost:3000...");
        axum::serve(
            TcpListener::bind("127.0.0.1:3000")
                .await
                .expect("error binding server to address."),
            app,
        )
        .await
        .unwrap();
    };

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(task);
}
