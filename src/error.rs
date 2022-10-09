use std::fmt;

#[derive(Debug)]
pub enum Error {
    Value(String),
    Format(fmt::Error),
    Reqwest(reqwest::Error),
    Yamaha(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Value(s) => write!(f, "Invalid value: {}", s),
            Self::Reqwest(e) => e.fmt(f),
            Self::Format(e)  => e.fmt(f),
            Self::Yamaha(status_code) => write!(f, "Yamaha returned an error: {}", status_code),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}
