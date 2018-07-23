use serde_json;

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

quick_error! {
    #[derive(Debug)]
    pub enum DockerApiError {
        MismatchedParametersError(msg: &'static str) {
            description("Provided parameters are not valid")
            display("Malformed parameters : {}", msg)
        }

        JsonSerializationError(err: serde_json::Error) {
            description("Error while serializing JSON")
            display("JSON Serialization error : {}", err)
        }

        JsonDeserializationError(err: serde_json::Error) {
            description("Error while deserializing JSON")
            display("JSON Deserialization error : {}", err)
        }

        RequestPrepareError(_err: &'static str) {
            description("An error occured while preparing request")
            display("Error while preparing request")
        }

        RequestError(msg: &'static str) {
            description("Request not handled properly")
            display("RequestError : {}", msg)
        }

        HTTPResponseParseError(err: &'static str) {
            description("Error while parsing response")
            display("Error while parsing response : {}", err)
        }

        InvalidApiResponseError(status: usize, body: String) {
            description("Response from Docker API is not valid")
            display("Invalid API response, status_code : {}, body: {}", status, body)
        }

        ApiRequestError(msg: &'static str) {
            description("The request to Docker API could not be handled")
            display("Docker API server Error : {}", msg)
        }

        ContainerError(msg: String) {
            description("The container in context faced some error")
            display("ContainerError : {}", msg)
        }
    }
}
