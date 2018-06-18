quick_error! {
    #[derive(Debug)]
    pub enum DockerClientError {
        InvalidTargetAddress(addr: &'static str) {
            description("The target address is not valid")
            display("The target address `{}` is not valid", addr)
        }

        SocketConnectionError(addr: &'static str) {
            description("Could not connect to docker socket.")
            display("Error while connection to docker socket at {}", addr)
        }
    }
}
