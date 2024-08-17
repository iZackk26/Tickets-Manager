use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut stream: TcpStream) {
    // Enviar el mensaje "Hola mundo" al cliente
    let message = b"Hola Mundo XD!";
    if let Err(e) = stream.write_all(message).await {
        eprintln!("Error al enviar el mensaje: {}", e);
        return;
    }

    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break, // El cliente cerró la conexión
            Ok(_) => {
                // El servidor simplemente reenvía lo que recibe del cliente
                if let Err(e) = stream.write_all(&buffer).await {
                    eprintln!("Error al enviar el mensaje: {}", e);
                }
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
