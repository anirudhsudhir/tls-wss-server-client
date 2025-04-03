use clap::Parser;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
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
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A TLS server in Rust, using rustls", long_about = None)]
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
    let args = Args::parse();

    let certs = CertificateDer::pem_file_iter(args.cert_path.clone())
        .unwrap()
        .map(|cert| cert.unwrap())
        .collect();
    let private_key = PrivateKeyDer::from_pem_file(args.key_path.clone()).unwrap();
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)
        .unwrap();

    let tls_acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind(&args.server).await.unwrap();
    println!("TLS server running at {}", args.server);

    while let Ok((stream, addr)) = listener.accept().await {
        let acceptor = tls_acceptor.clone();
        tokio::spawn(async move {
            let mut tls_stream = acceptor.accept(stream).await.unwrap();
            println!(
                "\nTLS connection to server has been established from {:?}!",
                addr
            );

            let mut buf = vec![0u8; 3000];
            println!("Waiting for a message from the client");
            let mut len = tls_stream.read(&mut buf).await.unwrap();
            println!(
                "Received a message from the client: `{}`",
                String::from_utf8(buf[..len].to_vec()).unwrap()
            );

            println!("Enter a message to be sent to the client:");
            len = io::stdin().read(&mut buf).await.unwrap();
            // TODO: remove newline (currently using len-1)
            tls_stream.write_all(&buf[..len - 1]).await.unwrap();

            println!("Closing connection to {:?}\n", addr);
        });
    }

    Ok(())
}
