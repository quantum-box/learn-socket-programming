//! # tcp server
//!
//! ## 処理の流れ
//!
//! TcoListener::bind()でソケットを生成し、
//! accept()でクライアントからの接続を待ち受ける。
//!
//! accept()で接続があると、TcpStreamを返す。
//! TcpStreamはRead, Writeトレイトを実装しているので、
//! read()でクライアントからの入力を待ち受け、
//! write()でクライアントにデータを送信する。
//!
//! このサーバは、クライアントからの入力をそのまま返却する。
//!
//! ## ソケットの役割
//!
//! [1]で生成されるlistenerの役割は、
//! クライアントからのコネクション確立要求を待ち受けること。
//! それが届いたら、3 way handshakeを行い、
//! コネクション確立済みのソケットがカーネル内部のキューに生成される。
//!
//! `accept()` は、コネクション確立済みのソケットを返却することであり、
//! もし、コネクション確立済みのソケットがなければ、
//! クライアントからの接続要求があるまでブロックする。
//!
//! 実際のデータのやり取りはsteamを通して行われる。
//!
//! listenerとstreamは、同じソケットだが、役割が違う
//! listenerのようなソケットをリスニングソケット
//! streamのようなソケットを接続済みソケットと呼ぶこととする
//! (サーバーソケット、クライアントソケットという呼び方もあるらしい)
//!
//! ## handler()の役割
//! `stream.read()` は、steamにデータが到着するまでブロックする。
//! `read()` は `EOF` が到着すると（今回は通信が切断されると）、0を返却する。
//! `stream` がdropされると、コネクションが切断される。
//!
//! run server with:
//!
//! ```shell
//! ❯ cargo run tcp server 127.0.0.1:33333
//! ❯ cargo run tcp server 127.0.0.1:33333
//!    Compiling socket-programming v0.1.0 (/Users/takanorifukuyama/git/github.com/takanorifukuyama/socket-programming)
//!     Finished dev [unoptimized + debuginfo] target(s) in 1.09s
//!      Running `target/debug/socket-programming tcp server '127.0.0.1:33333'`
//! [2023-09-12T06:43:34Z DEBUG socket_programming::tcp_server] Handling data from 127.0.0.1:52937
//! aaaaaaaaaaaaaa
//! ```
//!
//! connect to server with:
//! ```shell
//! ❯ telnet 127.0.0.1 33333
//! Trying 127.0.0.1...
//! Connected to localhost.
//! Escape character is '^]'.
//! aaaaaaaaaaaaaa
//! aaaaaaaaaaaaaa
//! ```

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

/// 指定のソケットアドレスで接続を待ち受ける
pub fn serve(address: &str) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (stream, _) = listener.accept()?; // [1]

        // スレッドを立ち上げて接続に対処する
        // これにより、複数のクライアントと同時に通信できる
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}

/// クライアントからの入力を待ち受け、受信したら同じものを返却する
fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        println!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}
