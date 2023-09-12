//! # UDP Client
//!
//! ポートにバインドしてデータを送信し、返ってきたデータを読み出してるだけ
//! TCPと違って通信相手が本当に存在するかを確認することなくいきなりデータを送りつける
//!
//! ## バインドするポート
//!
//! `UdpSocket::bind("127.0.0.1:0")` 0番ポート、ポートを指定しないと
//! OSが適当に空いてるポートを割り当ててくれる
//! ポートがすでに使われていますみたいなエラーが防げる
//!
//! ## データの大きさ
//!
//! `let mut buffer = [0u8; 1024];`
//! 1024バイトに適当に決めた
//! 溢れたら以降のデータは破棄

use std::net::UdpSocket;
use std::{io, str};

pub fn communicate(address: &str) -> Result<(), failure::Error> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send_to(input.as_bytes(), address)?;

        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to receive");
        print!(
            "{}",
            str::from_utf8(&buffer).expect("failed to convert to String")
        );
    }
}
