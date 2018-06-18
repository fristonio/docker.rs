/// Tempate strings for requesting the resource from docker.
static API_GET_REQUEST_TEMPLATE: &'static str =
    "GET {endpoint}?{body} HTTP/1.1\r\nHost: {version}\r\n";
static API_POST_REQUEST_TEMPLATE: &'static str =
    "POST {endpoint} HTTP/1.1{external_body}\r\nHost: {version}\r\n";

static API_VERSION: &'static str = "v1.37"

/// Gives a formatted API request which should be writtern
/// to the socket to docker.
pub fn get_formatted_api_request(
    api_endpoint: &str,
    method: &str,
    body: &str,
) -> Option<String> {
    if method == "GET" || method == "get" {
        return Some(format!(
            API_GET_REQUEST_TEMPLATE,
            endpoint = api_endpoint,
            body = body,
            version = API_VERSION
        ));
    }

    if method == "POST" || method == "post" {
        return Some(format!(
            API_POST_REQUEST_TEMPLATE,
            endpoint = api_endpoint,
            external_body = body,
            version = API_VERSION
        ));
    }

    None
}
