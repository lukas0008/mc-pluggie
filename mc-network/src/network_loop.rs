use std::{
    io::{Read, Write},
    sync::Arc,
};

use abi_stable::external_types::crossbeam_channel::RReceiver;
use dashmap::DashMap;
use pluggie::pluggie_context::{EventSender, PluggieCtx};

use crate::{
    SERVER_TOKEN, WAKE_TOKEN,
    client::Client,
    client_id::ClientId,
    client_mode::ClientMode,
    events::{NewConnectionEvent, RawPacketEvent},
    network_context::NetworkTask,
};

pub(crate) fn network_loop(
    (ctx, mut poll, task_receiver, new_connection, raw_packet, connections): (
        PluggieCtx,
        mio::Poll,
        RReceiver<NetworkTask>,
        EventSender<NewConnectionEvent>,
        EventSender<RawPacketEvent>,
        Arc<DashMap<mio::Token, Client>>,
    ),
) {
    let mut events = mio::Events::with_capacity(128);
    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut server = mio::net::TcpListener::bind(addr).unwrap();

    poll.registry()
        .register(&mut server, SERVER_TOKEN, mio::Interest::READABLE)
        .unwrap();

    let mut conn_id = 2usize;

    ctx.info(&format!("Listening on {}", addr));
    loop {
        poll.poll(&mut events, None).unwrap(); // Blocks until an event occurs

        let wake = || {
            while let Ok(task) = task_receiver.try_recv() {
                match task {
                    NetworkTask::SendPacket(client_id, mut data) => {
                        if let Some(mut client) = connections.get_mut(&client_id.as_token()) {
                            // dbg!("sending: ", String::from_utf8_lossy(&data));
                            client.to_write.append(&mut data);
                            if client.currently_writable {
                                client.conn.nodelay().unwrap();
                                let client = &mut *client;
                                let written = client.conn.write(&client.to_write).unwrap();
                                client.to_write.drain(..written);
                                client.currently_writable = false;
                            }
                        }
                    }
                    NetworkTask::CloseClient(client_id) => {
                        dbg!();
                        if let Some(mut client) = connections.get_mut(&client_id.as_token()) {
                            client.conn.shutdown(std::net::Shutdown::Both).unwrap();
                            poll.registry().deregister(&mut client.conn).unwrap();
                            ctx.info(&format!("Client {} closed", client_id));
                        }
                        connections.remove(&client_id.as_token());
                    }
                }
            }
        };
        for event in events.iter() {
            match event.token() {
                WAKE_TOKEN => {
                    wake();
                }
                SERVER_TOKEN => loop {
                    let (mut conn, addr) = match server.accept().map_err(|e| (e.kind(), e)) {
                        Ok(v) => v,
                        Err((std::io::ErrorKind::WouldBlock, _)) => break,
                        Err((_, e)) => {
                            ctx.error(&format!("Failed to accept connection: {}", e));
                            continue;
                        }
                    };
                    let id = conn_id;
                    conn_id += 1;
                    ctx.info(&format!("Accepted connection {}", id));
                    conn.nodelay().unwrap();
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
                    new_connection.call(&NewConnectionEvent(ClientId(id)));
                },
                token => {
                    let mut raw_packet_events = Vec::new();
                    if let Some(mut client) = connections.get_mut(&token) {
                        if event.is_readable() {
                            loop {
                                let mut byte_buf = [0; 4096];
                                let bytes_read = match client
                                    .conn
                                    .read(&mut byte_buf)
                                    .map_err(|e| (e.kind(), e))
                                {
                                    Ok(bytes_read) => bytes_read,
                                    Err((std::io::ErrorKind::WouldBlock, _)) => {
                                        break;
                                    }
                                    Err((_, err)) => {
                                        ctx.error(&format!(
                                            "Error reading from client {}: {}",
                                            client.id, err
                                        ));
                                        break;
                                    }
                                };
                                if bytes_read == 0 {
                                    break;
                                }
                                let events = client.update_received_bytes(&byte_buf[..bytes_read]);
                                raw_packet_events.extend(events);
                            }
                        } else if event.is_writable() {
                            client.conn.nodelay().unwrap();

                            if client.to_write.len() > 0 {
                                let client = &mut *client;
                                let written = client.conn.write(&client.to_write).unwrap();
                                client.to_write.drain(..written);
                                client.currently_writable = false;
                            } else {
                                client.currently_writable = true;
                            }
                        }
                    }
                    for event in raw_packet_events {
                        let mode = { connections.get(&token).unwrap().mode };
                        raw_packet.call(&RawPacketEvent {
                            client_id: ClientId(token.0),
                            client_mode: mode,
                            data: event,
                        });
                    }
                }
            }
        }
    }
}
