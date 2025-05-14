use std::io::{self, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        // Wait for the client to send a message
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break; // Connection closed
            }
            Ok(n) => {
                let received_msg = String::from_utf8_lossy(&buffer[0..n]);

                // Display the message from the client
                println!("User Say: {}", received_msg.trim());

                // Server operator can input a dynamic response
                print!("Server: Enter your reply: ");
                io::stdout().flush().unwrap();
                
                let mut server_response = String::new();
                io::stdin().read_line(&mut server_response).unwrap();

                // Trim newline and send response
                let server_response = server_response.trim();

                // Debug print the message being sent to the client
                println!("Server sending reply: {}", server_response);

                // Ensure the message is fully sent to the client
                if let Err(e) = stream.write_all(server_response.as_bytes()) {
                    eprintln!("Failed to send message: {}", e);
                    break;
                }

                // Ensure that the stream is flushed after sending the response
                if let Err(e) = stream.flush() {
                    eprintln!("Failed to flush stream: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                break;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server is listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}

