pub mod api_utils;
pub mod containers;
pub mod images;
pub mod version;

use utils::Response;

/// Highest level trait for a DockerAPI client
///
/// To implement this trait the only required method is
/// `request`.
/// Other docker API traits like containers and images must derive this
/// trait, so that any Client implementing the API should have a request
/// method available.
pub trait DockerApiClient {
    /// Just a helper function for the Containers DockerApiClient.
    /// It formats the API request using the given parameters, and using
    /// this request the docker daemon and sends back the response of the request
    /// if the request was successful else an err.
    fn get_response_from_api(
        &self,
        api_endpoint: &str,
        method: &str,
        body: &str,
    ) -> Result<Response, String> {
        let req = match api_utils::get_formatted_api_request(
            api_endpoint,
            method,
            body,
        ) {
            Some(req) => req,
            None => return Err("Error while preparing request".to_string()),
        };

        match self.request(&req) {
            Some(resp) => match Response::parse_http_response(resp) {
                Ok(response) => Ok(response),
                Err(err) => {
                    Err(format!("Response body was not valid : {}", err))
                }
            },
            None => Err("Got no response from docker host.".to_string()),
        }
    }

    fn request(&self, request: &str) -> Option<Vec<u8>>;
}
