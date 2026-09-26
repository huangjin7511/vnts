//! Listener helpers with a portable dual-stack default for wildcard IPv6 binds.

use socket2::{Domain, Protocol, Socket, Type};
use std::io;
use std::net::{SocketAddr, TcpListener, UdpSocket};

fn socket(addr: SocketAddr, socket_type: Type, protocol: Protocol) -> io::Result<Socket> {
    let socket = Socket::new(Domain::for_address(addr), socket_type, Some(protocol))?;

    // OS defaults differ: notably, Windows creates IPv6-only wildcard sockets unless
    // IPV6_V6ONLY is explicitly disabled. Keep `[::]:port` consistently dual-stack.
    if let SocketAddr::V6(addr) = addr
        && addr.ip().is_unspecified()
    {
        socket.set_only_v6(false)?;
    }

    Ok(socket)
}

pub fn bind_tcp_listener(addr: SocketAddr) -> io::Result<TcpListener> {
    let socket = socket(addr, Type::STREAM, Protocol::TCP)?;
    socket.bind(&addr.into())?;
    socket.listen(1024)?;
    let listener: TcpListener = socket.into();
    listener.set_nonblocking(true)?;
    Ok(listener)
}

pub fn bind_udp_socket(addr: SocketAddr) -> io::Result<UdpSocket> {
    let socket = socket(addr, Type::DGRAM, Protocol::UDP)?;
    socket.bind(&addr.into())?;
    let socket: UdpSocket = socket.into();
    socket.set_nonblocking(true)?;
    Ok(socket)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_ipv6_socket_is_not_ipv6_only() {
        let addr = "[::]:0".parse().expect("valid IPv6 wildcard address");
        let socket = socket(addr, Type::STREAM, Protocol::TCP).expect("create dual-stack socket");
        assert!(!socket.only_v6().expect("read IPV6_V6ONLY"));
    }
}
