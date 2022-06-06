use async_std::prelude::*;
use async_chat::utils::{self,ChatResult};
use async_std::io;
use async_std::net;

async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()>{
    
    println!("Commands:\n\
            join GROUP\n\
            post GROUP\n\
            Type Control-D (on Unix) or Control-Z (on Windows) \
            to close the connection.");
    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_result) = command_lines.next().await {

    }
    Ok(())
}

fn main(){
    println!("Hello world");
}