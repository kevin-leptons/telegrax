//! The result of a function call. It may be success or failure.

use std::fmt::Display;

use serde::Deserialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Cannot request to or read data from the endpoint. It could be a network
    /// issue, SSL certificate setup, and more.
    EndpointConnection,

    /// The endpoint returns data that does not follow specification.
    EndpointResponse,

    /// The endpoint reports an error. It could be invalid data from the client
    /// or an internal endpoint error. Unfortunately, there are no official
    /// error cases. Telegram provides a short description for error handling
    /// [here](https://core.telegram.org/bots/api#making-requests).
    ///
    /// This variant makes the error not as small as the CPU word. Fortunately,
    /// it will not affect the execution time much because there are rate limits
    /// for calling the endpoint, see module [crate::limit].
    Endpoint { description: String },
}

#[derive(Deserialize)]
struct EndpointResponseBodyError {
    description: String,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndpointConnection => write!(f, "EndpointConnection"),
            Self::EndpointResponse => write!(f, "EndpointResponse"),
            Self::Endpoint { description } => write!(f, "Endpoint: {description}"),
        }
    }
}

impl Error {
    pub(crate) fn from_http_api_error(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Status(_, response) => {
                let body: EndpointResponseBodyError = match response.into_json() {
                    Ok(v) => v,
                    Err(_) => return Self::EndpointResponse,
                };
                Self::Endpoint {
                    description: body.description,
                }
            }
            ureq::Error::Transport(_) => Self::EndpointConnection,
        }
    }
}
