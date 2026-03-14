use tokio::io;

#[derive(clap::Parser, Debug)]
pub struct TimerArgs {
    /// Executes the command when session is over
    #[arg(long)]
    pub on_session_over : Option<String>,
    
    /// Executes the command when break is over
    #[arg(long)]
    pub on_break_over : Option<String>,
}