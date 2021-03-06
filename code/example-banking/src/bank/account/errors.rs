use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandError {
    AlreadyCreated,
    NotOpened,
}

impl error::Error for CommandError {
    fn description(&self) -> &str {
        match *self {
            CommandError::NotOpened => "attempt to execute command on account that is not opened",
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
    NotInitialized,
    NotOpened,
}

impl error::Error for EventError {
    fn description(&self) -> &str {
        match *self {
            EventError::NotInitialized => "attempt to execute event before creation",
            EventError::AlreadyOpened => "attempt to open when already opened",
            EventError::NotOpened => "attempt to closed when not opened",
        }
    }
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err: &error::Error = self;
        f.write_str(err.description())
    }
}
