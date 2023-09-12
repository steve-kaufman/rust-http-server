/** StatusCode (u16 (HTTP Status Code), String (Reason Phrase, e.g. 'OK')) */
#[derive(Debug, PartialEq)]
pub struct StatusCode(pub u16, pub &'static str);

impl StatusCode {
    pub const CONTINUE: StatusCode = StatusCode(100, "Continue");
    pub const SWITCHING_PROTOCOLS: StatusCode = StatusCode(101, "Switching Protocols");
    // Successful 2xx
    pub const OK: StatusCode = StatusCode(200, "OK");
    pub const CREATED: StatusCode = StatusCode(201, "Created");
    pub const ACCEPTED: StatusCode = StatusCode(202, "Accepted");
    pub const NON_AUTHORITATIVE_INFORMATION: StatusCode =
        StatusCode(203, "Non-Authoritative Information");
    pub const NO_CONTENT: StatusCode = StatusCode(204, "No Content");
    pub const RESET_CONTENT: StatusCode = StatusCode(205, "Reset Content");
    pub const PARTIAL_CONTENT: StatusCode = StatusCode(206, "Partial Content");
    // Redirection 3xx
    pub const MULTIPLE_CHOICES: StatusCode = StatusCode(300, "Multiple Choices");
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode(301, "Moved Permanently");
    pub const FOUND: StatusCode = StatusCode(302, "Found");
    pub const SEE_OTHER: StatusCode = StatusCode(303, "See Other");
    pub const NOT_MODIFIED: StatusCode = StatusCode(304, "Not Modified");
    pub const USE_PROXY: StatusCode = StatusCode(305, "Use Proxy");
    pub const TEMPORARY_REDIRECT: StatusCode = StatusCode(307, "Temporary Redirect");
    pub const PERMANENT_REDIRECT: StatusCode = StatusCode(308, "Permanent Redirect");
    // Client Error 4xx
    pub const BAD_REQUEST: StatusCode = StatusCode(400, "Bad Request");
    pub const UNAUTHORIZED: StatusCode = StatusCode(401, "Unauthorized");
    pub const PAYMENT_REQUIRED: StatusCode = StatusCode(402, "Payment Required");
    pub const FORBIDDEN: StatusCode = StatusCode(403, "Forbidden");
    pub const NOT_FOUND: StatusCode = StatusCode(404, "Not Found");
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode(405, "Method Not Allowed");
    pub const NOT_ACCEPTABLE: StatusCode = StatusCode(406, "Not Acceptable");
    pub const PROXY_AUTHENTICATION_REQUIRED: StatusCode =
        StatusCode(407, "Proxy Authentication Required");
    pub const REQUEST_TIMEOUT: StatusCode = StatusCode(408, "Request Timeout");
    pub const CONFLICT: StatusCode = StatusCode(409, "Conflict");
    pub const GONE: StatusCode = StatusCode(410, "Gone");
    pub const LENGTH_REQUIRED: StatusCode = StatusCode(411, "Length Required");
    pub const PRECONDITION_FAILED: StatusCode = StatusCode(412, "Precondition Failed");
    pub const CONTENT_TOO_LARGE: StatusCode = StatusCode(413, "Content Too Large");
    pub const URI_TOO_LONG: StatusCode = StatusCode(414, "URI Too Long");
    pub const UNSUPPORTED_MEDIA_TYPE: StatusCode = StatusCode(415, "Unsupported Media Type");
    pub const RANGE_NOT_SATISFIABLE: StatusCode = StatusCode(416, "Range Not Satisfiable");
    pub const EXPECTATION_FAILED: StatusCode = StatusCode(417, "Expectation Failed");
    pub const MISDIRECT_REQUEST: StatusCode = StatusCode(421, "Misdirected Request");
    pub const UNPROCESSABLE_CONTENT: StatusCode = StatusCode(422, "Unprocessable Content");
    pub const UPGRADE_REQUIRED: StatusCode = StatusCode(426, "Upgrade Required");
    // Server Error 5xx
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500, "Internal Server Error");
    pub const NOT_IMPLEMENTED: StatusCode = StatusCode(501, "Not Implemented");
    pub const BAD_GATEWAY: StatusCode = StatusCode(502, "Bad Gateway");
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503, "Service Unavailable");
    pub const GATEWAY_TIMEOUT: StatusCode = StatusCode(504, "Gateway Timeout");
    pub const HTTP_VERSION_NOT_SUPPORTED: StatusCode =
        StatusCode(505, "HTTP Version Not Supported");

    pub fn from_int(code: usize) -> Result<Self, ()> {
        match code {
            100 => Ok(StatusCode::CONTINUE),
            101 => Ok(StatusCode::SWITCHING_PROTOCOLS),
            200 => Ok(StatusCode::OK),
            201 => Ok(StatusCode::CREATED),
            202 => Ok(StatusCode::ACCEPTED),
            203 => Ok(StatusCode::NON_AUTHORITATIVE_INFORMATION),
            204 => Ok(StatusCode::NO_CONTENT),
            205 => Ok(StatusCode::RESET_CONTENT),
            206 => Ok(StatusCode::PARTIAL_CONTENT),
            300 => Ok(StatusCode::MULTIPLE_CHOICES),
            301 => Ok(StatusCode::MOVED_PERMANENTLY),
            302 => Ok(StatusCode::FOUND),
            303 => Ok(StatusCode::SEE_OTHER),
            304 => Ok(StatusCode::NOT_MODIFIED),
            305 => Ok(StatusCode::USE_PROXY),
            307 => Ok(StatusCode::TEMPORARY_REDIRECT),
            308 => Ok(StatusCode::PERMANENT_REDIRECT),
            400 => Ok(StatusCode::BAD_REQUEST),
            401 => Ok(StatusCode::UNAUTHORIZED),
            402 => Ok(StatusCode::PAYMENT_REQUIRED),
            403 => Ok(StatusCode::FORBIDDEN),
            404 => Ok(StatusCode::NOT_FOUND),
            405 => Ok(StatusCode::METHOD_NOT_ALLOWED),
            406 => Ok(StatusCode::NOT_ACCEPTABLE),
            407 => Ok(StatusCode::PROXY_AUTHENTICATION_REQUIRED),
            408 => Ok(StatusCode::REQUEST_TIMEOUT),
            409 => Ok(StatusCode::CONFLICT),
            410 => Ok(StatusCode::GONE),
            411 => Ok(StatusCode::LENGTH_REQUIRED),
            412 => Ok(StatusCode::PRECONDITION_FAILED),
            413 => Ok(StatusCode::CONTENT_TOO_LARGE),
            414 => Ok(StatusCode::URI_TOO_LONG),
            415 => Ok(StatusCode::UNSUPPORTED_MEDIA_TYPE),
            416 => Ok(StatusCode::RANGE_NOT_SATISFIABLE),
            417 => Ok(StatusCode::EXPECTATION_FAILED),
            421 => Ok(StatusCode::MISDIRECT_REQUEST),
            422 => Ok(StatusCode::UNPROCESSABLE_CONTENT),
            426 => Ok(StatusCode::UPGRADE_REQUIRED),
            500 => Ok(StatusCode::INTERNAL_SERVER_ERROR),
            501 => Ok(StatusCode::NOT_IMPLEMENTED),
            502 => Ok(StatusCode::BAD_GATEWAY),
            503 => Ok(StatusCode::SERVICE_UNAVAILABLE),
            504 => Ok(StatusCode::GATEWAY_TIMEOUT),
            505 => Ok(StatusCode::HTTP_VERSION_NOT_SUPPORTED),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_int() {
        assert_eq!(StatusCode::CONTINUE, StatusCode::from_int(100).unwrap());
        assert_eq!(
            StatusCode::SWITCHING_PROTOCOLS,
            StatusCode::from_int(101).unwrap()
        );
        assert_eq!(StatusCode::OK, StatusCode::from_int(200).unwrap());
        assert_eq!(StatusCode::CREATED, StatusCode::from_int(201).unwrap());
        assert_eq!(StatusCode::ACCEPTED, StatusCode::from_int(202).unwrap());
        assert_eq!(
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            StatusCode::from_int(203).unwrap()
        );
        assert_eq!(StatusCode::NO_CONTENT, StatusCode::from_int(204).unwrap());
        assert_eq!(
            StatusCode::RESET_CONTENT,
            StatusCode::from_int(205).unwrap()
        );
        assert_eq!(
            StatusCode::PARTIAL_CONTENT,
            StatusCode::from_int(206).unwrap()
        );
        assert_eq!(
            StatusCode::MULTIPLE_CHOICES,
            StatusCode::from_int(300).unwrap()
        );
        assert_eq!(
            StatusCode::MOVED_PERMANENTLY,
            StatusCode::from_int(301).unwrap()
        );
        assert_eq!(StatusCode::FOUND, StatusCode::from_int(302).unwrap());
        assert_eq!(StatusCode::SEE_OTHER, StatusCode::from_int(303).unwrap());
        assert_eq!(StatusCode::NOT_MODIFIED, StatusCode::from_int(304).unwrap());
        assert_eq!(StatusCode::USE_PROXY, StatusCode::from_int(305).unwrap());
        assert_eq!(
            StatusCode::TEMPORARY_REDIRECT,
            StatusCode::from_int(307).unwrap()
        );
        assert_eq!(
            StatusCode::PERMANENT_REDIRECT,
            StatusCode::from_int(308).unwrap()
        );
        assert_eq!(StatusCode::BAD_REQUEST, StatusCode::from_int(400).unwrap());
        assert_eq!(StatusCode::UNAUTHORIZED, StatusCode::from_int(401).unwrap());
        assert_eq!(
            StatusCode::PAYMENT_REQUIRED,
            StatusCode::from_int(402).unwrap()
        );
        assert_eq!(StatusCode::FORBIDDEN, StatusCode::from_int(403).unwrap());
        assert_eq!(StatusCode::NOT_FOUND, StatusCode::from_int(404).unwrap());
        assert_eq!(
            StatusCode::METHOD_NOT_ALLOWED,
            StatusCode::from_int(405).unwrap()
        );
        assert_eq!(
            StatusCode::NOT_ACCEPTABLE,
            StatusCode::from_int(406).unwrap()
        );
        assert_eq!(
            StatusCode::PROXY_AUTHENTICATION_REQUIRED,
            StatusCode::from_int(407).unwrap()
        );
        assert_eq!(
            StatusCode::REQUEST_TIMEOUT,
            StatusCode::from_int(408).unwrap()
        );
        assert_eq!(StatusCode::CONFLICT, StatusCode::from_int(409).unwrap());
        assert_eq!(StatusCode::GONE, StatusCode::from_int(410).unwrap());
        assert_eq!(
            StatusCode::LENGTH_REQUIRED,
            StatusCode::from_int(411).unwrap()
        );
        assert_eq!(
            StatusCode::PRECONDITION_FAILED,
            StatusCode::from_int(412).unwrap()
        );
        assert_eq!(
            StatusCode::CONTENT_TOO_LARGE,
            StatusCode::from_int(413).unwrap()
        );
        assert_eq!(StatusCode::URI_TOO_LONG, StatusCode::from_int(414).unwrap());
        assert_eq!(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            StatusCode::from_int(415).unwrap()
        );
        assert_eq!(
            StatusCode::RANGE_NOT_SATISFIABLE,
            StatusCode::from_int(416).unwrap()
        );
        assert_eq!(
            StatusCode::EXPECTATION_FAILED,
            StatusCode::from_int(417).unwrap()
        );
        assert_eq!(
            StatusCode::MISDIRECT_REQUEST,
            StatusCode::from_int(421).unwrap()
        );
        assert_eq!(
            StatusCode::UNPROCESSABLE_CONTENT,
            StatusCode::from_int(422).unwrap()
        );
        assert_eq!(
            StatusCode::UPGRADE_REQUIRED,
            StatusCode::from_int(426).unwrap()
        );
        assert_eq!(
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::from_int(500).unwrap()
        );
        assert_eq!(
            StatusCode::NOT_IMPLEMENTED,
            StatusCode::from_int(501).unwrap()
        );
        assert_eq!(StatusCode::BAD_GATEWAY, StatusCode::from_int(502).unwrap());
        assert_eq!(
            StatusCode::SERVICE_UNAVAILABLE,
            StatusCode::from_int(503).unwrap()
        );
        assert_eq!(
            StatusCode::GATEWAY_TIMEOUT,
            StatusCode::from_int(504).unwrap()
        );
        assert_eq!(
            StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            StatusCode::from_int(505).unwrap()
        );
    }
}
