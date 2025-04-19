use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;
use rayon::prelude::*;

fn scan_port(ip: &str, port: u16, timeout: Duration) -> bool {
    let addr = format!("{}:{}", ip, port);
    let socket_addrs: Vec<SocketAddr> = match addr.to_socket_addrs() {
        Ok(addrs) => addrs.collect(),
        Err(_) => return false,
    };

    for socket_addr in socket_addrs {
        if TcpStream::connect_timeout(&socket_addr, timeout).is_ok() {
            return true;
        }
    }

    false
}

fn main() {
    let target_ip = "172.20.10.2"; // ← à modifier 
    let start_port = 1;
    let end_port = 1024;
    let timeout = Duration::from_secs(1);

    println!("🔍 Scanning {} from port {} to {}", target_ip, start_port, end_port);

    (start_port..=end_port).into_par_iter().for_each(|port| {
        if scan_port(target_ip, port, timeout) {
            println!("✅ Port {} is open", port);
        }
    });

    println!("✅ Scan terminé !");
}
