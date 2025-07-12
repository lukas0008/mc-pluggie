use std::{
    collections::HashMap,
    io::{Read, Write},
};

use abi_stable::external_types::crossbeam_channel::RReceiver;
use mclib_protocol::parse_varint;
use pluggie::pluggie_context::{EventSender, PluggieCtx};

use crate::{
    SERVER_TOKEN, WAKE_TOKEN,
    client::Client,
    client_id::ClientId,
    events::{NewConnectionEvent, RawPacketEvent},
    network_context::NetworkTask,
};

pub(crate) fn network_loop(
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
    loop {
        poll.poll(&mut events, None).unwrap(); // Blocks until an event occurs

        for event in events.iter() {
            dbg!();
            match event.token() {
                WAKE_TOKEN => {
                    while let Ok(task) = task_receiver.recv() {
                        match task {
                            NetworkTask::SendPacket(client_id, mut data) => {
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
                }
                SERVER_TOKEN => {
                    let id = conn_id;
                    conn_id += 1;
                    let (mut conn, addr) = server.accept().unwrap();
                    println!("Accepted connection {}", id);
                    poll.registry()
                        .register(
                            &mut conn,
                            mio::Token(id),
                            mio::Interest::READABLE | mio::Interest::WRITABLE,
                        )
                        .unwrap();
                    let mut read_buffer = Vec::new();
                    read_buffer.resize(4096, 0);
                    connections.insert(
                        mio::Token(id),
                        Client {
                            id: ClientId(id),
                            addr,
                            conn,
                            currently_writable: false,
                            to_write: Vec::new(),
                            read_buffer,
                            read_buffer_bytes: 0,
                        },
                    );
                    new_connection.call(&NewConnectionEvent(ClientId(id)));
                }
                token => {
                    if let Some(client) = connections.get_mut(&token) {
                        if event.is_readable() {
                            let bytes_read = client
                                .conn
                                .read(&mut client.read_buffer[client.read_buffer_bytes..])
                                .unwrap();
                            if bytes_read == 0 {
                                continue;
                            }
                            client.read_buffer_bytes += bytes_read;

                            let mut total_bytes_read = 0usize;
                            let (len, len_size) = if let Some((len, bytes_read)) =
                                parse_varint(&client.read_buffer[..client.read_buffer_bytes])
                            {
                                total_bytes_read += bytes_read as usize;
                                total_bytes_read += len as usize;
                                (len, bytes_read as usize)
                            } else {
                                continue;
                            };

                            if client.read_buffer.len() + len_size < len as usize {
                                client.read_buffer.resize(len as usize, 0);
                                continue;
                            }
                            if client.read_buffer_bytes + len_size < len as usize {
                                continue;
                            }

                            let packet_bytes =
                                &client.read_buffer[len_size..len_size + len as usize];
                            let packet_bytes = Vec::from(packet_bytes);
                            raw_packet.call(&RawPacketEvent {
                                client_id: ClientId(token.0),
                                data: packet_bytes,
                            });
                            client
                                .read_buffer
                                .copy_within(total_bytes_read as usize.., 0);
                            client.read_buffer_bytes -= total_bytes_read as usize;

                            // client.conn.read(&mut b).unwrap();
                        } else if event.is_writable() {
                            if client.to_write.len() > 0 {
                                let written = client.conn.write(&client.to_write).unwrap();
                                client.to_write.drain(..written);
                                client.currently_writable = false;
                            } else {
                                client.currently_writable = true;
                            }
                        }
                    }
                }
            }
        }
    }
}
