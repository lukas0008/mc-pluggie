use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting multiple client test...");

    // Give the server time to start
    thread::sleep(Duration::from_millis(1000));

    let mut handles = vec![];

    // Create 5 test clients
    for i in 1..=5 {
        let handle = thread::spawn(move || {
            match TcpStream::connect("127.0.0.1:9000") {
                Ok(mut stream) => {
                    println!("Client {} connected successfully", i);

                    // Send some test data
                    let message = format!("Hello from client {}", i);
                    if let Err(e) = stream.write_all(message.as_bytes()) {
                        println!("Client {} failed to write: {}", i, e);
                        return;
                    }

                    // Try to read response
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(0) => println!("Client {} connection closed by server", i),
                        Ok(n) => println!("Client {} received {} bytes", i, n),
                        Err(e) => println!("Client {} read error: {}", i, e),
                    }

                    // Keep connection alive for a bit
                    thread::sleep(Duration::from_secs(2));
                    println!("Client {} disconnecting", i);
                }
                Err(e) => {
                    println!("Client {} failed to connect: {}", i, e);
                }
            }
        });
        handles.push(handle);

        // Small delay between connections
        thread::sleep(Duration::from_millis(100));
    }

    // Wait for all clients to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All clients finished");
}
