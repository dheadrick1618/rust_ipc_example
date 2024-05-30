# Rust IPC example using unix named pipes (FIFOS)

Using only tokio and the nix crates, these two programs serve as a lightweight example of how to implement interprocess communication (IPC) using Rust.

## Usage

1. Run the reader program first using

```@sh
    cargo run
```

2. Run the writer program second, again using