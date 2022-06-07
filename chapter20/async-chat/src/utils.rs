use std::error::Error;

// TODO anyhowの利用して実装してみる
pub type ChatError = Box<dyn Error+Send + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;


use async_std::prelude::*;
use serde::Serialize;
use std::marker::Unpin;

 
pub async fn send_as_json<S,P>(outbound: &mut S, packet: &P)->ChatResult<()>
where 
    S: async_std::io::Write+ Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}