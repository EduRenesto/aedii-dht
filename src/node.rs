use std::collections::HashMap;

use actix::prelude::*;

use crate::dht_methods::*;
use crate::routing_table::RoutingTable;

use log::*;

pub struct Node {
    pub node_id: u128,
    pub routing_table: RoutingTable,
    pub peers: HashMap<u128, Vec<Addr<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        let node_id = rand::random();
        Node {
            node_id,
            routing_table: RoutingTable::new(node_id),
            peers: HashMap::new(),
        }
    }
}

impl Actor for Node {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("node {:16X} started", self.node_id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("node {:16X} stopped", self.node_id);
    }
}

impl Handler<PingMsg> for Node {
    type Result = NodeId;

    fn handle(&mut self, PingMsg(id): PingMsg, _: &mut Self::Context) -> Self::Result {
        debug!(
            "Node {:16X} received ping() from node {:16X}",
            self.node_id, id,
        );

        self.node_id.into()
    }
}

impl Handler<FindNode> for Node {
    type Result = FindNodeResponse;

    fn handle(&mut self, FindNode(id, target): FindNode, _ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Node {:16X} received find_node({:16X}) from node {:16X}",
            self.node_id, target, id,
        );

        match self.routing_table.find_exact(target) {
            Some(entry) => FindNodeResponse::Exact(entry.into()),
            None => FindNodeResponse::Closest(
                self.routing_table
                    .find_closest(target)
                    .iter()
                    .cloned()
                    .map(|entry| entry.into())
                    .collect(),
            ),
        }
    }
}

impl Handler<GetPeers> for Node {
    type Result = GetPeersResult;

    fn handle(
        &mut self,
        GetPeers(id, info_hash): GetPeers,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        debug!(
            "Node {:16X} received get_peers({:16X}) from node {:16X}",
            self.node_id, info_hash, id,
        );

        match self.peers.get(&info_hash) {
            Some(peers) => {
                GetPeersResult::Values(peers.iter().cloned().map(|peer| PeerInfo(peer)).collect())
            }
            None => GetPeersResult::Nodes(
                self.routing_table
                    .find_closest(info_hash)
                    .iter()
                    .cloned()
                    .map(|entry| entry.into())
                    .collect(),
            ),
        }
    }
}

impl Handler<AnnouncePeer> for Node {
    type Result = ();

    fn handle(
        &mut self,
        AnnouncePeer(addr, node_id, info_hash): AnnouncePeer,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        debug!(
            "Node {:16X} received announce_peer({:16X}) from node {:16X}",
            self.node_id, info_hash, node_id,
        );
        self.peers.entry(info_hash).or_insert(Vec::new()).push(addr);
    }
}
