use std::net::TcpListener;

use zero_to_prod_example::run;

#[tokio::main] // <- this is the same as tokio::main
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address");
    // We launch the server in a background task
    // tokio::spawn returns a handle to the spawned future, but we don't need it here

    println!("Server running on: http://127.0.0.1:{}", port);
    server.await?;

    println!("\n Server stopped");
    Ok(())
}
