//! docker-rs is a rust library to interact with Docker API
//!
//! * Currently the only method to connect to docker is through unix
//! socket.
#[macro_use]
extern crate quick_error;

pub mod api;
pub mod client;
mod errors;
mod utils;

pub use client::DockerClient;
