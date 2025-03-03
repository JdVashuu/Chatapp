# Chat Application in Rust

A simple real-time chat application built with Rust and Tokio, featuring a client-server architecture for text-based communication.

## Features

- Real-time messaging between multiple users
- Server broadcasts messages to all connected clients
- Username-based identification
- Connection/disconnection notifications
- Asynchronous I/O for efficient resource usage

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust installation)

### Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/chat-app.git
cd chat-app
```

### Running the Application

1. Start the server:

```bash
cargo run --bin server
```

2. In a separate terminal, start a client:

```bash
cargo run --bin client
```

You can run multiple clients to simulate multiple users in the chat.

## Usage

1. When you start a client, you'll be prompted to enter a username
2. Start typing messages, which will be sent to all connected users
3. Press Ctrl+C to exit a client or server

## Project Structure

- `src/bin/server.rs` - The chat server implementation
- `src/bin/client.rs` - The client application for connecting to the server

## Technical Details

- Built using Tokio for asynchronous operations
- Uses TCP sockets for communication
- Leverages Rust's ownership model for memory safety
- Employs broadcast channels to distribute messages to connected clients

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Tokio](https://tokio.rs/) - The asynchronous runtime for Rust
- Rust community for documentation and inspiration
