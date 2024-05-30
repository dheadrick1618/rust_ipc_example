/*

Open a FIFO for reading data from.

*/

use tokio::fs::OpenOptions;
use tokio::io::{self, AsyncReadExt};
use std::path::Path;
use std::fs::remove_file;
use nix::unistd::mkfifo;

const PIPE_PATH: &str = "/tmp/rust_ipc_fifo";

#[tokio::main]
async fn main() -> io::Result<()> {
    // Clean up any existing named pipe
    if Path::new(PIPE_PATH).exists() {
        remove_file(PIPE_PATH)?;
    }

    mkfifo(PIPE_PATH, nix::sys::stat::Mode::S_IRWXU).expect("Failed to create FIFO");

    // Open the named pipe for reading
    let mut fifo = OpenOptions::new()
        .read(true)
        .open(PIPE_PATH)
        .await?;

    let mut buffer = vec![0; 1024];

    // Loop indefinitely, reading data from the FIFO 
    loop {
        let n = fifo.read(&mut buffer).await?;
        if n == 0 {
            continue;
        }
        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    }
}
