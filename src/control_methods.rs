//! Métodos de controle para os nós da DHT.
//! Utilizados apenas para manipular o estado da DHT,
//! para efetuarmos os testes.

use std::collections::{BTreeSet, BinaryHeap};

use actix::prelude::*;

use log::*;

use crate::{
    dht_methods::{
        AnnouncePeer, FindNode, FindNodeResponse, GetPeers, GetPeersResult, NodeInfo, PingMsg,
    },
    extra_methods::{ExtraAddRoute, ExtraRemoveRoute},
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
#[rtype(result = "bool")]
pub struct CtrlDownloadFile(pub u128);

impl Handler<CtrlDownloadFile> for Node {
    type Result = ResponseFuture<bool>;

    fn handle(
        &mut self,
        CtrlDownloadFile(info_hash): CtrlDownloadFile,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let node_id = self.node_id;
        let closest = self.routing_table.find_closest(info_hash);
        let address = ctx.address();

        Box::pin(async move {
            // Interessante: dessa maneira, o algoritmo se comporta mais ou menos
            // como um Dijkstra pela DHT
            let mut should_visit = BinaryHeap::<(u128, NodeInfo)>::new();

            let mut visited = BTreeSet::<u128>::new();

            for entry in closest {
                // Adiciona o nó visitado à routing table
                address.do_send(ExtraAddRoute {
                    node_id: entry.node_id,
                    addr: entry.address.clone(),
                });

                let dist = node_id ^ entry.node_id;
                should_visit.push((dist, NodeInfo(entry.node_id, entry.address)));
            }

            while let Some(closest) = should_visit.pop() {
                if visited.contains(&closest.0) {
                    continue;
                }

                visited.insert(closest.0);

                let res = closest.1 .1.send(GetPeers(node_id, info_hash)).await;

                if res.is_err() {
                    // Na implementação real da DHT, o nó não é removido imediatamente.
                    // Ele é marcado como `unknown`, e se ele falha em responder por 4 vezes,
                    // aí sim ele é removido da routing table.
                    //
                    // Como nossos testes são em pequena escala, e só queremos validar
                    // o comportamento geral da rede, optamos por remover da routing table
                    // logo na primeira falha de resposta.
                    address.do_send(ExtraRemoveRoute { node_id: closest.0 });

                    continue;
                }

                let res = res.unwrap();

                match res {
                    GetPeersResult::Values(peers) => {
                        info!(
                            "Node {:16X} found peers to download {:16X} from",
                            node_id, info_hash,
                        );
                        info!("... courtesy of node {:16X}", closest.1 .0);

                        let mut successes = Vec::new();

                        for peer in peers {
                            let res = peer
                                .0
                                .send(AnnouncePeer(address.clone(), node_id, info_hash))
                                .await;

                            info!("\t- Good? {}", res.is_ok());

                            successes.push(res.is_ok());
                        }

                        return successes.iter().any(|b| *b);
                    }
                    GetPeersResult::Nodes(nodes) => {
                        for new_node in nodes {
                            let dist = node_id ^ new_node.0;
                            should_visit.push((dist, new_node));
                        }
                    }
                }
            }
            return false;
        })
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ControlUploadFile(pub u128);

impl Handler<ControlUploadFile> for Node {
    type Result = ResponseFuture<()>;

    fn handle(
        &mut self,
        ControlUploadFile(info_hash): ControlUploadFile,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let node_id = self.node_id;
        let mut closest = self.routing_table.find_closest(info_hash);
        closest.sort();
        let address = ctx.address();

        Box::pin(async move {
            let res = closest[0]
                .address
                .clone()
                .send(FindNode(node_id, info_hash))
                .await
                .unwrap();

            // Só um hop....
            if let FindNodeResponse::Closest(mut nodes) = res {
                nodes.sort();

                let NodeInfo(ref _target_node_id, ref target_addr) = nodes[0];

                target_addr.do_send(AnnouncePeer(address.clone(), node_id, info_hash));
                //for NodeInfo(target_node_id, target_addr) in nodes {
                //    if target_node_id == node_id {
                //        continue;
                //    }

                //    target_addr.do_send(AnnouncePeer(address.clone(), node_id, info_hash));
                //}
            }
        })
    }
}

#[derive(Message)]
#[rtype(response = "()")]
pub struct CtrlKillNode;

impl Handler<CtrlKillNode> for Node {
    type Result = ();

    fn handle(&mut self, _: CtrlKillNode, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}
