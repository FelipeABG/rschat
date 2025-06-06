<h1 align = "center"> Rschat </h1>

<p align="center">Real time multi-user chat backend and frontend (TUI) written in rust.</p>

<p align="center">
    <img src="./resources/chat.png">
</p>

## Overview

Rschat provides a complete chat system consisting of 2 parts:

- Server: Handles multiple concurrent connections and message routing.
- Client: Terminal user interface to display and send messages.

## Prerequisites

- Rust.
- Cargo package manager.

## Installation

To install Rschat just clone the repository and compile it using cargo:

- Cloning the repository:

```
git clone https://github.com/FelipeABG/rschat.git
```

- Compiling:

```
cargo build --release
```

## Usage

After installing it you can run the modules following the instructions:

### Backend

Executing the server with default options will try to bind to `localhost:8080`:

```
cargo run -p server
```

Or you can provide a port for the server to bind with:

```
cargo run -p server -- -a <server-address>
```

### Frontend

To execute the client you have to provide a name that will be used as your username in the session:

```rust
cargo run -p client -- -u <yourname>
// This will try to connect the client to localhost:8000
```

Or you can provide a prot for the client to connect:

```
cargo run -p client -- -u <yourname> -a <server-address>
```

**Binaries**:
You can also just execute the binaries for both parts with the required arguments.
