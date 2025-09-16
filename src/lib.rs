// Library for port scanning
use std::net::ToSocketAddrs;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::Semaphore;
use std::sync::Arc;

pub fn parse_ports(port_args: Vec<&str>) -> Vec<u16> {
    let mut ports = Vec::new();
    for arg in port_args {
        for part in arg.split(',') {
            if let Some((start, end)) = part.split_once('-') {
                if let (Ok(s), Ok(e)) = (start.parse::<u16>(), end.parse::<u16>()) {
                    ports.extend(s..=e);
                }
            } else if let Ok(port) = part.parse::<u16>() {
                ports.push(port);
            }
        }
    }
    ports
}

pub async fn scan_ports(target: &str, ports: Vec<u16>, threads: usize) {
    let semaphore = Arc::new(Semaphore::new(threads));
    let mut handles = vec![];
    for port in ports {
        let sem = semaphore.clone();
        let target = target.to_string();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            let addr = format!("{}:{}", target, port);
            // TCP scan with timeout
            let tcp_result = tokio::time::timeout(
                std::time::Duration::from_millis(500),
                TcpStream::connect(&addr)
            ).await;
            match tcp_result {
                Ok(Ok(_)) => println!("TCP port {} is open", port),
                Ok(Err(_)) => println!("TCP port {} is closed", port),
                Err(_) => println!("TCP port {}: timed out", port),
            }
            // UDP scan
            let udp_addr = match addr.to_socket_addrs() {
                Ok(mut addrs) => addrs.next(),
                Err(_) => None,
            };
            if let Some(udp_addr) = udp_addr {
                if let Ok(socket) = UdpSocket::bind("0.0.0.0:0").await {
                    let payload = [0u8; 1];
                    let _ = socket.send_to(&payload, &udp_addr).await;
                    let mut buf = [0u8; 1024];
                    let recv = tokio::time::timeout(
                        std::time::Duration::from_millis(300),
                        socket.recv_from(&mut buf)
                    ).await;
                    if let Ok(Ok((_n, _src))) = recv {
                        println!("UDP port {} is open (response received)", port);
                    } else {
                        println!("UDP port {}: no response (could be open or filtered)", port);
                    }
                }
            }
        }));
    }
    for h in handles {
        let _ = h.await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_ports_single() {
        let ports = parse_ports(vec!["80"]);
        assert_eq!(ports, vec![80]);
    }
    #[test]
    fn test_parse_ports_range() {
        let ports = parse_ports(vec!["80-82"]);
        assert_eq!(ports, vec![80, 81, 82]);
    }
    #[test]
    fn test_parse_ports_multiple() {
        let ports = parse_ports(vec!["22,80-81,443"]);
        assert_eq!(ports, vec![22, 80, 81, 443]);
    }
}
