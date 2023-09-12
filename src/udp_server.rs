//! # UDP Server
//!
//! UDPはデータがやってきたら返してるだけ
//! コネクションを張らないので、TCPと違ってクライアントの情報を保持する必要がない
//! 一つのソケットが全てのクライアントとの通信を捌く

use std::net::UdpSocket;
use std::str;

pub fn serve(address: &str) -> Result<(), failure::Error> {
    let server_socket = UdpSocket::bind(address)?;
    loop {
        let mut buf = [0u8; 1024];
        let (size, src) = server_socket.recv_from(&mut buf)?;
        debug!("Handling data from {}", src);
        print!("{}", str::from_utf8(&buf[..size])?);
        server_socket.send_to(&buf, src)?;
    }
}
