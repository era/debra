use anyhow::anyhow;
use debra::flatbuffers::root_as_message;
use debra::flatbuffers::{Message, MessageArgs, MessageType};
use flatbuffers::FlatBufferBuilder;
use quinn::{ClientConfig, Endpoint};
use quinn_proto::crypto::rustls::QuicClientConfig;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use std::io;
use std::io::Write;
use std::process::exit;
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish(),
    )
    .unwrap();
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let config = debra::config::ClientConfig::get();
    let client_id = config.client_id;

    run_client(config.listen, client_id).await.unwrap();
}

async fn run_client(
    server_addr: SocketAddr,
    client_id: i32,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut endpoint = Endpoint::client(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))?;

    endpoint.set_default_client_config(ClientConfig::new(Arc::new(QuicClientConfig::try_from(
        rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth(),
    )?)));

    info!("connecting to server");

    // connect to server
    let connection = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    info!("connected: addr={}", connection.remote_address());

    let (mut send, recv) = connection
        .open_bi()
        .await
        .map_err(|e| anyhow!("failed to open stream: {}", e))?;

    info!("sending first message");
    let message = connect_message(client_id);

    send_message(&mut send, message).await;
    info!("message send");

    reading_stream(recv);

    let _ = tokio::task::spawn_blocking(move || reading_chat(client_id, send)).await;

    // Make sure the server has a chance to clean up
    endpoint.wait_idle().await;

    Ok(())
}

fn reading_stream(mut receiver: quinn::RecvStream) {
    tokio::spawn(async move {
        loop {
            let bytes = debra::common::read_bytes(&mut receiver).await;
            match bytes {
                Err(e) => {
                    println!("failed to read message: {:?}", e);
                    exit(1);
                }
                Ok(bytes) => {
                    let message = root_as_message(&bytes)
                        .map_err(|e| anyhow!("failed parsing message: {:?}", e))
                        .unwrap();
                    println!(
                        "\n\nmessage received: {:?}",
                        message.message().map(|m| std::str::from_utf8(m.bytes()))
                    );
                    io::stdout().flush().unwrap();
                }
            }
        }
    });
}

///TODO: This is really bad, but it's just a super dirty quick thing to show this
fn reading_chat(client_id: i32, mut send: quinn::SendStream) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let for_client: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid integer.");
                continue;
            }
        };


        let mut string_input = String::new();
        io::stdin()
            .read_line(&mut string_input)
            .expect("Failed to read line");

        let message = string_input.trim();

        let bytes = message_for_client(client_id, for_client, message.as_bytes().to_vec());
        // terrible, I know, I just don't care enough given this is just a demo
        futures::executor::block_on(send_message(&mut send, bytes));
    }
}

async fn send_message(send: &mut quinn::SendStream, message: Vec<u8>) {
    send.write_all(&message.len().to_be_bytes()).await.unwrap();
    send.write_all(&message).await.unwrap();
}

fn message_for_client(client_id: i32, for_client: i32, message: Vec<u8>) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(1024 + message.len());
    let message = builder.create_vector(&message);
    let message = Message::create(
        &mut builder,
        &MessageArgs {
            client_id,
            message: Some(message),
            for_client,
            message_type: MessageType::ClientToClient,
        },
    );

    builder.finish(message, None);

    builder.finished_data().to_vec()
}

fn connect_message(client_id: i32) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(1024);
    let message = Message::create(
        &mut builder,
        &MessageArgs {
            client_id,
            message: None,
            for_client: 0,
            message_type: MessageType::ClientRegistration,
        },
    );

    builder.finish(message, None);

    builder.finished_data().to_vec()
}

/// from https://github.com/quinn-rs/quinn/blob/main/quinn/examples/insecure_connection.rs
/// Dummy certificate verifier that treats any certificate as valid.
/// NOTE, such verification is vulnerable to MITM attacks, but convenient for testing.
#[derive(Debug)]
struct SkipServerVerification(Arc<rustls::crypto::CryptoProvider>);

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self(Arc::new(rustls::crypto::ring::default_provider())))
    }
}

impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp: &[u8],
        _now: UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}
