use std::net::SocketAddr;
use tracing::error;
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, World!"
}
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let app=Router::new().route("/", get(hello_world));
    let addr=SocketAddr::from(([127, 0, 0, 1], 3000)) ;
    println!("Listening on http://{}",addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| -> Box<dyn std::error::Error> {
            error!("Server error: {}", e);
            Box::new(e) as _
        })?;

    Ok(())
}

    



