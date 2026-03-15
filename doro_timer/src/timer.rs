use std::io::{Error, ErrorKind};
use std::process::Command;

use crate::args::TimerArgs;
use crate::ticking::Ticking::{self, Break, None, Session};
use crate::visible::Visible;

#[derive(Debug)]
pub struct Timer {
    currently_showing: Visible,
    session_time: u64,
    break_time: u64,
    ticking: Ticking,
    paused: bool,

    on_session_over: Option<Command>,
    on_break_over: Option<Command>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            currently_showing: Visible::SessionTimer,
            session_time: 0,
            break_time: 0,
            ticking: Session,
            paused: true,
            on_session_over: Option::None,
            on_break_over: Option::None,
        }
    }

    pub fn is_running(&self) -> bool {
        !self.paused
    }

    /// Ticks the timer, and returns whether it actually ticked or not.
    ///
    /// - If Session is active - ticks the `session_timer`
    /// - If Break is active - ticks the `break_timer`
    /// - It returns `false` if ticking was not useful, that is when Session and Break, both are over
    pub async fn tick(&mut self) -> bool {
        match self.ticking {
            Session => self.tick_session().await,
            Break => self.tick_break().await,
            None => return false,
        }
        true
    }

    async fn tick_session(&mut self) {
        let timer = &mut self.session_time;
        if *timer == 0 {
            self.ticking = if self.break_time == 0 {
                Ticking::None
            } else {
                Ticking::Break
            };
            self.paused = true;
            self.currently_showing = if self.break_time != 0 {
                self.on_session_over();
                Visible::SessionOver
            } else {
                // No Break, we move back to idle
                self.on_break_over();
                Visible::BreakOver
            };
            return;
        }
        *timer -= 1;
    }

    async fn tick_break(&mut self) {
        let timer = &mut self.break_time;
        if *timer == 0 {
            self.ticking = if self.session_time == 0 {
                Ticking::None
            } else {
                Ticking::Session
            };
            self.paused = true;
            self.currently_showing = Visible::BreakOver;
            self.on_break_over();
            return;
        }
        *timer -= 1;
    }

    pub fn handle(&mut self, message: &str) {
        match message {
            "pl" => self.handle_play_pause(),
            "tg" => self.handle_toggle(),
            "rs" => self.handle_reset(),
            "in" => self.handle_increment(),
            "dc" => self.handle_decrement(),
            _ => {}
        }
    }

    // async fn session_is_over(&self) -> bool {
    //     self.ticking.is_break() && *self.session_time.lock().await == 0
    // }

    // async fn break_is_over(&self) -> bool {
    //     self.ticking.is_none() && *self.break_time.lock().await == 0
    // }

    pub fn on_session_over(&mut self) {
        if let Some(ref mut command) = self.on_session_over {
            let _ = command.spawn();
        };
    }

    pub fn on_break_over(&mut self) {
        if let Some(ref mut command) = self.on_break_over {
            let _ = command.spawn();
        }
    }
}

use Visible::*;
use tokio::io;
/// This one, handle events
impl Timer {
    fn handle_reset(&mut self) {
        match self.currently_showing {
            SessionTimer => self.session_time = 0,
            BreakTimer => self.break_time = 0,
            _ => {}
        }
    }

    fn handle_increment(&mut self) {
        match self.currently_showing {
            SessionTimer | BreakOver => {
                if self.session_time + 60 >= 3600 {
                    self.session_time = 3600;
                } else {
                    self.session_time += 60;
                }
                self.currently_showing = SessionTimer;
            }
            BreakTimer | SessionOver => {
                if self.break_time + 60 >= 3600 {
                    self.break_time = 3600;
                } else {
                    self.break_time += 60;
                }
                self.currently_showing = BreakTimer;
            }
        }
        if let None = self.ticking {
            self.ticking = Session;
        }
    }

    fn handle_decrement(&mut self) {
        match self.currently_showing {
            SessionTimer | SessionOver => {
                if self.session_time <= 60 {
                    self.session_time = 0;
                } else {
                    self.session_time -= 60;
                }
                self.currently_showing = SessionTimer;
            }
            BreakTimer | BreakOver => {
                if self.break_time <= 60 {
                    self.break_time = 0;
                } else {
                    self.break_time -= 60;
                }
                self.currently_showing = BreakTimer;
            }
        }
    }

    fn handle_play_pause(&mut self) {
        match self.ticking {
            Session if matches!(self.currently_showing, BreakOver | BreakTimer) => {
                self.currently_showing.toggle()
            }
            Break if matches!(self.currently_showing, SessionOver | SessionTimer) => {
                self.currently_showing.toggle()
            }
            None if self.session_time == 0 && self.break_time == 0 => {
                return;
            }
            _ => {}
        }
        self.paused = !self.paused;
    }

    fn handle_toggle(&mut self) {
        self.currently_showing.toggle();
    }
}

/// Waybar formatting implementation
impl Timer {
    fn text(&self) -> String {
        match self.currently_showing {
            Visible::SessionTimer => {
                let timer = self.session_time;
                format!("{:02}:{:02}", timer / 60, timer % 60)
            }
            Visible::SessionOver => "Start break?".to_owned(),
            Visible::BreakTimer => {
                let time = self.break_time;
                format!("{:02}:{:02}", time / 60, time % 60)
            }
            Visible::BreakOver => "".to_owned(),
        }
    }

    pub fn to_waybar_json(&self) -> String {
        let currently_showing = self.currently_showing.to_string();

        let mut classes = vec![currently_showing.as_str()];
        if !self.is_running() {
            classes.push("paused");
        }

        format!(
            "{{\"text\" : \"{}\", \"tooltip\" : \"{}\", \"class\" : {:?}, \"percentage\" : \"{}\", \"alt\" : \"{}\"}}",
            self.text(),
            "tooltip",
            classes,
            "0",
            self.currently_showing
        )
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_command<T>(command: T) -> io::Result<Command>
where
    T: AsRef<str>,
{
    let args = shell_words::split(command.as_ref()).map_err(|_| ErrorKind::InvalidData)?;
    let (program, args) = args.split_first().ok_or(ErrorKind::InvalidData)?;
    let mut command = Command::new(program);
    command.args(args);
    Ok(command)
}

impl TryFrom<&TimerArgs> for Timer {
    type Error = std::io::Error;
    fn try_from(args: &TimerArgs) -> Result<Self, std::io::Error> {
        Ok(Timer {
            on_break_over: args.on_break_over.as_ref().map(parse_command).transpose()?,
            on_session_over: args
                .on_session_over
                .as_ref()
                .map(parse_command)
                .transpose()?,
            ..Default::default()
        })
    }
}
