#![allow(non_snake_case)]
use std::collections::HashMap;

use api::api_utils;
use api::DockerApiClient;
use utils::Response;

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

    #[serde(default)]
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

/// Structure for implementing Container Config
/// Derives Default fot being able to get started even with minimal
/// config.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContainerConfig {
    pub Image: String,
    pub Cmd: Vec<String>,

    pub Hostname: String,
    pub Domainname: String,
    pub User: String,
    pub AttachStdin: bool,
    pub AttachStdout: bool,
    pub AttachStderr: bool,
    pub Tty: bool,
    pub OpenStdin: bool,
    pub StdinOnce: bool,
    pub Env: Vec<String>,
    pub Entrypoint: Option<String>,
    pub Labels: Option<HashMap<String, String>>,
    pub WorkingDir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateContainerResponse {
    pub Id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContainerState {
    pub Status: String,
    pub Running: bool,
    pub Paused: bool,
    pub Restarting: bool,
    pub OOMKilled: bool,
    pub Dead: bool,
    pub Pid: u64,
    pub ExitCode: u64,
    pub Error: String,
    pub StartedAt: String,
    pub FinishedAt: String,
}

/// * To use HostConfig use serde_json
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContainerDetails {
    pub Id: String,
    pub Created: String,
    pub Path: String,
    pub Platform: Option<String>,
    pub Args: Vec<String>,
    pub State: ContainerState,
    pub Image: String,
    pub ResolvConfPath: String,
    pub Name: String,
    pub HostnamePath: String,
    pub HostsPath: String,
    pub LogPath: String,
    pub RestartCount: u64,
    pub Driver: String,
    pub MountLabel: String,
    pub ProcessLabel: String,
    pub AppArmorProfile: String,
    pub ExecIDs: Option<String>,
    pub HostConfig: serde_json::Value,
    pub Config: ContainerConfig,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContainerFsChange {
    Path: String,
    Kind: u8,
}

pub trait Containers: DockerApiClient {
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

    /// Get Containers from the API endpoint with the method and query_param.
    /// Helper function for Container trait.
    fn get_containers(
        &self,
        api_endpoint: &str,
        method: &str,
        query_param: &str,
    ) -> Result<Vec<Container>, String> {
        let json_resp =
            match self.get_response_from_api(api_endpoint, method, query_param)
            {
                Ok(resp) => {
                    if resp.status_code == 200 {
                        resp.body
                    } else {
                        return Err(format!(
                            "Invalid Response : {} :: {}",
                            resp.status_code, resp.body
                        ));
                    }
                }
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

        return Ok(containers);
    }

    /// List all the running containers
    /// Return an instance of Vector of container
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate docker_rs;
    ///
    /// use docker_rs::api::containers::Containers;
    /// use docker_rs::client::DockerClient;
    ///
    /// let client = match DockerClient::new("unix:///var/run/docker.sock") {
    ///     Ok(a) => a,
    ///     Err(err) => {
    ///         println!("{}", err);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// match client.list_running_containers(None) {
    ///     Ok(containers) => println!("{:?}", containers),
    ///     Err(err) => println!("An error occured : {}", err),
    /// }
    /// ```
    fn list_running_containers(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<Container>, String> {
        let api_endpoint = "/containers/json";
        let method = "GET";

        let query_params = match limit {
            Some(limit) => format!("?size=true&limit={}", limit),
            None => "?size=true".to_string(),
        };

        self.get_containers(api_endpoint, method, &query_params)
    }

    /// List all containers whether running or stopped.
    fn list_all_containers(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<Container>, String> {
        let api_endpoint = "/containers/json";
        let method = "GET";

        let query_params = match limit {
            Some(limit) => format!("?all=true&size=true&limit={}", limit),
            None => "?all=true&size=true".to_string(),
        };

        self.get_containers(api_endpoint, method, &query_params)
    }

    /// List container with the filter provided, the filter can be looked from
    /// Docker engine official API documentation.
    /// https://docs.docker.com/engine/api/v1.37/#operation/ContainerList
    fn get_container_details_with_filter(
        &self,
        filter: &str,
        limit: Option<u32>,
    ) -> Result<Vec<Container>, String> {
        let api_endpoint = "/containers/json";
        let method = "GET";

        let query_params = match limit {
            Some(limit) => format!(
                "?all=true&size=true&limit={}&filter={}",
                limit, filter
            ),
            None => format!("?all=true&size=true&filter={}", filter),
        };

        self.get_containers(api_endpoint, method, &query_params)
    }

    /// Create a container from the ContainerConfig structure with the provided
    /// `name`. The response for the request is the CreateContaierResponse struct
    /// which contains the ID for the container which we created.
    fn create_container(
        &self,
        name: &str,
        config: ContainerConfig,
    ) -> Result<CreateContainerResponse, String> {
        let api_endpoint = format!("/containers/create?name={}", name);
        let method = "POST";
        let body = match serde_json::to_string(&config) {
            Ok(body) => body,
            Err(err) => {
                return Err(format!(
                    "Error while serialize Cotainer config : {}",
                    err
                ))
            }
        };

        match self.get_response_from_api(&api_endpoint, method, &body) {
            Ok(resp) => {
                if resp.status_code != 201 {
                    return Err(format!(
                        "Invalid Request : {} :: {}",
                        resp.status_code, resp.body
                    ));
                }
                match serde_json::from_str(&resp.body) {
                    Ok(info) => return Ok(info),
                    Err(err) => {
                        return Err(format!(
                            "Error while deserializing JSON response : {}",
                            err
                        ))
                    }
                };
            }
            Err(err) => Err(err),
        }
    }

    /// Creates/Spawn docker container from the configuration provided. It only
    ///
    /// * Rust does not provide named arguments, so we are doing it this way
    /// Currently rust structures does not have default values, so all the
    /// values for the structure needs to be specified.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate docker_rs;
    ///
    /// use docker_rs::api::containers::Containers;
    /// use docker_rs::client::DockerClient;
    ///
    /// let client = match DockerClient::new("unix:///var/run/docker.sock") {
    ///     Ok(a) => a,
    ///     Err(err) => {
    ///         println!("{}", err);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// let mut cmd: Vec<String> = Vec::new();
    /// cmd.push("ls".to_string());
    ///
    /// match client.create_container_minimal("my_container", "debian:jessie", cmd) {
    ///     Ok(containers) => println!("{:?}", containers),
    ///     Err(err) => println!("An error occured : {}", err),
    /// }
    /// ```
    fn create_container_minimal(
        &self,
        name: &str,
        image: &str,
        cmd: Vec<String>,
    ) -> Result<CreateContainerResponse, String> {
        let config = ContainerConfig {
            Image: image.to_string(),
            Cmd: cmd,
            ..Default::default()
        };

        self.create_container(name, config)
    }

    /// Inspects the container with the provided ID
    /// Returns Low level information about the container.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate docker_rs;
    ///
    /// use docker_rs::api::containers::Containers;
    /// use docker_rs::client::DockerClient;
    ///
    /// let client = match DockerClient::new("unix:///var/run/docker.sock") {
    ///     Ok(a) => a,
    ///     Err(err) => {
    ///         println!("{}", err);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// // ID of the container passed as an argument.
    /// match client.inspect_container("f808ca...") {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(err) => println!("An error occured : {}", err),
    /// }
    /// ```
    fn inspect_container(&self, id: &str) -> Result<ContainerDetails, String> {
        let api_endpoint = format!("/containers/{id}/json", id = id);
        let method = "GET";

        match self.get_response_from_api(&api_endpoint, method, "") {
            Ok(resp) => {
                if resp.status_code != 200 {
                    return Err(format!(
                        "Invalid Request : {} :: {}",
                        resp.status_code, resp.body
                    ));
                }
                match serde_json::from_str(&resp.body) {
                    Ok(info) => return Ok(info),
                    Err(err) => {
                        return Err(format!(
                            "Error while deserializing JSON response : {}",
                            err
                        ))
                    }
                };
            }
            Err(err) => Err(err),
        }
    }

    /// Gives the changes done to somewhere in the filesystem in the docker container as a list of
    /// files with the kind of changes.
    fn get_container_filesystem_changes(
        &self,
        id: &str,
    ) -> Result<Vec<ContainerFsChange>, String> {
        let api_endpoint = format!("/containers/{id}/changes", id = id);
        let method = "GET";

        match self.get_response_from_api(&api_endpoint, method, "") {
            Ok(resp) => {
                // If the response is null, then there is no changes in the file
                // system so just return and empty vector. Serializing this will
                // result in error.
                if resp.status_code != 200 {
                    return Err(format!(
                        "Invalid Request : {} :: {}",
                        resp.status_code, resp.body
                    ));
                }
                if resp.body == "null" {
                    return Ok(Vec::new());
                }

                match serde_json::from_str(&resp.body) {
                    Ok(info) => return Ok(info),
                    Err(err) => {
                        return Err(format!(
                            "Error while deserializing JSON response : {}",
                            err
                        ))
                    }
                };
            }
            Err(err) => Err(err),
        }
    }

    /// Function to manipulate container status
    /// It is a parent function for all the commands which result in a status change
    /// of the container.
    ///
    /// This includes the following:
    /// * `start_container`
    /// * `stop_container`
    /// * `pause_container`
    /// * `unpause_container`
    /// * `restart_container`
    /// * `kill_container`
    /// * `rename_container`
    ///
    /// You can call any of these function or directly manipulate_container_status
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate docker_rs;
    ///
    /// use docker_rs::api::containers::Containers;
    /// use docker_rs::client::DockerClient;
    ///
    /// let client = match DockerClient::new("unix:///var/run/docker.sock") {
    ///     Ok(a) => a,
    ///     Err(err) => {
    ///         println!("{}", err);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// // ID of the container passed as an argument.
    /// match client.manipulate_container_status("start", "f808ca...", "") {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(err) => println!("An error occured : {}", err),
    /// }
    ///
    /// // Or alternatively you can also directly use
    /// match client.start_container("f808ca...") {
    ///     Ok(info) => println!("{}", info),
    ///     Err(err) => println!("An error occured : {}", err),
    /// }
    ///
    /// // Similarly other function can also be used
    /// ```
    fn manipulate_container_status(
        &self,
        action: &str,
        id: &str,
        params: &str,
    ) -> Result<String, String> {
        let api_endpoint = format!(
            "/containers/{id}/{action}",
            id = id,
            action = action
        );
        let method = "GET";

        match self.get_response_from_api(&api_endpoint, method, params) {
            Ok(resp) => {
                if resp.status_code == 204 {
                    Ok(format!("Container {} successful", action))
                } else if resp.status_code == 304 {
                    Err(format!("Container already {}ed", action))
                } else {
                    Err(format!(
                        "Error while requesting Docker API : {} :: {}",
                        resp.status_code, resp.body
                    ))
                }
            }
            Err(err) => return Err(err),
        }
    }

    fn start_container(&self, id: &str) -> Result<String, String> {
        self.manipulate_container_status("start", id, "")
    }

    fn stop_container(
        &self,
        id: &str,
        delay: Option<&str>,
    ) -> Result<String, String> {
        let param = match delay {
            Some(d) => format!("t={}", d),
            None => String::new(),
        };
        self.manipulate_container_status("stop", id, &param)
    }

    fn pause_container(&self, id: &str) -> Result<String, String> {
        self.manipulate_container_status("pause", id, "")
    }

    fn unpause_container(&self, id: &str) -> Result<String, String> {
        self.manipulate_container_status("unpause", id, "")
    }

    fn restart_container(
        &self,
        id: &str,
        delay: Option<&str>,
    ) -> Result<String, String> {
        let param = match delay {
            Some(d) => format!("t={}", d),
            None => String::new(),
        };
        self.manipulate_container_status("restart", id, &param)
    }

    fn kill_container(
        &self,
        id: &str,
        signal: Option<&str>,
    ) -> Result<String, String> {
        let param = match signal {
            Some(sig) => format!("signal={}", sig),
            None => String::new(),
        };
        self.manipulate_container_status("kill", id, &param)
    }

    fn rename_container(&self, id: &str, name: &str) -> Result<String, String> {
        let name_param = &format!("name={}", name);
        self.manipulate_container_status("rename", id, name_param)
    }
}
