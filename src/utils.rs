use std::path::Path;

/// This function validates a given unix domain socket address, it can be either
/// of an absolute socket path or unix domain socket address.
///
/// * unix:///var/run/docker.sock
/// * /var/run/docker.sock
///
/// The function checks wheather the provided address points to a valid socket
/// or not. It returns a Vector of slices containing the protocol("unix" by default)
/// and the address to the socket wrapped in option.
pub fn validate_unix_socket_address(address: &str) -> Option<Vec<&str>> {
    let socket_protocol = "unix";
    let addr_comp: Vec<&str>;

    if address.contains("://") {
        addr_comp = address.split("://").collect();
        if addr_comp.len() != 2 || addr_comp[0] != socket_protocol {
            return None;
        }
    } else {
        addr_comp = vec![socket_protocol, address];
    }

    let path = Path::new(addr_comp[1]);
    if !path.is_file() {
        return None;
    }

    return Some(addr_comp);
}
