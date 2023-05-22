# TCP Echo Server with Rust
Run `cargo r` in the cloned directory.

In a new terminal, run:  `echo "hello echo" | nc localhost 8080` .

The response will be read into a buffer, printed to server console, and returned to client. The client's IP address and port will be logged on the server.

Example response:

```
Incoming Address: 127.0.0.1:58269
Incoming Message: hello echo
```

# `tcpdump` packet capture
Run `tcpdump -i lo0 -s 0 -w capture.pcap`.

Stop the dump after making requests to `localhost`/`lo0`.

Read the dump with `tcpdump -r capture.pcap | less`

Packet metadata will be readable, including the client host IP.

# Note
This is tested to be functional on FreeBSD and macOS.
