use clap::Parser;

use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[clap(name = "server")]
pub struct ServerConfig {
    #[clap(default_value = "127.0.0.1:4433")]
    pub listen: SocketAddr,
}

impl ServerConfig {
    //TODO get is a bad name
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Parser, Debug)]
#[clap(name = "client")]
pub struct ClientConfig {
    #[clap(long)]
    pub client_id: i32,
    #[clap(long, default_value = "127.0.0.1:4433")]
    pub listen: SocketAddr,
}

impl ClientConfig {
    //TODO get is a bad name
    pub fn get() -> Self {
        Self::parse()
    }
}
