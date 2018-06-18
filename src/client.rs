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
    /// Creates a new DockerClient object connected to docker's unix domain socket.
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

        // Check if the protocol is unix or not.
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


/// Implement clone for the DockerClient structure.
/// The clone here is not true clone, the unix_socket cloned
/// still refers to the stream and change to one of the two will
/// propogate the changes to other.
impl Clone for DockerClient {
    fn clone(&self) -> Self {
        let sock = self.unix_socket
            .try_clone().expect("Error while trying to clone the socket");

        let protocol = match self.protocol {
            ConnectionProtocol::UNIX =>  ConnectionProtocol::UNIX,
        };

        let docker_client_clone = DockerClient {
            unix_socket: sock,
            protocol: protocol
        };

        return docker_client_clone;
    }
}
