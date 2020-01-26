use std::error;
use std::fmt;

pub struct EmulateCycleError {
    pub message: String,
}

impl fmt::Display for EmulateCycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for EmulateCycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EmulateCycleError {{ message: {} }}",
            self.message
        )
    }
}

impl error::Error for EmulateCycleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
