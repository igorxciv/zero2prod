use std::net::TcpListener;
use zero2prod::startup::run;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind port");
    run(listener)?.await
}
