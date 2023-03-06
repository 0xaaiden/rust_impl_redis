// Uncomment this block to pass the first stage
// use std::net::{TcpListener, TcpStream};
// use std::io::{Read, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, self};
// use tokio::io;
async fn handle_connection(mut stream: TcpStream) -> io::Result<()>  {
    let mut buf = [0; 512];

    loop {
        // Wait for the client to send us a message but ignore the content for now
        let bytes_read = stream.read(&mut buf).await?;
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }

        stream.write("+PONG\r\n".as_bytes()).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    //
    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((mut stream, _)) => {
                println!("accepted new connection {:?}", stream);

                tokio::spawn(async move {
                    handle_connection(stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }


}
