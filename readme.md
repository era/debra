# Debra: A simple chat demo application using QUIC

## Overview

**Debra** is a simple chat demo application that utilizes the QUIC protocol through the `quinn` (Dexter fans, did you see what I did here? :P) library in Rust. It allows multiple clients to connect to a server and send messages to each other in real-time. 

It uses flatbuffers (so we can use zero-copy on the server) for the communication between clients. 


To run the server:

`RUST_LOG=info cargo run --bin server`

To run the clients you must specify the client id:

`cargo run --bin client -- --client-id 2`

After that you enter the id of the client you want to send a message, press enter.
Write the message and press enter, done!


I describe this as a "chat application", but the main idea is that you can have multiple
clients, sending messages to each other through the server. This could be used to for different
reasons.


## References

- [QUIC](https://quicwg.org/) - The protocol that powers this chat application.
- [FlatBuffers](https://google.github.io/flatbuffers/) - For efficient serialization of messages.

