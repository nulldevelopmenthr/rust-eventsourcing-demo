use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandError {
    AlreadyCreated,
}

impl error::Error for CommandError {
    fn description(&self) -> &str {
        match *self {
            CommandError::AlreadyCreated => "attempt to create when already created",
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err: &error::Error = self;
        f.write_str(err.description())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventError {
    AlreadyOpened,
}

impl error::Error for EventError {
    fn description(&self) -> &str {
        match *self {
            EventError::AlreadyOpened => "attempt to open when already opened",
        }
    }
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err: &error::Error = self;
        f.write_str(err.description())
    }
}
