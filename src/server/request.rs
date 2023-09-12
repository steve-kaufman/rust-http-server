use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Error, ErrorKind, Read},
    str::FromStr,
};

use super::{http_version::HttpVersion, method::Method};

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub raw_target: String,
    pub http_version: HttpVersion,
    pub headers: HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn from_stream(stream: &mut dyn Read) -> Result<Self, Error> {
        let mut buf_reader = BufReader::new(stream);

        let mut request_line = String::new();
        buf_reader.read_line(&mut request_line)?;
        request_line = request_line.trim().to_string();

        let mut request_line_parts = request_line.split(' ');

        let method_str = request_line_parts.next().unwrap();
        let target_str = request_line_parts.next().unwrap();
        let http_version_str = request_line_parts.next().unwrap();
        if let Some(word) = request_line_parts.next() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected fourth word word in request line: {}", word),
            ));
        }

        let method = match Method::from_str(method_str) {
            Ok(method) => method,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid method: {}", method_str),
                ))
            }
        };
        let raw_target = target_str.to_string();
        let http_version = match HttpVersion::from_str(http_version_str) {
            Ok(http_version) => http_version,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid http_version: {}", http_version_str),
                ))
            }
        };

        let mut headers: HashMap<String, Vec<String>> = HashMap::new();

        loop {
            let mut header_line = String::new();
            buf_reader.read_line(&mut header_line)?;
            header_line = header_line.trim().to_string();
            if header_line.is_empty() {
                break;
            }
            println!("Reading line: '{}'", header_line);
            if header_line.is_empty() {
                println!("Empty line; breaking");
                break;
            }

            let mut line_parts = header_line.split(':');
            let header_name = line_parts.next().unwrap().trim().to_string().to_lowercase();
            let header_values = line_parts.collect::<Vec<_>>().join("");
            let header_values = header_values.trim().to_string();

            let header_values = header_values.split(';');

            for value in header_values {
                let value = value.trim().to_string();
                if !headers.contains_key(&header_name) {
                    headers.insert(header_name.clone(), vec![]);
                }
                let values_vec = headers.get_mut(&header_name).unwrap();
                values_vec.push(value);
            }
        }

        println!("Collecting body");

        let mut content_length: usize = 0;

        if let Some(length_str) = headers.get("content-length") {
            content_length = usize::from_str(length_str.first().unwrap().as_str()).unwrap();
        }

        let mut body = vec![];
        let mut body_buf: [u8; 128] = [0; 128];

        while body.len() < content_length {
            buf_reader.read(&mut body_buf)?;
            body.extend_from_slice(&body_buf);
        }

        println!("Body: {}", String::from_utf8(body.clone()).unwrap());

        Ok(Request {
            method,
            raw_target,
            http_version,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_request() {
        let mut raw_request = "POST /foo HTTP/1.1\r\n\
            Host: localhost\r\n\
            x-my-header: foo; bar\r\n\
            x-my-header: baz\r\n\
            Content-Length: 44\r\n\
            \r\n\
            The quick brown fox jumped over the lazy dog\n\
        "
        .as_bytes();

        let parsed_request = Request::from_stream(&mut raw_request).unwrap();

        assert_eq!(Method::POST, parsed_request.method);
        assert_eq!("/foo", parsed_request.raw_target);
        assert_eq!(HttpVersion::Http1_1, parsed_request.http_version);

        assert_eq!(
            &vec!["localhost".to_string()],
            parsed_request.headers.get("host").unwrap()
        );

        assert_eq!(
            &vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
            parsed_request.headers.get("x-my-header").unwrap()
        );

        assert_eq!(
            "The quick brown fox jumped over the lazy dog\n"
                .as_bytes()
                .to_vec(),
            parsed_request
                .body
                .into_iter()
                .take_while(|b| *b != 0)
                .collect::<Vec<_>>()
        );
    }
}
