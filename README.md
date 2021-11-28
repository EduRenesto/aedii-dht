# dht

Implementação extremamente simples da [BEP-0005](http://bittorrent.org/beps/bep_0005.html).

### Rodando

O código foi escrito em Rust, então para rodar será necessário ter um
toolchain Rust (nightly) instalado e configurado. Uma maneira fácil de
fazer isso é usando o [rustup.rs](https://rustup.rs).

Depois de instalado, basta entrar na pasta do projeto e rodar o
seguinte comando:

```shell
$ cargo run
```

### Arquitetura

Para simular uma rede real, aproveitei a framework
[Actix](https://docs.rs/actix), que é uma implementação do Actor Model
(isso dá pra colocar no relatório, fica bonitinho :p). Cada nó é
implementado por um ator (implementação no arquivo `src/node.rs`), e
as comunicações que se dariam por TCP e UDP são feitas utilizando as
mensagens e mailboxes do actor model.

No arquivo `src/main.rs`, existe um cenário de teste simples que
provavelmente vai evoluir. Cito:

```rust
// Add node 1 to the routing list of node 0
addrs[0].1.send(CtrlAddRoute(addrs[1].0, addrs[1].1.clone())).await.unwrap();

// Add node 2 to the routing list of node 1
addrs[1].1.send(CtrlAddRoute(addrs[2].0, addrs[2].1.clone())).await.unwrap();

// Add a peer to node 2. 
addrs[2].1.send(CtrlAddPeer(addrs[1].0, addrs[4].1.clone())).await.unwrap();

// Download file!
addrs[0].1.send(CtrlDownloadFile(addrs[1].0)).await.unwrap();
```

Esse exemplo monta uma rede simples, onde o arquivo cujo `info_hash` é
igual ao `node_id` do nó 2 está disponível no nó 4. O roteamento é
feito tal que o ńo 0 conhece apenas o nó 1, e o nó 1 conhece apenas o
ńo 2. Seguindo a arquitetura da Mainline DHT, o nó 2 é um dos
responsáveis por manter a lista dos peers do arquivo, e de fato o nó 0
finalmente o encontra e depois tenta fazer o "download" do arquivo a
partir do peer encontrado.

Os logs gerados mostram bem os *hops*:

```
 DEBUG dht::node > Node 6D470C47217F68B1F7C3BB99C04753EC received get_peers(F74196C56AC16BFE0394FD3E0CB5501B) from node 6D62D178AFB674BCDBF0344E52706ADB
 DEBUG dht::node > Node F74196C56AC16BFE0394FD3E0CB5501B received get_peers(F74196C56AC16BFE0394FD3E0CB5501B) from node 6D62D178AFB674BCDBF0344E52706ADB
 DEBUG dht::control_methods > Node 6D62D178AFB674BCDBF0344E52706ADB found peers to download F74196C56AC16BFE0394FD3E0CB5501B from
 DEBUG dht::node            > Node 445F178ED7B8968E6C01FC68DA0C6982 received announce_peer(F74196C56AC16BFE0394FD3E0CB5501B) from node 6D62D178AFB674BCDBF0344E52706ADB
```
