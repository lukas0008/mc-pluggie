use std::{collections::HashMap, io::Write};

use abi_stable::external_types::crossbeam_channel::RReceiver;
use pluggie::pluggie_context::{EventSender, PluggieCtx};

use crate::{
    Client, NetworkTask, NewConnectionEvent, SERVER_TOKEN, WAKE_TOKEN, client_id::ClientId,
};

pub(crate) fn network_loop(
    (ctx, mut poll, task_receiver, event_sender): (
        PluggieCtx,
        mio::Poll,
        RReceiver<NetworkTask>,
        EventSender<NewConnectionEvent>,
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
                    connections.insert(
                        mio::Token(id),
                        Client {
                            addr,
                            conn,
                            currently_writable: false,
                            to_write: Vec::new(),
                        },
                    );
                    // TODO: do in another thread
                    event_sender.call(&NewConnectionEvent(ClientId(id)));
                }
                token => {
                    if let Some(client) = connections.get_mut(&token) {
                        if event.is_readable() {
                            // let mut b = [0u8];
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
