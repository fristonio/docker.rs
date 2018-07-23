extern crate rust_docker;

use rust_docker::api::containers::Containers;
use rust_docker::api::version::Version;
use rust_docker::client::DockerClient;

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

#[test]
fn test_error_when_image_does_not_exist_locally() {
    if let Ok(client) = DockerClient::new("unix:///var/run/docker.sock") {
        let cmd = vec![String::from("ls")];
        let res = client.create_container_minimal(
            "kk",
            "this-image:doesnt-exist",
            cmd,
        );

        assert!(res.is_err());

        if let Err(e) = res {
            assert_eq!(
                format!("{}", e),
                "Invalid API response, status_code : 404, body: {\"message\":\"No such image: this-image:doesnt-exist\"}"
            );
        }
    } else {
        assert!(false, "Could not create a new DockerClient object");
    }
}
