# TLS server and client in Rust using rustls

## Usage

### Generate certificates

```sh
cd certs
chmod +x generate.sh
./generate.sh
```

### Server

```sh
cargo r -- bin server <server_endpoint> <root_cert> <private_key>

# example
cargo r -- bin server localhost:4443 certs/localhost_cert.pem certs/localhost_private.pem
```

### Client

```sh
cargo r -- bin client <server_endpoint> <server_port> <root_cert>

# example
cargo r -- bin client localhost 4443 certs/localhost_cert.pem
```
