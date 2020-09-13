pub const RESPONSE_CODES: [(&str, &str); 2] = [("200", "OK"), ("404", "NOT FOUND")];

pub const GET: &[u8; 16] = b"GET / HTTP/1.1\r\n";
