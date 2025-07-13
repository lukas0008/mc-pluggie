use std::{
    collections::HashMap,
    io::{Read, Write},
};

use abi_stable::external_types::crossbeam_channel::RReceiver;
use pluggie::pluggie_context::{EventSender, PluggieCtx};

use crate::{
    client::Client,
    client_id::ClientId,
    client_mode::ClientMode,
    events::{NewConnectionEvent, RawPacketEvent},
    network_context::NetworkTask,
    SERVER_TOKEN, WAKE_TOKEN,
};

pub(crate) fn debug_network_loop(
    (ctx, mut poll, task_receiver, new_connection, raw_packet): (
        PluggieCtx,
        mio::Poll,
        RReceiver<NetworkTask>,
        EventSender<NewConnectionEvent>,
        EventSender<RawPacketEvent>,
    ),
) {
    let mut events = mio::Events::with_capacity(128);
    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut server = mio::net::TcpListener::bind(addr).unwrap();

    poll.registry()
        .register(&mut server, SERVER_TOKEN, mio::Interest::READABLE)
        .unwrap();

    let mut connections: HashMap<mio::Token, Client> = HashMap::new();
    let mut conn_id = 2usize;

    ctx.info(&format!("Listening on {}", addr));

    let mut loop_iteration = 0;

    loop {
        loop_iteration += 1;
        ctx.info(&format!("=== Poll iteration {} ===", loop_iteration));

        poll.poll(&mut events, None).unwrap(); // Blocks until an event occurs

        ctx.info(&format!("Got {} events this iteration", events.len()));

        for (event_index, event) in events.iter().enumerate() {
            ctx.info(&format!(
                "Processing event {}/{}: token={:?}, readable={}, writable={}",
                event_index + 1,
                events.len(),
                event.token(),
                event.is_readable(),
                event.is_writable()
            ));

            match event.token() {
                WAKE_TOKEN => {
                    ctx.info("Processing WAKE_TOKEN");
                    while let Ok(task) = task_receiver.recv() {
                        match task {
                            NetworkTask::SendPacket(client_id, mut data) => {
                                ctx.info(&format!("Sending packet to client {}", client_id.0));
                                if let Some(client) = connections.get_mut(&client_id.as_token()) {
                                    client.to_write.append(&mut data);
                                    if client.currently_writable {
                                        let written = client.conn.write(&client.to_write).unwrap();
                                        client.to_write.drain(..written);
                                        client.currently_writable = false;
                                    }
                                }
                            }
                        }
                    }
                    ctx.info("Finished processing WAKE_TOKEN");
                }
                SERVER_TOKEN => {
                    ctx.info("Processing SERVER_TOKEN (new connections)");
                    let mut accepted_count = 0;
                    loop {
                        let id = conn_id;
                        conn_id += 1;
                        let (mut conn, addr) = match server.accept().map_err(|e| (e.kind(), e)) {
                            Ok(v) => v,
                            Err((std::io::ErrorKind::WouldBlock, _)) => {
                                ctx.info(&format!(
                                    "No more connections to accept (accepted {} this round)",
                                    accepted_count
                                ));
                                break;
                            }
                            Err((_, e)) => {
                                ctx.error(&format!("Failed to accept connection: {}", e));
                                continue;
                            }
                        };
                        accepted_count += 1;
                        ctx.info(&format!("Accepted connection {} from {}", id, addr));

                        poll.registry()
                            .register(
                                &mut conn,
                                mio::Token(id),
                                mio::Interest::READABLE | mio::Interest::WRITABLE,
                            )
                            .unwrap();

                        connections.insert(
                            mio::Token(id),
                            Client {
                                id: ClientId(id),
                                addr,
                                conn,
                                currently_writable: false,
                                to_write: Vec::new(),
                                read_buffer: Vec::new(),
                                mode: ClientMode::Handshake,
                            },
                        );

                        ctx.info(&format!("Registered client {} with mio", id));
                        new_connection.call(&NewConnectionEvent(ClientId(id)));
                    }
                }
                token => {
                    ctx.info(&format!("Processing client token {:?}", token));

                    if let Some(client) = connections.get_mut(&token) {
                        if event.is_readable() {
                            ctx.info(&format!("Client {} is readable", client.id.0));
                            let mut byte_buf = [0; 4096];

                            match client.conn.read(&mut byte_buf) {
                                Ok(0) => {
                                    ctx.info(&format!(
                                        "Client {} disconnected (read 0 bytes)",
                                        client.id.0
                                    ));
                                    // Remove the client from connections
                                    connections.remove(&token);
                                    continue;
                                }
                                Ok(bytes_read) => {
                                    ctx.info(&format!(
                                        "Client {} sent {} bytes",
                                        client.id.0, bytes_read
                                    ));
                                    client.update_received_bytes(
                                        &byte_buf[..bytes_read],
                                        &raw_packet,
                                    );
                                }
                                Err(e) => {
                                    ctx.error(&format!(
                                        "Error reading from client {}: {}",
                                        client.id.0, e
                                    ));
                                    connections.remove(&token);
                                    continue;
                                }
                            }
                        }

                        if event.is_writable() {
                            ctx.info(&format!("Client {} is writable", client.id.0));
                            if client.to_write.len() > 0 {
                                match client.conn.write(&client.to_write) {
                                    Ok(written) => {
                                        ctx.info(&format!(
                                            "Wrote {} bytes to client {}",
                                            written, client.id.0
                                        ));
                                        client.to_write.drain(..written);
                                        client.currently_writable = false;
                                    }
                                    Err(e) => {
                                        ctx.error(&format!(
                                            "Error writing to client {}: {}",
                                            client.id.0, e
                                        ));
                                        connections.remove(&token);
                                        continue;
                                    }
                                }
                            } else {
                                client.currently_writable = true;
                            }
                        }
                    } else {
                        ctx.info(&format!(
                            "Received event for unknown client token {:?}",
                            token
                        ));
                    }
                }
            }
        }

        ctx.info(&format!("Active connections: {}", connections.len()));
        for (token, client) in &connections {
            ctx.info(&format!(
                "  Client {}: token={:?}, buffer_len={}",
                client.id.0,
                token,
                client.read_buffer.len()
            ));
        }
    }
}
