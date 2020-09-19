use tokio::io;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("0.0.0.0:8080").await?;

    let mut counter = 1u32;
    loop {
        let (socket, _) = listener.accept().await?;
        println!("Accepted connection #{}", counter);
        tokio::spawn(async move {
            match echo(socket).await {
                Ok(()) => println!("Connection #{} completed successfully", counter),
                Err(e) => println!("Connection #{} errored: {:?}", counter, e),
            }
        });
        counter += 1;
    }
}

async fn echo(socket: TcpStream) -> io::Result<()> {
    let (mut recv, mut send) = io::split(socket);
    io::copy(&mut recv, &mut send).await?;
    Ok(())
}
