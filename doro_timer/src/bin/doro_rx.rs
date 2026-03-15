use std::{
    sync::LazyLock, time::Duration
};


use clap::Parser;
use doro_timer::{SOCKET_PATH, Timer, args::TimerArgs};
use tokio::{
    fs, io,
    net::UnixDatagram,
    signal::unix::{SignalKind, signal},
    time::{self},
};

static ARGS : LazyLock<TimerArgs> = LazyLock::new(|| TimerArgs::parse());

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    
    let _ = fs::remove_file(SOCKET_PATH).await;
    let listener = UnixDatagram::bind(SOCKET_PATH)?;

    let mut ticker = time::interval(Duration::from_secs(1));
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    let mut buffer = [0u8; 4];

    let mut timer = Timer::try_from(&*ARGS)?;

    // Print whenever a HUP signal is received
    loop {
        println!("{}", timer.to_waybar_json());
        tokio::select! {
            _ = ticker.tick(), if timer.is_running() => {
                timer.tick().await;
            },
            Ok(len) = listener.recv(&mut buffer) => {
                if let Ok(message) = str::from_utf8(&buffer[..len]) {
                    timer.handle(message);
                }
            }
        }
    }
}

