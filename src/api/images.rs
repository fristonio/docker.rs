#![allow(non_snake_case)]

use std::collections::HashMap;

use api::DockerApiClient;
use utils;

use serde_json;

use errors::DockerApiError;

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
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate rust_docker;
    ///
    /// use rust_docker::api::images::Images;
    /// use rust_docker::client::DockerClient;
    ///
    /// let client = match DockerClient::new("unix:///var/run/docker.sock") {
    ///     Ok(a) => a,
    ///     Err(err) => {
    ///         println!("{}", err);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// let images_info = client.list_images(None).expect("Error");
    /// println!("{:?}", images_info);
    /// ```
    fn list_images(
        &self,
        filter: Option<&str>,
    ) -> Result<Vec<ImageCompactInfo>, DockerApiError> {
        let api_endpoint = "/images/json";
        let method = "GET";

        let filter_val = filter.unwrap_or("");
        if !filter_val.is_empty() && !utils::api::validate_json_str(filter_val)
        {
            return Err(DockerApiError::MismatchedParametersError(
                "The provided filter is not a valid JSON.",
            ));
        }

        let query_params = &format!("?filter={}", filter_val);

        let resp =
            self.get_response_from_api(api_endpoint, method, query_params)?;
        if resp.status_code != 200 {
            return Err(DockerApiError::InvalidApiResponseError(
                resp.status_code,
                resp.body,
            ));
        }

        let images_info: Vec<ImageCompactInfo> =
            match serde_json::from_str(&resp.body) {
                Ok(info) => info,
                Err(err) => {
                    return Err(DockerApiError::JsonDeserializationError(err))
                }
            };

        Ok(images_info)
    }

    /// Build docker image from the given directory
    /// This function uses docker API to construct an image, and returns the ID
    /// for that image as a result. If an error occurs in between it returns
    /// DockerApiError.
    ///
    /// Building image will be canceled in case when the connection to the docker host
    /// is lost. This function assumes that the Dockerfile is present in the root of the
    /// base directory. If there is not Dockerfile in the root of the directory, this
    /// function will return an early error specifying that the Dockerfile was not found.
    ///
    /// TODO: Add an option to provide a custom Dockerfile path inside the base directory.
    fn build_image(&self, base_dir: &str) -> Result<(), DockerApiError> {
        let base_dir_path = match utils::file::get_validated_dir_path(base_dir)
        {
            Ok(base_dir_path) => base_dir_path,
            Err(e) => {
                Err(DockerApiError::MismatchedParametersError(e.description()))
            }
        };

        let default_dockerfile_path = "Dockerfile";
        if !base_dir_path
            .join(default_dockerfile_path)
            .as_path()
            .is_file()
        {
            Err(DockerApiError::MismatchedParametersError(format!(
                "No Dockerfile present in the root of the directory : {}",
                base_dir_path
            )))
        }

        let tar_path = match utils::file::create_gzipped_tarball(
            base_dir_path.to_str().unwrap(),
        ) {
            Ok(tar_path) => tar_path,
            Err(e) => {
                Err(DockerApiError::MismatchedParametersError(e.description()))
            }
        };

        Ok(build_image_from_tarball(tar_path.to_str().unwrap()))
    }

    fn build_image_from_tarball(&self, _tar_path: &str) {}
}
