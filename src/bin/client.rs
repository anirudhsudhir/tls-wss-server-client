use clap::Parser;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::pem::PemObject;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, stdout},
    net::TcpStream,
};
use tokio_rustls::TlsConnector;

use std::sync::Arc;

/// TLS client
#[derive(Parser, Debug)]
#[command(name = "TLS Client")]
#[command(version = "0.1.0")]
#[command(about = "A TLS client in Rust, using rustls", long_about = None)]
struct Args {
    /// The server endpoint
    #[arg(short, long, default_value_t = String::from("localhost"))]
    server: String,

    /// The server port
    #[arg(short, long, default_value_t = String::from("4443"))]
    port: String,

    /// Certificate path
    #[arg(short, long, default_value_t = String::from("certs/localhost_cert.pem"))]
    cert_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let root_cert = CertificateDer::pem_file_iter(args.cert_path)
        .unwrap()
        .map(|cert| cert.unwrap());
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_parsable_certificates(root_cert);
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let server_name = args.server.clone().try_into().unwrap();
    let conn = TlsConnector::from(Arc::new(config));

    let sock = TcpStream::connect(format!("{}:{}", args.server, args.port))
        .await
        .unwrap();
    println!(
        "Connected to TCP socket, starting TLS connection to {}:{}",
        args.server, args.port
    );

    let mut tls = conn.connect(server_name, sock).await.unwrap();
    println!("Writing to TLS stream");

    tls.write_all("hello from the client".as_bytes())
        .await
        .unwrap();

    let mut plaintext = vec![0u8; 1000];
    let len = tls.read(&mut plaintext).await.unwrap();
    let _ = plaintext.split_off(len);

    stdout()
        .write_all(
            format!(
                "Response from the server: `{}`\n",
                String::from_utf8(plaintext).unwrap()
            )
            .as_bytes(),
        )
        .await
        .unwrap();
}
