use reqwest;
use rpser;
use chrono;
use std::{error as stderror, fmt, io, num};

#[derive(Debug)]
pub enum Error {
    TooManyRecords,
    FnsError(String),
    ReqError(reqwest::Error),
    RpcError(rpser::RpcError),
    XmlError(rpser::xml::Error),
    ParseIntError(num::ParseIntError),
    ParseDateTimeError(chrono::ParseError),
    IoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TooManyRecords => write!(
                f,
                "В запросе не может быть больше 10000 элементов"
            ),
            Error::FnsError(ref err_msg) => write!(f, "{}", err_msg),
            Error::ReqError(ref e) => fmt::Display::fmt(e, f),
            Error::RpcError(ref e) => fmt::Display::fmt(e, f),
            Error::XmlError(ref e) => fmt::Display::fmt(e, f),
            Error::ParseIntError(ref e) => fmt::Display::fmt(e, f),
            Error::ParseDateTimeError(ref e) => fmt::Display::fmt(e, f),
            Error::IoError(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl stderror::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::TooManyRecords => {
                "В запросе не может быть больше 10000 элементов"
            }
            Error::FnsError(_) => {
                "Сервис сообщил об ошибке обработки запроса"
            }
            Error::ReqError(ref e) => e.description(),
            Error::RpcError(ref e) => e.description(),
            Error::XmlError(ref e) => e.description(),
            Error::ParseIntError(ref e) => e.description(),
            Error::ParseDateTimeError(ref e) => e.description(),
            Error::IoError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&stderror::Error> {
        match *self {
            Error::TooManyRecords => None,
            Error::FnsError(_) => None,
            Error::ReqError(ref e) => e.cause(),
            Error::RpcError(ref e) => e.cause(),
            Error::XmlError(ref e) => e.cause(),
            Error::ParseIntError(ref e) => e.cause(),
            Error::ParseDateTimeError(ref e) => e.cause(),
            Error::IoError(ref e) => e.cause(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(other: reqwest::Error) -> Error {
        Error::ReqError(other)
    }
}

impl From<rpser::RpcError> for Error {
    fn from(other: rpser::RpcError) -> Error {
        Error::RpcError(other)
    }
}

impl From<rpser::xml::Error> for Error {
    fn from(other: rpser::xml::Error) -> Error {
        Error::XmlError(other)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(other: num::ParseIntError) -> Error {
        Error::ParseIntError(other)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(other: chrono::ParseError) -> Error {
        Error::ParseDateTimeError(other)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Error {
        Error::IoError(other)
    }
}
