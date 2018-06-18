extern crate docker_rs;

use docker_rs::api::containers::Containers;
use docker_rs::api::version::Version;
use docker_rs::client::DockerClient;

use std::process::exit;

#[test]
fn test() {
    let client = match DockerClient::new("unix:///var/run/docker.sock") {
        Ok(a) => a,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };
    let _new_client = client.clone();
    client.get_version_info();
    println!("{:?}", client.list_all_containers().unwrap());
}
