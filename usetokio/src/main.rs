use tokio::io;
use tokio::process::Command;

async fn hello() -> Result<(), std::io::Error> {
    let mut stdout = io::stdout();
    let mut hello: &[u8] = b"Hello, world!\n";
    io::copy(&mut hello, &mut stdout).await?;
    Ok(())
}

async fn copy_stdin() -> Result<(), std::io::Error> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    io::copy(&mut stdin, &mut stdout).await?;
    Ok(())
}

async fn spawn_process() -> Result<(), std::io::Error> {
    for _ in 0..10 {
        Command::new("echo").arg("Hello, world!").spawn()?.await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    hello().await?;
    copy_stdin().await?;
    spawn_process().await?;
    Ok(())
}
