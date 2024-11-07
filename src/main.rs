pub mod message;

use anyhow::Result;
use message::Message;
use tokio::io::{stdin, AsyncWriteExt, BufReader};
use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

/*
    nodes & client are sequentially numbered (n1, n2, ...), nodes "n" and clients "c"

 */

 #[tokio::main]
 async fn main() -> Result<()> {
    
    let mut stdin_stream = LinesStream::new(BufReader::new(stdin()).lines());
    let mut stdout = tokio::io::stdout();

    while let Some(msg_res) = stdin_stream.next().await {
        let mut msg = serde_json::from_str::<Message>(msg_res?.as_str())?;
        
        std::mem::swap(&mut msg.dest, &mut msg.src);
        
        msg.body.type_field = "echo_ok".into();
        msg.body.in_reply_to = Some(msg.body.msg_id.unwrap_or(0));

        let response = serde_json::to_vec(&msg)?;

        stdout.write_all(&response).await?;
        stdout.flush().await?;
    }
    
    Ok(())
 }
