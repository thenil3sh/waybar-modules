pub mod ticking;
pub mod timer;
pub mod visible;
pub mod args;

pub use timer::Timer;

pub static SOCKET_PATH: &str = "/tmp/bar_doro.sock";
