use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{self, AsyncReadExt},
    net::TcpListener,
};
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        ServerConfig,
        pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
    },
};
use tokio_tungstenite::{self, tungstenite::Message};

use std::error::Error;
use std::sync::Arc;

const TERMINATE_CONN_COMMAND: &str = "TERMINATE_CONN";

/// WSS server
#[derive(Parser, Debug)]
#[command(name = "WSS Server")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A secure websockets server in Rust, using rustls", long_about = None)]
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
    println!("Secure websockets server running at {}", args.server);

    while let Ok((stream, addr)) = listener.accept().await {
        let acceptor = tls_acceptor.clone();
        tokio::spawn(async move {
            let tls_stream = acceptor.accept(stream).await.unwrap();
            let ws_stream = tokio_tungstenite::accept_async(tls_stream).await.unwrap();
            println!(
                "\nWSS connection to server has been established from {:?}!",
                addr
            );

            let (mut outgoing, mut incoming) = ws_stream.split();
            let mut buf = vec![0u8; 3000];

            loop {
                println!("Waiting for a message from the client");
                if let Some(Ok(msg)) = incoming.next().await {
                    let msg_str = String::from_utf8(msg.into_data().to_vec()).unwrap();

                    if msg_str == TERMINATE_CONN_COMMAND {
                        println!(
                            "Received a `{}` message, terminating connection",
                            TERMINATE_CONN_COMMAND
                        );
                        break;
                    }
                    println!("Received a message from the client: `{msg_str}`",);

                    println!("Enter a message to be sent to the client:");
                    let len = io::stdin().read(&mut buf).await.unwrap();
                    // TODO: remove newline (currently using len-1)
                    outgoing
                        .send(Message::binary(buf[..len - 1].to_vec()))
                        .await
                        .unwrap();
                } else {
                    break;
                }
            }
        });
    }

    Ok(())
}
