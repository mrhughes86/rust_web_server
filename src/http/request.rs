use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FtmResult, Display, Formatter, Debug};


pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}


impl TryFrom<&[u8]> for Request { // Byte conversion error handling.
    type Error = ParseError;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{unimplemented!()}
}



pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
// FMT display error
impl Dispay for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FtmResult{
        write!(f, "{}" , self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FtmResult{
        write!(f, "{}" , self.message())
    }
}
