use anyhow::{anyhow, Result};

pub async fn read_bytes(receiver: &mut quinn::RecvStream) -> Result<Vec<u8>> {
    let mut size = [0u8; 8];

    receiver
        .read_exact(&mut size)
        .await
        .map_err(|e| anyhow!("failed reading request: {}", e))?;

    let size = usize::from_be_bytes(size);

    let mut bytes: Vec<u8> = Vec::with_capacity(size);
    bytes.resize(size, 0);

    receiver
        .read_exact(&mut bytes)
        .await
        .map_err(|e| anyhow!("failed reading request: {}", e))?;

    Ok(bytes)
}
