use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(_) => {
                stream.write_all(&buffer).await.unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Servidor escuchando en 127.0.0.1:7878");

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}