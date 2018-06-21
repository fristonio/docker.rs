static API_VERSION: &'static str = "v1.37";

/// Gives a formatted API request which should be writtern
/// to the socket to docker.
pub fn get_formatted_api_request(
    api_endpoint: &str,
    method: &str,
    body: &str,
) -> Option<String> {
    if method == "GET" || method == "get" {
        return Some(format!(
            "GET {endpoint}{body} HTTP/1.1\r\nHost: {version}\r\n\r\n",
            endpoint = api_endpoint,
            body = body,
            version = API_VERSION
        ));
    }

    if method == "POST" || method == "post" {
        return Some(format!(
            "POST {endpoint} HTTP/1.1\r\nHost: {version}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{external_body}\r\n\r\n",
            endpoint = api_endpoint,
            version = API_VERSION,
            length = body.len(),
            external_body = body
        ));
    }

    None
}
