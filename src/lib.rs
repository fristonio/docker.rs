//! docker-rs is a rust library to interact with Docker API
//!
//! * Currently the only method to connect to docker is through unix
//! socket.
#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate serde_derive;

extern crate flate2;
extern crate serde;
extern crate serde_json;
extern crate tar;
extern crate uuid;

pub mod api;
pub mod client;
pub mod errors;
pub mod file;
pub mod utils;

pub use client::DockerClient;
