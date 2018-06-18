pub mod api_utils;
pub mod version;

/// Highest level trait for a DockerAPI client
///
/// To implement this trait the only required method is
/// `request`.
/// Other docker API traits like containers and images must derive this
/// trait, so that any Client implementing the API should have a request
/// method available.
pub trait DockerApiClient {
    fn request(&self, request: &str) -> Option<Vec<u8>>;
}
