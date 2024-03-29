use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server

    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind to port 8080");
    zero2prod::run(listener)?.await
}
