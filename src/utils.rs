use std::collections::HashMap;
use std::path::Path;

use std::str;
use std::usize;

// This implementation of HTTP response parsing is mostly taken from
// https://github.com/p00s/minihttpse
// with minor changes.
const CR: u8 = b'\r';
const LF: u8 = b'\n';

#[derive(Debug)]
pub struct Response {
    pub status_code: usize,
    pub body: String,
}
/// Response represent a minimal HTTP response that we are concerned with
/// for docker API response parsing.
///
/// The implementation is pretty straight forward and does not do something
/// awfully bad.
impl Response {
    pub fn parse_http_response(res: Vec<u8>) -> Result<Response, String> {
        let mut pos: usize = 0;
        for i in 0..(res.len() - 1) {
            if res[i] == CR && res[i + 1] == LF && res[i + 2] == CR
                && res[i + 3] == LF
            {
                pos = i + 3;
                break;
            }
        }

        if pos == 0 {
            return Err("Not a valid HTTP response".to_string());
        }

        let (resp_header, resp_body): (&[u8], &[u8]) = res.split_at(pos);

        let header_info = match String::from_utf8(resp_header.to_vec()) {
            Ok(h) => h,
            Err(_) => return Err("Error while parsing HTTP header".to_string()),
        };

        let body = resp_body[1..].to_owned();

        let mut header_vec: Vec<&str> = header_info.split("\r\n").collect();
        let status = header_vec[0].to_owned();
        let status_vec: Vec<&str> = status.splitn(3, " ").collect();

        let status_code: usize = match status_vec[1].parse() {
            Ok(s) => s,
            Err(_) => {
                return Err("Error while parsing HTTP status code".to_string())
            }
        };

        header_vec.remove(0);
        let len = header_vec.len();
        header_vec.remove(len - 1);

        let mut headers: HashMap<String, String> = HashMap::new();
        for header in header_vec {
            let item = header.to_owned();
            let item_vec: Vec<&str> = item.splitn(2, ": ").collect();
            headers.insert(item_vec[0].to_owned(), item_vec[1].to_owned());
        }

        let body = match headers.get("Transfer-Encoding") {
            Some(enc) => {
                if enc == "chunked" {
                    match Response::parse_chunk(body) {
                        Ok(parsed) => parsed,
                        Err(err) => return Err(err),
                    }
                } else {
                    body
                }
            }
            None => body,
        };

        let response = match String::from_utf8(body) {
            Ok(s) => s.trim().to_owned(),
            Err(_) => {
                return Err("Error while parsing response body".to_string())
            }
        };

        Ok(Response {
            status_code: status_code,
            body: response,
        })
    }

    pub fn parse_chunk(body: Vec<u8>) -> Result<Vec<u8>, String> {
        let mut buf: Vec<u8> = Vec::new();
        let mut count: usize = 0;

        loop {
            let mut pos: usize = 0;
            for i in count..body.len() - 1 {
                if body[i] == CR && body[i + 1] == LF {
                    pos = i;
                    break;
                }
            }
            if pos == 0 {
                return Err("Chuncked response without length marker".to_string());
            }

            let size_s = match str::from_utf8(&body[count..pos]) {
                Ok(s) => s,
                Err(_) => return Err("Invlid chunks".to_string()),
            };

            count = pos + 2;
            let size: usize = match usize::from_str_radix(size_s, 16) {
                Ok(s) => s,
                Err(_) => return Err("Invalid chunks".to_string()),
            };

            if size == 0 && count + 2 == body.len() {
                return Ok(buf);
            }

            buf.extend_from_slice(&body[pos + 2..pos + 2 + size]);
            count = count + size + 2;
        }
    }
}

/// Parses HTTP response body. Takes an HTTP response calls the Response
/// structure to parse the response.
/// It returns the parsed HTTP response body. We are not dealing with
/// errors here. So if something goes south do check once here although it
/// is highly unlikely
///
/// TODO: Remove all printlns from here.
pub fn parse_http_response_body(resp: Vec<u8>) -> Option<String> {
    match Response::parse_http_response(resp) {
        Ok(response) => {
            if response.status_code == 200 {
                return Some(response.body);
            } else {
                println!("Response is not OK");
                return None;
            }
        }
        Err(err) => {
            println!("Error while parsing response {}", err);
            return None;
        }
    }
}

/// This function validates a given unix domain socket address, it can be either
/// of an absolute socket path or unix domain socket address.
///
/// * unix:///var/run/docker.sock
/// * /var/run/docker.sock
///
/// The function checks wheather the provided address points to a valid socket
/// or not. It returns a Vector of slices containing the protocol("unix" by default)
/// and the address to the socket wrapped in option.
pub fn validate_unix_socket_address(address: &str) -> Option<Vec<&str>> {
    let socket_protocol = "unix";
    let addr_comp: Vec<&str>;

    if address.contains("://") {
        addr_comp = address.split("://").collect();
        if addr_comp.len() != 2 || addr_comp[0] != socket_protocol {
            return None;
        }
    } else {
        addr_comp = vec![socket_protocol, address];
    }

    let path = Path::new(addr_comp[1]);
    if !path.exists() {
        return None;
    }

    return Some(addr_comp);
}
