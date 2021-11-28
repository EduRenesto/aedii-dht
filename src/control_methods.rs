//! Métodos de controle para os nós da DHT.
//! Utilizados apenas para manipular o estado da DHT,
//! para efetuarmos os testes.

use std::collections::BinaryHeap;

use actix::prelude::*;

use log::*;

use crate::{
    dht_methods::{AnnouncePeer, GetPeers, GetPeersResult, NodeInfo, PingMsg},
    node::Node,
    routing_table::RoutingTableEntry,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct CtrlPing(pub Addr<Node>);

impl Handler<CtrlPing> for Node {
    type Result = ();

    fn handle(&mut self, CtrlPing(target): CtrlPing, _ctx: &mut Self::Context) -> Self::Result {
        target.do_send(PingMsg(self.node_id));
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CtrlAddRoute(pub u128, pub Addr<Node>);

impl Handler<CtrlAddRoute> for Node {
    type Result = ();

    fn handle(
        &mut self,
        CtrlAddRoute(node_id, address): CtrlAddRoute,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.routing_table
            .insert(RoutingTableEntry { node_id, address })
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CtrlAddPeer(pub u128, pub Addr<Node>);

impl Handler<CtrlAddPeer> for Node {
    type Result = ();

    fn handle(
        &mut self,
        CtrlAddPeer(info_hash, addr): CtrlAddPeer,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.peers.entry(info_hash).or_insert(Vec::new()).push(addr);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CtrlDownloadFile(pub u128);

impl Handler<CtrlDownloadFile> for Node {
    type Result = ResponseFuture<()>;

    fn handle(
        &mut self,
        CtrlDownloadFile(info_hash): CtrlDownloadFile,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let node_id = self.node_id;
        let closest = self.routing_table.find_closest(info_hash);
        let address = ctx.address();

        Box::pin(async move {
            let mut should_visit = BinaryHeap::<(u128, NodeInfo)>::new();

            for entry in closest {
                let dist = node_id ^ entry.node_id;
                should_visit.push((dist, NodeInfo(entry.node_id, entry.address)));
            }

            while let Some(closest) = should_visit.pop() {
                let res = closest
                    .1
                     .1
                    .send(GetPeers(node_id, info_hash))
                    .await
                    .unwrap();

                match res {
                    GetPeersResult::Values(peers) => {
                        // TODO do something with the peers
                        debug!(
                            "Node {:16X} found peers to download {:16X} from",
                            node_id, info_hash,
                        );

                        for peer in peers {
                            peer.0
                                .send(AnnouncePeer(address.clone(), node_id, info_hash))
                                .await
                                .unwrap();
                        }

                        break;
                    }
                    GetPeersResult::Nodes(nodes) => {
                        for new_node in nodes {
                            let dist = node_id ^ new_node.0;
                            should_visit.push((dist, new_node));
                        }
                    }
                }
            }
        })
    }
}
