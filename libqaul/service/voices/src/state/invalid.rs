use std::fmt::{Display, Formatter, Result};
use failure::Fail;

/// The method called is invalid for the current state
#[derive(Debug)]
pub(crate) struct InvalidState(String);

impl InvalidState {
    pub(crate) fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl Fail for InvalidState {}

impl Display for InvalidState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "The requested operation is invalid with the call in state {}",
            self.0
        )
    }
}

/// The state machine tried to move between states in an invalid way
#[derive(Debug)]
pub(crate) struct InvalidStateTransition {
    from: String,
    to: String,
}

impl InvalidStateTransition {
    pub(crate) fn new<A: Into<String>, B: Into<String>>(from: A, to: B) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

impl Fail for InvalidStateTransition {}

impl Display for InvalidStateTransition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Tried to transistion from call state {} to {}",
            self.from, self.to
        )
    }
}
