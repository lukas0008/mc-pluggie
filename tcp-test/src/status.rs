use std::time::Duration;

use mclib_protocol::{
    packet::PacketSerialize,
    server::{handshake::SHandshakePacket, status::SStatusRequest},
    varint::Varint,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    // let varint = Varint::new(136);
    // dbg!(varint.to_bytes());
    // dbg!(Varint::parse(&varint.to_bytes()));
    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();
    dbg!();
    let blurp = SHandshakePacket {
        intent: Varint::new(1),
        protocol_version: Varint::new(754),
        server_address: "yogurt".to_string(),
        server_port: 25565,
    }
    .serialize_packet();
    let mut pack = Vec::new();
    pack.extend(Varint::new(blurp.len() as i32 + 1).to_bytes());
    pack.push(0);
    pack.extend(blurp);
    stream.write_all(&pack).await.unwrap();
    let blurp = SStatusRequest {}.serialize_packet();
    let mut pack = Vec::new();
    pack.extend(Varint::new(blurp.len() as i32 + 1).to_bytes());
    pack.push(0);
    pack.extend(blurp);
    stream.write_all(&pack).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let mut buf = [0u8; 2048];
    let a = stream.read(&mut buf).await.unwrap();
    dbg!(&buf[..a]);
}
