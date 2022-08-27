use std::fmt::Display;


#[derive(Debug)]
pub enum Error {
    CommunicatorError(String),
    UknownError,
    UnexpectedStatusCode(u16, u16, String),
    NetworkError(String),
    SerdeError
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CommunicatorError(message) => write!(f, "Error of type Communicator Error. Message: {}", message),
            Error::UknownError => write!(f, "Error of type Uknown Error."),
            Error::UnexpectedStatusCode(expected, actual, message) => write!(f, "Error of type UnexpectedStatusCode. Expected: {}, Actual: {}, Message: {}", expected, actual, message),
            Error::NetworkError(message) => write!(f, "Error of type Network Error. Message: {}", message),
            Error::SerdeError => write!(f, "Error of type Serde Error."),
        }
    }
}