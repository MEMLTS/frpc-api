use axum::{routing, Router};
use tokio::net::TcpListener;

mod edit;

#[tokio::main]
async fn main() {




    let router = Router::new()
        .route("/", routing::get(||async { "Hello, World!" }));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started at http://{:?}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
