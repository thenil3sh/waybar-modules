use std::fmt::Display;

use Visible::{BreakOver, BreakTimer, SessionOver, SessionTimer};

use crate::visible;

#[derive(Debug)]
pub enum Visible {
    BreakTimer,
    BreakOver,
    SessionTimer,
    SessionOver,
}

impl Visible {
    pub fn toggle(&mut self) {
        *self = match self {
            BreakTimer | BreakOver => SessionTimer,
            SessionTimer | SessionOver => BreakTimer,
        }
    }
}

impl Display for Visible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SessionTimer => "session-timer",
                SessionOver => "session-over",
                BreakTimer => "break-timer",
                BreakOver => "break-over",
            }
        )
    }
}
