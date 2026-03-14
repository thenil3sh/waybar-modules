use std::{
    path::{Path, PathBuf}, sync::LazyLock, time::Duration
};

use clap::Parser;
use timer::{args::{ClientArgs, TimerArgs}, Timer};
use tokio::{
    fs, io,
    net::UnixDatagram,
    signal::unix::{self, SignalKind, signal},
    time::{self, Interval},
};

static SOCKET_PATH: &str = "/tmp/bar_doro.sock";
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

async fn handle_hangup() {
    let mut signal = signal(SignalKind::hangup()).unwrap();

    signal.recv().await;
    fs::remove_file(SOCKET_PATH).await.unwrap();
}
