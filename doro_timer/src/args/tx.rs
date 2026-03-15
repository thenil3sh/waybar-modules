#[derive(clap::Parser, Debug)]
pub struct ClientArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[repr(u8)]
#[derive(clap::Subcommand, Debug)]
pub enum Command {
    #[command(about = "Add minutes to timer (\x1b[1m1 minute\x1b[0m is added by default)")]
    Increase,
    #[command(about = "Decreases timer minute by \x1b[1m1 (or n\x1b[0m if specified)")]
    Decrease,
    /// Toggles the view between Break & Session timer
    Toggle,
    /// Starts/Pauses/Continues timer tick
    PlayPause,
    /// Sets timer to 00:00
    Reset,
}

use std::fmt::{Display, Result};
impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Increase => "in",
                Self::Decrease => "dc",
                Self::Toggle => "tg",
                Self::PlayPause => "pl",
                Self::Reset => "rs"
            }
        )
    }
}
