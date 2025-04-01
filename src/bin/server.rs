use clap::Parser;
use rustls::ServerConfig;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

/// TLS server
#[derive(Parser, Debug)]
#[command(name = "TLS Server")]
#[command(version = "0.1.0")]
#[command(about = "A TLS client in Rust, using rustls", long_about = None)]
struct Args {
    /// The server endpoint
    #[arg(short, long, default_value_t = String::from("localhost:4443"))]
    server: String,

    /// Certificate path
    #[arg(short, long, default_value_t = String::from("certs/localhost_cert.pem"))]
    cert_path: String,

    /// Key path
    #[arg(short, long, default_value_t = String::from("certs/localhost_private.pem"))]
    key_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    let config = configure_tls(&mut args).unwrap();

    let listener = TcpListener::bind(&args.server).unwrap();
    println!("TLS server running at {}", args.server);

    while let Ok((mut stream, _)) = listener.accept() {
        let cfg_clone = config.clone();
        thread::spawn(move || {
            println!("\nReceived new connection at server");
            let mut conn = rustls::ServerConnection::new(Arc::new(cfg_clone)).unwrap();
            let mut tls_stream = rustls::Stream::new(&mut conn, &mut stream);

            let mut buf = vec![0u8; 100];
            let len = tls_stream.read(&mut buf).unwrap();
            let _ = buf.split_off(len);

            println!(
                "Received message from client: {}",
                String::from_utf8(buf).unwrap()
            );

            let _ = tls_stream.write(b"Hello from the server").unwrap();
            println!("Closing current connection\n");
        });
    }

    Ok(())
}

fn configure_tls(args: &mut Args) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let certs = CertificateDer::pem_file_iter(args.cert_path.clone())
        .unwrap()
        .map(|cert| cert.unwrap())
        .collect();
    let private_key = PrivateKeyDer::from_pem_file(args.key_path.clone()).unwrap();
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)
        .unwrap();

    Ok(config)
}
