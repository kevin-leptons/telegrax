use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    description: String,
}

#[derive(Debug)]
pub enum ErrorCode {
    /// An error is returned by API endpoint. It could be server, client or
    /// transport layer error.
    Endpoint,
}
#[derive(Deserialize)]
struct HttpApiResponseBody {
    description: String,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code={}; description={}", self.code, self.description)
    }
}

impl Error {
    pub(crate) fn from_http_api_error(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Status(status, response) => {
                let body: HttpApiResponseBody = match response.into_json() {
                    Ok(v) => v,
                    Err(e) => {
                        return Self {
                            code: ErrorCode::Endpoint,
                            description: format!("Cannot parse API response. {}", e.to_string()),
                        }
                    }
                };
                return Self {
                    code: ErrorCode::Endpoint,
                    description: format!("status={}; description={}", status, body.description),
                };
            }
            ureq::Error::Transport(transport) => Self {
                code: ErrorCode::Endpoint,
                description: transport.to_string(),
            },
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Endpoint => write!(f, "Endpoint"),
        }
    }
}
