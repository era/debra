use anyhow::{anyhow, Result};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::common::read_bytes;
use crate::flatbuffers::{root_as_message, MessageType};

/// Letter is the Message Router exchange between itself
/// to perform actions, such as forwarding messages to clients
/// but also removing clients from its map.
pub enum Letter {
    // forward the message to i32 id
    Forward(i32, Vec<u8>),
    // remove the i32 id from our clients map
    Remove(i32),
}

/// Router is the main responsible for handling connections and 
/// requests from clients. Each new connection is kept in a new Task
/// and all messages from clients are forward to a Tokio channel.
/// From another Task, Router can decide what to do with the message.
pub struct Router {
    clients: clashmap::ClashMap<i32, Arc<Mutex<quinn::SendStream>>>,
    rx: Mutex<mpsc::Receiver<Letter>>,
    tx: mpsc::Sender<Letter>,
}

impl Default for Router {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel(32);
        Self {
            clients: clashmap::ClashMap::new(),
            rx: Mutex::new(rx),
            tx,
        }
    }
}

impl Router {
    /// handles a new connection, it assumes the first message will be a simple ClientRegistration
    /// and only reads the `client_id` to setup the env to wait for messages from it.
    ///
    /// It spawns a new task that will read from the incomming stream. It will also
    /// add the outcomming stream to a clients hashmap to be used later when forwarding
    /// messages to that client_id.
    pub async fn new_connection(&self, conn: quinn::Incoming) -> Result<()> {
        let connection = conn.await?;

        // for now let's just assume we will have a single stream per client
        let stream = connection.accept_bi().await;
        let (sender, mut receiver) = match stream {
            Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                info!("connection closed");
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
            Ok(s) => s,
        };


        let bytes = read_bytes(&mut receiver).await?;

        let message =
            root_as_message(&bytes).map_err(|e| anyhow!("failed parsing message: {:?}", e))?;

        if message.message_type() != MessageType::ClientRegistration {
            error!("not expecting message type: {:?}", message.message_type());
            return Err(anyhow!("First Message should be client registration"));
        }

        let id = message.client_id();

        info!("registring client: {id}");

        // we just assume something else is setting the ids for the clients
        // and we can trust the clients
        self.clients
            .insert(message.client_id(), Arc::new(Mutex::new(sender)));

        let sender = self.tx.clone();

        tokio::spawn(async move {
            // this will keep reading messages from the stream
            // until an error happens.
            loop {
                match read_bytes(&mut receiver).await {
                    Ok(bytes) => {
                        info!("new message from client: {id}");
                        let letter_to = root_as_message(&bytes)
                            .map_err(|e| error!("failed parsing message: {:?}", e))
                            .ok()
                            .map(|m| m.for_client());

                        if let Some(letter_to) = letter_to {
                            // ignore the error for now
                            let _ = sender.send(Letter::Forward(letter_to, bytes)).await;
                        } else {
                            info!("message for nobody");
                        }
                    }
                    Err(e) => {
                        error!("error while reading message: {:?}", e);
                        // ignore the error for now
                        let _ = sender.send(Letter::Remove(id)).await;
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    /// keep processing messages, this should be started in a Task and used only once.
    /// If you call multiple times, all tasks besides the first will be blocked waiting
    /// the Mutex.
    pub async fn route(&self) {
        // no point in any other task ever to access this lock, so we grab
        // it forever
        let mut rx = self.rx.lock().await;
        while let Some(letter) = rx.recv().await {
            self.handle_letters(letter).await;
        }
    }

    /// handle internal messages, which could be messages to be forward or internal maintanence
    /// messages (such as removing a client when it disconnects).
    async fn handle_letters(&self, letter: Letter) -> Option<()> {
        match letter {
            Letter::Remove(id) => {
                self.clients.remove(&id);
            }
            Letter::Forward(letter_to, bytes) => {
                // clones to avoid deadlock due to clashmap
                let stream = self.clients.get(&letter_to)?.clone();
                // spawns a new task to avoid blocking our main loop
                tokio::spawn(async move {
                    let mut stream = stream.lock().await;
                    // first bytes are always the size
                    let _ = stream
                        .write_all(&bytes.len().to_be_bytes())
                        .await
                        .map_err(|e| error!("error while sending message {:?}", e));

                    let _ = stream
                        .write_all(&bytes)
                        .await
                        .map_err(|e| error!("error while sending message {:?}", e));

                    info!("finished forwarding message");
                });
            }
        }

        Some(())
    }
}
