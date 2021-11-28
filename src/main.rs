#![feature(linked_list_cursors)]
#![feature(drain_filter)]

//! # Implementação básica do BEP-0005
//!
//! Escrito como o projeto da disciplina Algoritmos e Estruturas de Dados II,
//! ministrado na UFABC pelo Prof. Dr. Carlo Kleber.
//!
//! Esse projeto implementa uma DHT básica em memória, baseada no BEP-0005
//! do BitTorrent.

use actix::prelude::*;

pub(crate) mod bucket;
pub(crate) mod control_methods;
pub(crate) mod dht_methods;
pub(crate) mod extra_methods;
pub(crate) mod node;
pub(crate) mod routing_table;

#[cfg(test)]
mod tests;

use control_methods::*;
use node::*;

const TOTAL_NODES: usize = 8;

#[actix_rt::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    pretty_env_logger::init();

    let addrs = (0..TOTAL_NODES)
        .map(|_| {
            let node = Node::new();
            let node_id = node.node_id;

            (node_id, node.start())
        })
        .collect::<Vec<_>>();

    /*
    let addr = addrs[0].send(QueryMsg(rand::random()))
    .await
    .expect("auauauu")
    .unwrap();

    assert!(addrs[0] == addr);
    */

    // Ping
    addrs[0].1.send(CtrlPing(addrs[1].1.clone())).await.unwrap();

    // Add node 1 to the routing list of node 0
    addrs[0]
        .1
        .send(CtrlAddRoute(addrs[1].0, addrs[1].1.clone()))
        .await
        .unwrap();

    // Add node 2 to the routing list of node 1
    addrs[1]
        .1
        .send(CtrlAddRoute(addrs[2].0, addrs[2].1.clone()))
        .await
        .unwrap();

    // Add node 3 to the routing list of node 0
    addrs[0]
        .1
        .send(CtrlAddRoute(addrs[3].0, addrs[3].1.clone()))
        .await
        .unwrap();

    // Add node 5 to the routing list of node 3
    addrs[3]
        .1
        .send(CtrlAddRoute(addrs[5].0, addrs[5].1.clone()))
        .await
        .unwrap();

    // Add node 2 to the routing list of node 5
    addrs[5]
        .1
        .send(CtrlAddRoute(addrs[2].0, addrs[2].1.clone()))
        .await
        .unwrap();

    // Add a peer to node 2.
    addrs[2]
        .1
        .send(CtrlAddPeer(addrs[2].0, addrs[4].1.clone()))
        .await
        .unwrap();

    // Download file!
    addrs[0].1.send(CtrlDownloadFile(addrs[2].0)).await.unwrap();

    System::current().stop();
}
