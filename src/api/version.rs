use api::api_utils::get_formatted_api_request;
use api::DockerApiClient;
use utils;

pub trait Version: DockerApiClient {
    fn get_version_info(&self) -> Result<String, String> {
        let api_endpoint = "/info";
        let method = "GET";

        let req = match get_formatted_api_request(api_endpoint, method, "") {
            Some(req) => req,
            None => return Err("Error while preparing request".to_string()),
        };

        let resp = match self.request(&req) {
            Some(resp) => match utils::parse_http_response_body(resp) {
                Some(body) => body,
                None => return Err("Response body was not valid".to_string()),
            },
            None => return Err("Got no response from docker host.".to_string()),
        };

        Ok(resp)
    }
}
