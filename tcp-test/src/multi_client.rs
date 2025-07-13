use tokio::{io::AsyncWriteExt, net::TcpStream, time::sleep, time::Duration};

#[tokio::main]
async fn main() {
    println!("Starting multi-client test...");

    let mut handles = vec![];

    // Create 5 clients concurrently
    for i in 1..=5 {
        let handle = tokio::spawn(async move {
            println!("Client {} attempting to connect...", i);

            match TcpStream::connect("127.0.0.1:9000").await {
                Ok(mut conn) => {
                    println!("Client {} connected successfully!", i);

                    // Send a handshake packet
                    let handshake = vec![9u8, 1, 0, 0, 0, 0, 0, 0, 0, 0];
                    if let Err(e) = conn.write_all(&handshake).await {
                        println!("Client {} failed to write handshake: {}", i, e);
                        return;
                    }

                    println!("Client {} sent handshake packet", i);

                    // Keep connection alive for a bit
                    sleep(Duration::from_secs(3)).await;

                    // Send another packet
                    let packet2 = vec![5u8, 2, 0, 0, 0];
                    if let Err(e) = conn.write_all(&packet2).await {
                        println!("Client {} failed to write second packet: {}", i, e);
                        return;
                    }

                    println!("Client {} sent second packet", i);

                    // Keep connection alive a bit longer
                    sleep(Duration::from_secs(2)).await;

                    println!("Client {} disconnecting", i);
                }
                Err(e) => {
                    println!("Client {} failed to connect: {}", i, e);
                }
            }
        });

        handles.push(handle);

        // Small delay between connection attempts
        sleep(Duration::from_millis(100)).await;
    }

    // Wait for all clients to finish
    for handle in handles {
        if let Err(e) = handle.await {
            println!("Client task failed: {}", e);
        }
    }

    println!("All clients finished");
}
