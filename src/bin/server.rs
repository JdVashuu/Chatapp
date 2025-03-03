use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[derive(Clone)]
struct ChatMessage {
    username: String,
    content: String,
    msg_type: MessageType,
}

#[derive(Clone)]
enum MessageType {
    Chat,
    System,
    UserJoined,
    UserLeft,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    //creating a buffer for the messages. 10 for now
    let (tx, _rx) = broadcast::channel::<ChatMessage>(10);
    let usernames = Arc::new(Mutex::new(HashMap::new()));

    println!("\nServer running on 127.0.0.1:8080\n");

    loop {
        //new connection
        let (mut socket, addr) = listener.accept().await.unwrap();

        //clone the transmitter for this connection
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let usernames = Arc::clone(&usernames);

        //new connection, new task
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            writer.write_all(b"Enter your username").await.unwrap();
            writer.flush().await.unwrap();

            let username = match reader.read_line(&mut line).await {
                Ok(_) => line.trim().to_string(),
                Err(_) => return,
            };
            line.clear();

            {
                let mut usernames = usernames.lock().await;
                usernames.insert(addr, username.clone());
            }

            // tx.send(ChatMessage {
            //     username: "System".to_string(),
            //     content: format!("{} joined the chat\n", username),
            //     msg_type: MessageType::UserJoined,
            // })
            // .unwrap();

            if let Err(e) = tx.send(ChatMessage {
                username: "System".to_string(),
                content: format!("{} joined the chat\n", username),
                msg_type: MessageType::UserJoined,
            }) {
                eprintln!("Failed to send message: {}", e);
            }

            writer
                .write_all(format!("Welcome, {}!\n", username).as_bytes())
                .await
                .unwrap();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0{
                            //user disconnectd
                            let mut usernames = usernames.lock().await;
                            if let Some(username) = usernames.remove(&addr){
                                // tx.send(ChatMessage{
                                //     username: "System".to_string(),
                                //     content: format!("{} left the chat\n", username),
                                //     msg_type: MessageType::UserLeft,
                                // }).unwrap();
                                //
                                if let Err(e) = tx.send(ChatMessage {
                                    username: "System".to_string(),
                                    content: format!("{} left the chat\n", username),
                                    msg_type: MessageType::UserLeft,
                                }) {
                                    eprintln!("Failed to send message: {}", e);
                                }
                            }
                            break;
                        }

                        // tx.send(ChatMessage{
                        //     username: username.clone(),
                        //     content: line.clone(),
                        //     msg_type: MessageType::Chat,
                        // }).unwrap();
                        // line.clear();

                        if let Err(e) = tx.send(ChatMessage {
                            username: username.clone(),
                            content: line.clone(),
                            msg_type:  MessageType::Chat,
                        }) {
                            eprintln!("Failed to send message: {}", e);
                        }

                    }
                    result = rx.recv() => {
                        let msg = result.unwrap();
                        match msg.msg_type{
                            MessageType::Chat =>{
                                if msg.username != username {
                                    writer.write_all(format!("{}: {}", msg.username, msg.content).as_bytes()).await.unwrap();
                                }
                            }
                            MessageType::System |
                            MessageType::UserJoined |
                            MessageType::UserLeft => {
                                writer.write_all(msg.content.as_bytes()).await.unwrap();
                            }
                        }
                    }
                }
            }
        });
    }
}
