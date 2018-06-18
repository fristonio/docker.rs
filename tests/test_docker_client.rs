extern crate docker_rs;

use docker_rs::client::DockerClient;

fn main() {
    let client = DockerClient::new("unix:///var/run/docker.sock").unwrap();
    let _new_client = client.clone();
}
