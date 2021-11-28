use actix::prelude::*;

use crate::control_methods::*;
use crate::node::Node;

/// O tamanho em nós da rede.
const TOTAL_NODES: usize = 8;

#[allow(dead_code)]
fn setup_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();
}

async fn setup_dht() -> Vec<(u128, Addr<Node>)> {
    // Constrói os nós
    let addrs = (0..TOTAL_NODES)
        .map(|_| {
            let node = Node::new();
            let node_id = node.node_id;

            (node_id, node.start())
        })
        .collect::<Vec<_>>();

    // Conecta todos os nós ao nó 0.
    // Esse passo imita o "bootstrap" que os clientes
    // de BitTorrent fazem.
    for i in 1..TOTAL_NODES {
        addrs[0]
            .1
            .send(CtrlAddRoute(addrs[i].0, addrs[i].1.clone()))
            .await
            .unwrap();

        addrs[i]
            .1
            .send(CtrlAddRoute(addrs[0].0, addrs[0].1.clone()))
            .await
            .unwrap();
    }

    addrs
}

async fn upload_file(addr: &Addr<Node>, info_hash: u128) {
    addr.send(ControlUploadFile(info_hash)).await.unwrap();
}

async fn download_file(addr: &Addr<Node>, info_hash: u128) -> bool {
    addr.send(CtrlDownloadFile(info_hash)).await.unwrap()
}

async fn kill_node(addr: &Addr<Node>) {
    addr.send(CtrlKillNode).await.unwrap();
}

#[actix_rt::test]
async fn one_hop() {
    // given
    //setup_logger();

    let network = setup_dht().await;

    let info_hash = rand::random();
    upload_file(&network[1].1, info_hash).await;

    // when
    let res = download_file(&network[TOTAL_NODES - 1].1, info_hash).await;

    // then
    assert!(res);
}

#[actix_rt::test]
async fn cant_find() {
    // given
    //setup_logger();

    let network = setup_dht().await;

    let info_hash = rand::random();

    // when
    let res = download_file(&network[TOTAL_NODES - 1].1, info_hash).await;

    // then
    assert!(!res);
}

#[actix_rt::test]
async fn node_removal_resilient() {
    // given
    //setup_logger();

    let network = setup_dht().await;

    let info_hash = rand::random();

    upload_file(&network[1].1, info_hash).await;
    download_file(&network[TOTAL_NODES - 1].1, info_hash).await;
    download_file(&network[TOTAL_NODES - 2].1, info_hash).await;
    download_file(&network[TOTAL_NODES - 3].1, info_hash).await;
    download_file(&network[TOTAL_NODES - 4].1, info_hash).await;
    kill_node(&network[1].1).await;

    // when
    let res = download_file(&network[2].1, info_hash).await;

    // then
    assert!(res);
}

#[actix_rt::test]
async fn node_removal_crash() {
    // given
    //setup_logger();

    let network = setup_dht().await;

    let info_hash = rand::random();

    upload_file(&network[1].1, info_hash).await;
    kill_node(&network[1].1).await;

    // when
    let res = download_file(&network[2].1, info_hash).await;

    // then
    assert!(!res);
}
