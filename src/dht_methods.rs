//! Declarações das mensagens relevantes da BEP-0005.
//! Documentação foi extraída diretamente da especificação,
//! disponível (aqui)[http://bittorrent.org/beps/bep_0005.html].

use actix::prelude::*;

use crate::{node::Node, routing_table::RoutingTableEntry};

/// Essa struct existe só por detalhe de implementação.
#[derive(MessageResponse)]
pub struct NodeId(u128);

impl From<u128> for NodeId {
    fn from(id: u128) -> Self {
        NodeId(id)
    }
}

/// Contact information for peers is encoded as a 6-byte string. Also
/// known as "Compact IP-address/port info" the 4-byte IP address is
/// in network byte order with the 2 byte port in network byte order
/// concatenated onto the end.
pub struct PeerInfo(pub Addr<Node>);

impl From<RoutingTableEntry> for PeerInfo {
    fn from(entry: RoutingTableEntry) -> Self {
        PeerInfo(entry.address)
    }
}

/// Contact information for nodes is encoded as a 26-byte string. Also
/// known as "Compact node info" the 20-byte Node ID in network byte
/// order has the compact IP-address/port info concatenated to the
/// end.
#[derive(PartialEq, Eq)]
pub struct NodeInfo(pub u128, pub Addr<Node>);

impl PartialOrd for NodeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for NodeInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl From<RoutingTableEntry> for NodeInfo {
    fn from(entry: RoutingTableEntry) -> Self {
        NodeInfo(entry.node_id, entry.address)
    }
}

/// The most basic query is a ping. "q" = "ping" A ping query has a
/// single argument, "id" the value is a 20-byte string containing the
/// senders node ID in network byte order. The appropriate response to
/// a ping has a single key "id" containing the node ID of the
/// responding node.
#[derive(Message)]
#[rtype(result = "NodeId")]
pub struct PingMsg(pub u128);

/// Find node is used to find the contact information for a node given
/// its ID. "q" == "find_node" A find_node query has two arguments,
/// "id" containing the node ID of the querying node, and "target"
/// containing the ID of the node sought by the queryer. When a node
/// receives a find_node query, it should respond with a key "nodes"
/// and value of a string containing the compact node info for the
/// target node or the K (8) closest good nodes in its own routing
/// table.
#[derive(Message)]
#[rtype(result = "FindNodeResponse")]
pub struct FindNode(pub u128, pub u128);

#[derive(MessageResponse)]
pub enum FindNodeResponse {
    Exact(NodeInfo),
    Closest(Vec<NodeInfo>),
}

/// Get peers associated with a torrent infohash. "q" = "get_peers" A
/// get_peers query has two arguments, "id" containing the node ID of
/// the querying node, and "info_hash" containing the infohash of the
/// torrent. If the queried node has peers for the infohash, they are
/// returned in a key "values" as a list of strings. Each string
/// containing "compact" format peer information for a single peer. If
/// the queried node has no peers for the infohash, a key "nodes" is
/// returned containing the K nodes in the queried nodes routing table
/// closest to the infohash supplied in the query.
#[derive(Message)]
#[rtype(result = "GetPeersResult")]
pub struct GetPeers(pub u128, pub u128);

#[derive(MessageResponse)]
pub enum GetPeersResult {
    Values(Vec<PeerInfo>),
    Nodes(Vec<NodeInfo>),
}

/// Announce that the peer, controlling the querying node, is
/// downloading a torrent on a port. announce_peer has four arguments:
/// "id" containing the node ID of the querying node, "info_hash"
/// containing the infohash of the torrent, "port" containing the port
/// as an integer, and the "token" received in response to a previous
/// get_peers query.
#[derive(Message)]
#[rtype(result = "()")]
pub struct AnnouncePeer(pub Addr<Node>, pub u128, pub u128);
