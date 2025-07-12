use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    let mut conn = TcpStream::connect("127.0.0.1:9000").await.unwrap();

    conn.write_all(&[2u8, b'h', b'i']).await.unwrap();
}
