#![allow(non_snake_case)]

use std::collections::HashMap;

use api::DockerApiClient;
use utils;

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCompactInfo {
    pub Id: String,
    pub ParentId: String,
    pub RepoTags: Vec<String>,
    pub RepoDigests: Option<Vec<String>>,
    pub Created: u64,
    pub Size: u64,
    pub VirtualSize: u64,
    pub SharedSize: i64,
    pub Labels: Option<HashMap<String, String>>,
    pub Containers: i32,
}

pub trait Images: DockerApiClient {
    /// Only images from final layer is listed in the image by default.
    /// filter corresponds to a JSON encoded string of filters as mentioned
    /// in the https://docs.docker.com/engine/api/v1.37/#operation/ImageList
    fn list_images(
        &self,
        filter: Option<&str>,
    ) -> Result<Vec<ImageCompactInfo>, String> {
        let api_endpoint = "/images/json";
        let method = "GET";

        let filter_val = filter.unwrap_or("");
        if !filter_val.is_empty() && !utils::validate_json_str(filter_val) {
            return Err("The provided filter is not a valid JSON.".to_owned());
        }

        let query_params = &format!("?filter={}", filter_val);

        let resp =
            self.get_response_from_api(api_endpoint, method, query_params)?;
        if resp.status_code != 200 {
            return Err(format!(
                "Invalid Request : {} :: {}",
                resp.status_code, resp.body
            ));
        }

        let images_info: Vec<ImageCompactInfo> =
            match serde_json::from_str(&resp.body) {
                Ok(info) => info,
                Err(err) => {
                    return Err(format!(
                        "Error while deserializing JSON response : {}",
                        err
                    ))
                }
            };

        Ok(images_info)
    }
}
