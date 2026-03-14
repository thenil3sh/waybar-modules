use std::{
    io::{Error, ErrorKind},
    sync::LazyLock,
};

use clap::Parser;
use timer::{SOCKET_PATH, args::ClientArgs};
use tokio::{io, net::UnixDatagram};

static ARGS : LazyLock<ClientArgs> = LazyLock::new(|| ClientArgs::parse());

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {

    let socket = UnixDatagram::unbound()?;

    if let Err(x) = socket
        .send_to(ARGS.command.to_string().as_bytes(), SOCKET_PATH)
        .await
        && let Some(code) = x.raw_os_error()
    {
        eprintln!("Couldn't connect doro_rx. Is it running? : \x1b[31m{x}\x1b[0m");
        std::process::exit(code);
    }
    Ok(())
}
