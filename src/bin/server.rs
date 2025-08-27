use quinn::{Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::time::Duration;

use tracing::info;

use debra::config::ServerConfig as Config;
use debra::router::Router;

/// based on quinn example https://github.com/quinn-rs/quinn/blob/main/quinn/examples/common/mod.rs
///
/// Constructs a QUIC endpoint configured to listen for incoming connections on a certain address
/// and port.
///
/// ## Returns
///
/// - a stream of incoming QUIC connections
/// - server certificate serialized into DER format
pub fn make_server_endpoint(
    bind_addr: SocketAddr,
) -> Result<(Endpoint, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let (server_config, server_cert) = configure_server()?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
}

/// based on quinn example https://github.com/quinn-rs/quinn/blob/main/quinn/examples/common/mod.rs
/// Returns default server configuration along with its certificate.
fn configure_server(
) -> Result<(ServerConfig, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert);
    let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish(),
    )
    .unwrap();

    let shutdown = tokio_graceful::Shutdown::default();

    shutdown.spawn_task_fn(run_server);
    match shutdown.shutdown_with_limit(Duration::from_secs(10)).await {
        Ok(elapsed) => {
            tracing::info!(
                "shutdown: gracefully {}s after shutdown signal received",
                elapsed.as_secs_f64()
            );
        }
        Err(e) => {
            tracing::warn!("shutdown: forcefully due to timeout: {}", e);
        }
    }
}

async fn run_server(shutdown_guard: tokio_graceful::ShutdownGuard) {
    let config = Config::get();

    let (endpoint, _cert) = make_server_endpoint(config.listen).unwrap();
    info!("listening on {}", endpoint.local_addr().unwrap());
    let router = Arc::new(Router::default());

    let router_task = router.clone();

    tokio::spawn(async move {
        router_task.route().await;
    });

    loop {
        let shutdown_guard = shutdown_guard.clone();
        tokio::select! {
            _ = shutdown_guard.cancelled() => {
                router.close().await;
                break;
            }
            conn = endpoint.accept() => {
                match conn {
                    Some(conn) => {
                        info!("new connection");
                        let router = router.clone();
                        tokio::spawn(async move {
                            let res = router.new_connection(conn).await;
                            info!("new_connection result: {:?}", res);
                        });

                    }
                    _ => break
                }
            }
        }
    }
}
