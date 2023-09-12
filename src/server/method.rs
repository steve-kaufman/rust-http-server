use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    HEAD,
    OPTIONS,
    POST,
    PUT,
    PATCH,
    DELETE,
    CONNECT,
    TRACE,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(()),
        }
    }
}
