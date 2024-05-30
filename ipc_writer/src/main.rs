/*

Open a FIFO for writing data to, and write a message to it.

NOTE! -> This assumed a reader process has already opened a FIFO for reading.
*/

use tokio::fs::OpenOptions;
use tokio::io::{self, AsyncWriteExt};


const PIPE_PATH: &str = "/tmp/rust_ipc_fifo";

#[tokio::main]
async fn main() -> io::Result<()> {

    // Open the (already created) named pipe for writing
    let mut fifo = OpenOptions::new()
        .write(true)
        .open(PIPE_PATH)
        .await?;

    let message = b"Hello from the writer process!";
    fifo.write_all(message).await?;
    println!("Message written to the pipe");

    Ok(())
}
