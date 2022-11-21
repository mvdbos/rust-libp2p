use clap::Parser;
use tokio::sync::mpsc;

use crate::mychat_behaviour::MyChatNetworkBehaviour;
use network_client::NetworkClient;

use crate::cli::{
    keypair_from_peer_no, listen_address_with_peer_id, local_listen_address_from_peer_no, Cli,
};
use crate::network::network_builder::NetworkBuilder;

mod cli;
mod network_client;
mod network;
mod mychat_direct_message_protocol;
mod mychat_behaviour;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let (notification_tx, notification_rx) = mpsc::unbounded_channel();
    let (instruction_tx, instruction_rx) = mpsc::unbounded_channel();

    let keypair = keypair_from_peer_no(cli.peer_no);

    let behaviour = MyChatNetworkBehaviour::new(
        &keypair,
        cli.bootstrap_peer_no
            .map(|peer_no| vec![listen_address_with_peer_id(peer_no)]),
    )?;

    let mut network_builder =
        NetworkBuilder::new(keypair, instruction_rx, notification_tx, behaviour)?;

    network_builder =
        network_builder.listen_address(local_listen_address_from_peer_no(cli.peer_no));

    let network = network_builder.build()?;
    let peer_id = *network.peer_id();

    let client = NetworkClient::new(peer_id, instruction_tx, notification_rx);

    tokio::spawn(network.run());
    let client_handle = tokio::spawn(client.run());

    client_handle.await??;

    Ok(())
}
