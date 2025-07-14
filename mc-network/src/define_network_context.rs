use std::sync::Arc;

use crate::network_context::{NetworkContext, NetworkContextFuncs, NetworkContextImplementation};

pub fn defined_network_context(net_ctx_impl: NetworkContextImplementation) -> NetworkContext {
    let net_ctx_impl1 = net_ctx_impl.clone();
    let net_ctx_impl2 = net_ctx_impl.clone();
    let net_ctx_impl3 = net_ctx_impl.clone();
    let net_ctx_impl4 = net_ctx_impl.clone();
    let net_ctx = NetworkContext(Arc::new(NetworkContextFuncs {
        send_raw_packet: Box::new(move |client_id, packet| {
            net_ctx_impl1.send_raw_packet(client_id, packet);
        }),
        send_packet: Box::new(move |client_id, packet| {
            net_ctx_impl2.send_packet(client_id, packet);
        }),
        close_client: Box::new(move |client_id| {
            net_ctx_impl3.close_client(client_id);
        }),
        switch_client_mode: Box::new(move |client_id, mode| {
            net_ctx_impl4.switch_client_mode(client_id, mode);
        }),
    }));
    net_ctx
}
