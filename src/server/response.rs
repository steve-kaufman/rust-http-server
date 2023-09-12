use std::{
    collections::HashMap,
    io::{Error, Write},
};

use super::{http_version::HttpVersion, status_code::StatusCode};

pub struct Response {
    pub http_version: HttpVersion,
    pub status_code: StatusCode,
    pub headers: HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        return Response {
            http_version: HttpVersion::Http1_1,
            status_code: StatusCode::OK,
            headers: HashMap::new(),
            body: vec![],
        };
    }

    pub fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let http_version_str = match self.http_version {
            HttpVersion::Http1_0 => "HTTP/1.0",
            HttpVersion::Http1_1 => "HTTP/1.1",
            HttpVersion::Http2_0 => "HTTP/2.0",
        };

        let status_code = self.status_code.0;
        let reason_phrase = self.status_code.1;

        let status_line = format!("{} {} {}\r\n", http_version_str, status_code, reason_phrase);

        let headers_lines = self
            .headers
            .iter()
            .map(|(header_name, header_values)| {
                format!("{}: {}", header_name, header_values.join(";"))
            })
            .collect::<Vec<_>>()
            .join("\r\n")
            + "\r\n";

        stream.write_all(status_line.as_bytes())?;
        stream.write_all(headers_lines.as_bytes())?;
        stream.write_all("\r\n".as_bytes())?;
        stream.write_all(&self.body)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_header_line(line: &String, fields: &mut HashMap<String, Vec<String>>) {
        let line_parts = line.split(":").map(|s| s.trim()).collect::<Vec<_>>();
        let (header_name, header_values) = match line_parts.as_slice() {
            [first, second] => (first, second),
            _ => panic!("Unexpected number of parts in header line: '{}'", line),
        };

        fields.insert(
            header_name.to_string(),
            header_values
                .split(";")
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn test() {
        let mut response = Response::new();
        response.status_code = StatusCode::INTERNAL_SERVER_ERROR;
        response.body = "The quick brown fox jumped over the lazy dog.\nabcdefghijklmnopqrstuvwxyz\nABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes().to_vec();
        response
            .headers
            .insert("Accept".to_string(), vec!["*/*".to_string()]);
        response.headers.insert(
            "Access-Control-Allow-Origins".to_string(),
            vec!["localhost".to_string(), "*.example.com".to_string()],
        );

        let mut output = vec![];

        response.write_to(&mut output).expect("Should not fail");

        let output = String::from_utf8(output).unwrap();
        let mut lines = output.lines();

        assert_eq!("HTTP/1.1 500 Internal Server Error", lines.next().unwrap());

        let mut fields = HashMap::new();
        let header_line1 = lines.next().unwrap();
        let header_line2 = lines.next().unwrap();

        parse_header_line(&header_line1.to_string(), &mut fields);
        parse_header_line(&header_line2.to_string(), &mut fields);

        assert_eq!(
            &vec!["*/*".to_string()],
            fields.get(&"Accept".to_string()).unwrap()
        );
        assert_eq!(
            &vec!["localhost", "*.example.com"],
            fields
                .get(&"Access-Control-Allow-Origins".to_string())
                .unwrap()
        );

        let blank_line = lines.next().unwrap();
        assert!(blank_line.is_empty());

        let body = lines.collect::<Vec<_>>().join("\n");

        assert_eq!(
            "The quick brown fox jumped over the lazy dog.\n\
            abcdefghijklmnopqrstuvwxyz\n\
            ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            body
        );
    }
}
