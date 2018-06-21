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

    let info = client.get_version_info();
    println!("{:?}", info);

    let all_containers = client.list_all_containers(None).unwrap();
    println!("{:?}", all_containers);

    let running_cont = client.list_running_containers(None).unwrap();
    println!("{:?}", running_cont);

    let mut cmd: Vec<String> = Vec::new();
    cmd.push("ls".to_string());
    let res = client
        .create_container_minimal("kk", "debian:jessie", cmd)
        .unwrap();
    println!("{:?}", res);
}
