use reqwest;
use serde_json;

use std::fmt;
use std::error;
use CommandErrors;



#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Serialize(serde_json::Error),
    ApiError(CommandErrors),
}


impl From<reqwest::Error> for Error {
    fn from(e : reqwest::Error) -> Error {
        Error::Request(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e : serde_json::Error) -> Error {
        Error::Serialize(e)
    }
}

impl From<CommandErrors> for Error {
    fn from(e : CommandErrors) -> Error {
        Error::ApiError(e)
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Request(ref e) => write!(f, "{}", e),
            &Error::Serialize(ref e) => write!(f, "{}", e),
            &Error::ApiError(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &'static str {
        match self {
            &Error::Request(_) => "request failed",
            &Error::Serialize(_) => "serialization failed",
            &Error::ApiError(_) => "api error",

        }
    }
}