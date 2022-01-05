# tokio-udp-protobuf-example

Example package for communication using UDP socket and protobuf for gaming server development

- start client

```
cargo run --bin client
```

- start server

```
cargo run --bin server
```

When client is running, user can type input from the stdin, the message then will be converted to a protobuf message and will then send to the server.

Server will echo back the messages to client, and client will receive it and print it.
