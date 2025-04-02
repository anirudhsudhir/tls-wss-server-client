use clap::Parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        ServerConfig,
        pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
    },
};

use std::error::Error;
use std::sync::Arc;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    let tls_acceptor = TlsAcceptor::from(Arc::new(configure_tls(&mut args)?));
    let listener = TcpListener::bind(&args.server).await.unwrap();
    println!("TLS server running at {}", args.server);

    while let Ok((stream, _)) = listener.accept().await {
        let acceptor = tls_acceptor.clone();
        tokio::spawn(async move {
            println!("\nReceived new connection at server, starting TLS stream");
            let mut tls_stream = acceptor.accept(stream).await.unwrap();

            let mut buf = vec![0u8; 100];
            let len = tls_stream.read(&mut buf).await.unwrap();
            let _ = buf.split_off(len);

            println!(
                "Received message from client: {}",
                String::from_utf8(buf).unwrap()
            );

            tls_stream
                .write_all(b"Hello from the server")
                .await
                .unwrap();
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
