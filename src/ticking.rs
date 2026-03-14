#[derive(PartialEq, Debug)]
pub enum Ticking {
    Session,
    Break,
    None,
}

use std::fmt::Display;

use Ticking::{Break, None, Session};
impl Ticking {
    pub(crate) fn is_session(&self) -> bool {
        *self == Session
    }

    pub(crate) fn is_break(&self) -> bool {
        *self == Break
    }

    pub(crate) fn is_none(&self) -> bool {
        *self == None
    }
}
