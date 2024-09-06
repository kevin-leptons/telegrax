use std::{
    fmt::Display,
    fs::{self, File},
    io,
    os::unix::prelude::PermissionsExt,
    path::PathBuf,
    str::FromStr,
};

use libc::{S_IROTH, S_IRWXO, S_IWOTH, S_IXOTH};
use regex::Regex;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use serde_json::error::Category;
use telegrax::bot::Bot;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotFound,
    PermissionInsecured(u32),
    PermissionDenied,
    Syntax(String),
    FieldMissing(String),
    FieldUnknown(String),
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy)]
pub enum Mode {
    Validate,
    Trade,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Validate => write!(f, "validate"),
            Mode::Trade => write!(f, "trade"),
        }
    }
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ModeVisitor)
    }
}

struct ModeVisitor;

impl<'de> Visitor<'de> for ModeVisitor {
    type Value = Mode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A mode such as 'validate' or 'trade'")
    }

    fn visit_str<E>(self, v: &str) -> std::prelude::v1::Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Mode::from_str(v) {
            None => Err(de::Error::custom("Invalid mode as a string")),
            Some(v) => Ok(v),
        }
    }
}

impl Mode {
    pub fn from_str(value: &str) -> Option<Mode> {
        match value {
            "validate" => Some(Mode::Validate),
            "trade" => Some(Mode::Trade),
            _ => None,
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    telegram_bot_token: String,
    telegram_chat_identity: String,
}

impl Configuration {
    pub fn load() -> Result<Self> {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let config_file = PathBuf::from_str(&crate_path)
            .unwrap()
            .join("config.test.json");
        let data = Self::read_secured_file(&config_file)?;
        Self::parse_json(data)
    }

    fn read_secured_file(path: &PathBuf) -> Result<String> {
        Self::assert_secured_file(path)?;
        match fs::read_to_string(path) {
            Ok(v) => return Ok(v),
            Err(e) => return io_error_to_program_result(e, path),
        }
    }

    fn parse_json(value: String) -> Result<Configuration> {
        let deserializer = &mut serde_json::Deserializer::from_str(&value);
        match serde_path_to_error::deserialize::<_, Configuration>(deserializer) {
            Ok(v) => Ok(v),
            Err(e) => Self::translate_parsing_error(e),
        }
    }

    fn translate_parsing_error<T>(e: serde_path_to_error::Error<serde_json::Error>) -> Result<T> {
        match e.inner().classify() {
            Category::Syntax | Category::Eof => {
                return Err(Error::Syntax(e.to_string()));
            }
            _ => {}
        }
        match Self::get_missing_field(&e.inner()) {
            None => {}
            Some(v) => return Err(Error::FieldMissing(v)),
        };
        match Self::get_unknown_field(&e.inner()) {
            None => {}
            Some(v) => return Err(Error::FieldUnknown(v)),
        }
        return Err(Error::Other(e.to_string()));
    }

    fn get_missing_field(e: &serde_json::Error) -> Option<String> {
        let regex = Regex::new(r"^missing field `(.+)` at line \d+ column \d+$").unwrap();
        let message = e.to_string();
        let matches: Vec<[&str; 1]> = regex
            .captures_iter(message.as_str())
            .map(|c| c.extract().1)
            .collect();
        if matches.len() != 1 {
            return None;
        }
        return Some(matches[0][0].to_string());
    }

    fn get_unknown_field(e: &serde_json::Error) -> Option<String> {
        let regex =
            Regex::new(r"^unknown field `(.+)`, expected one of .* at line \d+ column \d+$")
                .unwrap();
        let message = e.to_string();
        let matches: Vec<[&str; 1]> = regex
            .captures_iter(message.as_str())
            .map(|c| c.extract().1)
            .collect();
        if matches.len() != 1 {
            return None;
        }
        return Some(matches[0][0].to_string());
    }

    fn assert_secured_file(path: &PathBuf) -> Result<()> {
        let file = match File::open(path) {
            Ok(v) => v,
            Err(e) => return io_error_to_program_result(e, path),
        };
        let metadata = match file.metadata() {
            Ok(v) => v,
            Err(e) => return io_error_to_program_result(e, path),
        };
        let mode = metadata.permissions().mode();
        if (mode & S_IRWXO) > 0
            || (mode & S_IROTH) > 0
            || (mode & S_IWOTH) > 0
            || (mode & S_IXOTH) > 0
        {
            return Err(Error::PermissionInsecured(mode));
        }
        return Ok(());
    }

    pub fn telegram_bot_token(&self) -> &str {
        &self.telegram_bot_token
    }

    pub fn telegram_chat_identity(&self) -> &str {
        &self.telegram_chat_identity
    }
}

fn io_error_to_program_result<T>(io_error: io::Error, _path: &PathBuf) -> Result<T> {
    match io_error.kind() {
        io::ErrorKind::NotFound => {
            return Err(Error::NotFound);
        }
        io::ErrorKind::PermissionDenied => return Err(Error::PermissionDenied),
        _ => return Err(Error::Other(io_error.to_string())),
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => {
                write!(f, "NotFound")
            }
            Error::PermissionDenied => write!(f, "PermissionDenied"),
            Error::PermissionInsecured(mode) => {
                write!(f, "PermissionInsecured 0o{:o}", mode)
            }
            Error::Syntax(v) => write!(f, "Syntax: {}", v),
            Error::FieldMissing(v) => write!(f, "FieldMissing: {}", v),
            Error::FieldUnknown(v) => write!(f, "FieldUnknown: {}", v),
            Error::Other(v) => write!(f, "Other: {}", v),
        }
    }
}

pub fn create_services() -> (Configuration, Bot) {
    let config = Configuration::load().unwrap();
    let bot_config = telegrax::bot::Configuration {
        token: config.telegram_bot_token().to_string(),
    };
    let bot = Bot::new(bot_config);
    (config, bot)
}
