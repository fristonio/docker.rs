//! A client for communicating with the docker server
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixStream;

use api::containers::Containers;
use api::images::Images;
use api::version::Version;
use api::DockerApiClient;

use errors::DockerClientError;
use utils;

/// A structure defining a Client to interact with the docker API
///
/// * unix_socket: UnixStream connection for docker socket.
/// * protocol: Underlying protocol we are using(UNIX by default.)
pub struct DockerClient {
    socket: UnixStream,
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
            match utils::api::validate_unix_socket_address(connection_addr) {
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
            socket: unix_socket,
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
    fn clone(&self) -> DockerClient {
        let sock = self
            .socket
            .try_clone()
            .expect("Error while trying to clone the socket");

        let protocol = match self.protocol {
            ConnectionProtocol::UNIX => ConnectionProtocol::UNIX,
        };

        let docker_client_clone = DockerClient {
            socket: sock,
            protocol: protocol,
        };

        return docker_client_clone;
    }
}

impl DockerApiClient for DockerClient {
    fn request(&self, request: &str) -> Option<Vec<u8>> {
        let mut client = self.socket.try_clone().unwrap();

        let buf = request.as_bytes();
        match client.write_all(buf) {
            Ok(_) => println!("Wrote all data to socket"),
            Err(_) => return None,
        };

        // Can't figure out why read_to_end was not working here. :/
        const BUFFER_SIZE: usize = 1024;
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut raw_resp: Vec<u8> = Vec::new();
        loop {
            let len = match client.read(&mut buffer) {
                Ok(len) => len,
                Err(_) => return None,
            };

            for i in 0..len {
                raw_resp.push(buffer[i]);
            }

            if len < BUFFER_SIZE {
                break;
            }
        }

        Some(raw_resp)
    }
}

impl Version for DockerClient {}
impl Containers for DockerClient {}
impl Images for DockerClient {}
