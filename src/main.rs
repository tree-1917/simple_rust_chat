use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server");

    let mut input = String::new();
    let mut buffer = [0; 1024];

    loop {
        print!("Enter message: ");
        io::stdout().flush()?; // Ensure the prompt is shown

        input.clear();
        io::stdin().read_line(&mut input)?;

        // If the client types 'exit', terminate the chat
        if input.trim().to_lowercase() == "exit" {
            println!("Exiting the chat...");
            break;
        }

        // Send the message to the server
        stream.write_all(input.as_bytes())?;

        // Read server response
        let n = stream.read(&mut buffer)?;
        println!("{}", str::from_utf8(&buffer[0..n]).unwrap());
    }

    Ok(())
}

