//! A client for communicating with the docker server
use std::os::unix::net::UnixStream;

use errors::DockerClientError;
use utils;

/// A structure defining a Client to interact with the docker API
///
/// * unix_socket: UnixStream connection for docker socket.
/// * protocol: Underlying protocol we are using(UNIX by default.)
pub struct DockerClient {
    unix_socket: UnixStream,
    protocol: ConnectionProtocol,
}

enum ConnectionProtocol {
    UNIX,
}

impl DockerClient {
    pub fn new(
        connection_addr: &'static str,
    ) -> Result<DockerClient, DockerClientError> {
        // Check if the provided unix socket address is valid and return
        // components for the socket.
        let addr_components =
            match utils::validate_unix_socket_address(connection_addr) {
                Some(addr_comps) => addr_comps,
                None => {
                    return Err(DockerClientError::InvalidTargetAddress(
                        connection_addr,
                    ))
                }
            };

        // Try connecting to the docker socket address
        let unix_socket = match UnixStream::connect(addr_components[1]) {
            Ok(sock) => sock,
            Err(_err) => {
                return Err(DockerClientError::SocketConnectionError(
                    addr_components[1],
                ))
            }
        };

        let protocol = match addr_components[0] {
            "unix" => ConnectionProtocol::UNIX,
            _ => {
                return Err(DockerClientError::InvalidTargetAddress(
                    connection_addr,
                ))
            }
        };

        let docker_client = DockerClient {
            unix_socket: unix_socket,
            protocol: protocol,
        };

        Ok(docker_client)
    }
}
