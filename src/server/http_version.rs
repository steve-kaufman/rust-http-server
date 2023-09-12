use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    Http1_0,
    Http1_1,
    Http2_0,
}

impl FromStr for HttpVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(Self::Http1_0),
            "HTTP/1.1" => Ok(Self::Http1_1),
            "HTTP/2.0" => Ok(Self::Http2_0),
            _ => Err(()),
        }
    }
}
