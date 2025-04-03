# A TLS and WSS server and client in Rust using rustls

## Usage

### Generate certificates

```sh
cd certs
chmod +x generate.sh
./generate.sh
```

### TLS Server

```sh
A TLS server in Rust, using rustls

Usage: tls_server [OPTIONS]

Options:
  -s, --server <SERVER>        The server endpoint [default: localhost:4443]
  -c, --cert-path <CERT_PATH>  Certificate path [default: certs/localhost_cert.pem]
  -k, --key-path <KEY_PATH>    Key path [default: certs/localhost_private.pem]
  -h, --help                   Print help
  -V, --version                Print version
```

### TLS Client

```sh
A TLS client in Rust, using rustls

Usage: tls_client [OPTIONS]

Options:
  -s, --server <SERVER>        The server endpoint [default: localhost]
  -p, --port <PORT>            The server port [default: 4443]
  -c, --cert-path <CERT_PATH>  Certificate path [default: certs/localhost_cert.pem]
  -h, --help                   Print help
  -V, --version                Print version
```

### WSS Server

```sh
A secure websockets server in Rust, using rustls

Usage: wss_server [OPTIONS]

Options:
  -s, --server <SERVER>        The server endpoint [default: localhost:4443]
  -c, --cert-path <CERT_PATH>  Certificate path [default: certs/localhost_cert.pem]
  -k, --key-path <KEY_PATH>    Key path [default: certs/localhost_private.pem]
  -h, --help                   Print help
  -V, --version                Print version
```

### WSS Client

```sh
A secure websockets client in Rust, using rustls

Usage: wss_client [OPTIONS]

Options:
  -s, --server <SERVER>        The server endpoint [default: localhost]
  -p, --port <PORT>            The server port [default: 4443]
  -c, --cert-path <CERT_PATH>  Certificate path [default: certs/localhost_cert.pem]
  -h, --help                   Print help
  -V, --version                Print version
```
