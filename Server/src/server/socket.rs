use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt};
use crate::algorithm::test;
use crate::server::Buyer::Buyer;
use crate::stadium::structures::Zone;

#[tokio::main]
pub async fn server(stadium: &HashMap<String, Zone>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Servidor escuchando en 127.0.0.1:7878");

    loop {
        // Aceptar una conexión entrante
        let (mut socket, _) = listener.accept().await.unwrap();

        // Clonar el socket para usarlo en la tarea asíncrona //.
        tokio::spawn(async move {
            // Buffer para leer datos
            let mut buf = vec![0; 1024];

            // Leer datos del socket
            match socket.read(&mut buf).await {

                Ok(n) if n == 0 => return, // Conexión cerrada por parte del cliente

                Ok(n) => {
                    // Buffer para cargar los datos
                    let data = &buf[..n];

                    match serde_json::from_slice::<Buyer>(data) { // Deserializa los datos
                        Ok(buyer) => { // Verifica que la deserialización sea exitosa
                            // Aquí se implementa la lógica
                            println!("Datos recibidos: {:?}", buyer);
                            //test(&stadium, buyer);


                        }
                        Err(e) => println!("Error al deserializar: {}", e),
                    }
                }
                Err(e) => {
                    println!("Error al leer del socket: {}", e);
                }
            }
        });
    }
}
