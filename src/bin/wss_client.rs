use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use tokio::io::{self, AsyncReadExt};
use tokio_rustls::rustls::pki_types::{CertificateDer, pem::PemObject};
use tokio_tungstenite::{Connector, connect_async_tls_with_config, tungstenite::Message};

use std::sync::Arc;

const TERMINATE_CONN_COMMAND: &str = "TERMINATE_CONN";

/// WSS client
#[derive(Parser, Debug)]
#[command(name = "WSS Client")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A secure websockets client in Rust, using rustls", long_about = None)]
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

    let conn = Connector::Rustls(Arc::new(config));

    let (ws_stream, _) = connect_async_tls_with_config(
        format!("wss://{}:{}", args.server, args.port),
        None,
        false,
        Some(conn),
    )
    .await
    .unwrap();

    let (mut outgoing, mut incoming) = ws_stream.split();

    println!(
        "Connection to secure websockets server {}:{} has been established!",
        args.server, args.port
    );

    let mut buf = vec![0u8; 3000];

    loop {
        println!(
            "Enter a message to be sent to the server (Enter `{}` to terminate the connection):",
            TERMINATE_CONN_COMMAND
        );
        let len = io::stdin().read(&mut buf).await.unwrap();
        // TODO: remove newline (currently using len-1)
        outgoing
            .send(Message::binary(buf[..len - 1].to_vec()))
            .await
            .unwrap();

        if String::from_utf8(buf[..len - 1].to_vec()).unwrap() == TERMINATE_CONN_COMMAND {
            break;
        }

        println!("Waiting for a message from the server");
        if let Some(Ok(msg)) = incoming.next().await {
            println!(
                "Received a message from the server: `{}`",
                String::from_utf8(msg.into_data().to_vec()).unwrap()
            );
        }
    }
}
