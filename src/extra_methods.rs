//! Mensagens "extra", enviadas dos próprios nós para eles mesmos.
//!
//! Mais uma vez, as mensagens aqui são apenas detalhes de implementação.
//! Quando um ator (nó) executa uma ação assíncrona, por razões de lifetimes,
//! ele não pode mais acessar seu próprio estado, porque isso causaria um
//! `move` do `self`. Então, essas mensagens existem para "defer" as ações
//! que os atores fariam em si mesmos. Dessa maneira, podemos criar handlers
//! síncronos para essas mensagens, e então podemos mutar o estado interno do nó.

use actix::prelude::*;

use crate::{node::Node, routing_table::RoutingTableEntry};

/// Mensagem para adicionar uma rota à lista de roteamento do nó atual.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ExtraAddRoute {
    pub node_id: u128,
    pub addr: Addr<Node>,
}

impl Handler<ExtraAddRoute> for Node {
    type Result = ();

    fn handle(&mut self, msg: ExtraAddRoute, _ctx: &mut Self::Context) -> Self::Result {
        let entry = RoutingTableEntry {
            node_id: msg.node_id,
            address: msg.addr,
        };
        self.routing_table.insert(entry);
    }
}

/// Mensagem para remover uma rota à lista de roteamento do nó atual.
/// Em termos da Kademlia, marca um nó como `not good`.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ExtraRemoveRoute {
    pub node_id: u128,
}

impl Handler<ExtraRemoveRoute> for Node {
    type Result = ();

    fn handle(&mut self, msg: ExtraRemoveRoute, _ctx: &mut Self::Context) -> Self::Result {
        self.routing_table.remove(msg.node_id);
    }
}
