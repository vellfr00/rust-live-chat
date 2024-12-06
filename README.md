# Rust Live Chat
A simple web-based chat application built with Rust. Permits to define rooms and chat with other users in it.
A backend web server is the central point of the application and accepts HTTP requests to manage the chat rooms and messages.
No persistence is managed as the server is stateless and all data is lost when the server is stopped.
A simple CLI-based client is also provided to interact with the server, implementing the basic functionalities of the chat application.

## Run the application
### Server
To run the server, execute:
```bash
cargo run -- --run=server --host=127.0.0.1 --port=3000
```
This will run the server on `http://127.0.0.1:3000`.

### Client
To run the client, execute:
```bash
cargo run -- --run=client --host=127.0.0.1 --port=3000
```
This will run the client and connect to the server on `http://127.0.0.1:3000`.

## Docker Server
The application backend web server can be run in a Docker container. To build the image, run:
```bash
docker build -t rust-live-chat .
```

To run the container, execute:
```bash
docker run -p 3000:3000 -e SERVER_PORT=3000 rust-live-chat
```

This will run the server inside the Docker container and expose the port `3000` to the host machine.
A CLI client can connect to the server inside the container as usual by running:
```bash
cargo run -- --run=client --host=127.0.0.1 --port=3000
```

## Tests
To run the tests, execute:
```bash
cargo test
```