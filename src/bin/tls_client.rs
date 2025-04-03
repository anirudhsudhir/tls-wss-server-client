use clap::Parser;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::{
    TlsConnector,
    rustls::pki_types::{CertificateDer, pem::PemObject},
};

use std::sync::Arc;

/// TLS client
#[derive(Parser, Debug)]
#[command(name = "TLS Client")]
#[command(version = env!("CARGO_PKG_VERSION"))]
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
    let mut tls = conn.connect(server_name, sock).await.unwrap();
    println!(
        "TLS connection to server {}:{} has been established!",
        args.server, args.port
    );

    let mut buf = vec![0u8; 3000];
    println!("Enter a message to be sent to the server:");
    let mut len = io::stdin().read(&mut buf).await.unwrap();
    // TODO: remove newline (currently using len-1)
    tls.write_all(&buf[..len - 1]).await.unwrap();

    println!("Waiting for a message from the server");
    len = tls.read(&mut buf).await.unwrap();
    io::stdout()
        .write_all(
            format!(
                "Received a message from the server: `{}`\n",
                String::from_utf8(buf[..len].to_vec()).unwrap()
            )
            .as_bytes(),
        )
        .await
        .unwrap();
}
