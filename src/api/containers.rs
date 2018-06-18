#![allow(non_snake_case)]
use std::collections::HashMap;

use api::api_utils;
use api::DockerApiClient;
use utils;

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub Id: String,
    pub Names: Vec<String>,
    pub Image: String,
    pub ImageID: String,
    pub Command: String,
    pub State: String,
    pub Status: String,
    pub Ports: Vec<Port>,
    pub Labels: Option<HashMap<String, String>>,
    pub SizeRw: Option<u64>,
    #[serde(default)]
    pub SizeRootFs: u64,
    pub HostConfig: HostConfig,
    pub Mounts: Vec<Mounts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    pub PrivatePort: u32,
    pub PublicPort: u32,
    pub Type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostConfig {
    pub NetworkMode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mounts {
    pub Name: String,
    pub Source: String,
    pub Destination: String,
    pub Driver: String,
    pub Mode: String,
    pub RW: bool,
    pub Propagation: String,
}

pub trait Containers: DockerApiClient {
    fn get_response_from_api(
        &self,
        api_endpoint: &str,
        method: &str,
        query_params: &str,
    ) -> Result<String, String> {
        let req = match api_utils::get_formatted_api_request(
            api_endpoint,
            method,
            query_params,
        ) {
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

    /// List all the running containers
    /// Return an instance of Vector of container
    fn list_running_docker_containers(&self) -> Result<Vec<Container>, String> {
        let api_endpoint = "/containers/json";
        let method = "GET";

        let json_resp =
            match self.get_response_from_api(api_endpoint, method, "") {
                Ok(resp) => resp,
                Err(err) => return Err(err),
            };

        let containers: Vec<Container> = match serde_json::from_str(&json_resp)
        {
            Ok(info) => info,
            Err(err) => {
                return Err(format!(
                    "Error while deserializing JSON response : {}",
                    err
                ))
            }
        };

        Ok(containers)
    }

    /// List all containers whether running or stopped.
    fn list_all_containers(&self) -> Result<Vec<Container>, String> {
        let api_endpoint = "/containers/json";
        let method = "GET";
        let query_params = "?all=true";

        let json_resp = match self.get_response_from_api(
            api_endpoint,
            method,
            query_params,
        ) {
            Ok(resp) => resp,
            Err(err) => return Err(err),
        };

        let containers: Vec<Container> = match serde_json::from_str(&json_resp)
        {
            Ok(info) => info,
            Err(err) => {
                return Err(format!(
                    "Error while deserializing JSON response : {}",
                    err
                ))
            }
        };

        Ok(containers)
    }
}
