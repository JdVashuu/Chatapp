use std::error::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //connect to server
    let socket = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = socket.into_split();

    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin);
    let mut user_input = String::new();

    println!("\n\nConnected to chat server!\nPlease enter your username:    ");

    loop {
        tokio::select! {
            //handle message from the server
            result = reader.read_line(&mut line) => {
                if result? == 0{
                    println!("Server closed the connection");
                    break;
                }

                print!("{}", line);
                line.clear();
            }

            //handle user input
            result = stdin.read_line(&mut user_input) => {
                if result? == 0{
                    break;
                }

                writer.write_all(user_input.as_bytes()).await?;
                writer.flush().await?;
                user_input.clear();
            }
        }
    }
    Ok(())
}
