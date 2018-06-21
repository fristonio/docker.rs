# docker.rs

> A docker API wrapper library for rust.

## Development

docker.rs is currently under development, follow the below instructions to get started.

* Install rustc and rust-toolchain.
* Clone the repository.
* Jump to `src/` and start hacking.

### Usage

docker_rs provides a rust interface to interact with Docker API. It is currently build to support
latest version(1.37) of docker API. To get started make sure you have the docker daemon up.

* For now only API implementation for only containers is available. I will be adding the rest soon.

#### Connecting to docker unix socket interface.

```rust
let client = match DockerClient::new("unix:///var/run/docker.sock") {
    Ok(a) => a,
    Err(err) => {
        println!("{}", err);
        exit(1);
    }
};
```

#### Using API methods inherited by DockerApiClient.

```rust
// Get version info for docker
let info = client.get_version_info();

// Get all containers(running/stopped)
let all_containers = client.list_all_containers(None).unwrap();

// Get only running containers
let running_cont = client.list_running_containers(None).unwrap();

// Create a new container using an Image
let mut cmd: Vec<String> = Vec::new();
cmd.push("ls".to_string());
let res = client
    .create_container_minimal("kk", "debian:jessie", cmd)
    .unwrap();

// Inspect the info for a container.
let inspect_info = client
    .inspect_container(
        "f808ca866b5fa80f65d6cd0937c72049272ea4c5aa4453e2abdd08d5efb59d3d",
    )
    .unwrap();

// Get info regarding changes made to filesystem inside container
let inspect_info = client
    .get_container_filesystem_changes(
        "f808ca866b5fa80f65d6cd0937c72049272ea4c5aa4453e2abdd08d5efb59d3d",
    )
    .unwrap();


// Start a created container
let start_info = client.start_container("f808ca...").unwrap();

// Kill a container
let kill_info = client.kill_container("f808ca...").unwrap();
```


The library currently only provides unix socket interface support for communicating with docker daemon 
and is therefore fit for most purposes wherein the docker daemon you are interacting is local.
To add an implementation of HTTP capable DockerClient look at the implementation of unix socket in [/src/client.rs](/src/client.rs).

The only required method for implementing `DockerApiClient` is `request` wherein you make a request to the docker API
and returns the response. Once you have this you can implement each of api helpers like `Containers` for your client
which uses this function itself. 

### External Links

* [Docs.rs](https://docs.rs/rust_docker)
* [Crates.io](https://crates.io/crates/rust-docker)

### License

This project is licensed under [MIT License](/LICENSE.md)

### Development notes

* docker-rs encourage you to keep your code foramtted using rustfmt.
* For any feature you add to the library, write proper documentation according to rust standards.
