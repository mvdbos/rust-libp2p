extern crate core;

use std::fmt::Debug;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc;

use libp2p::futures::StreamExt;
pub use libp2p::PeerId;
use libp2p::Swarm;
use libp2p::swarm::NetworkBehaviour;
use event_handler::EventHandler;
use instruction_handler::InstructionHandler;

pub mod network_builder;
pub mod instruction_handler;
pub mod event_handler;

pub struct Network<TBehaviour>
    where
        TBehaviour: NetworkBehaviour,
{
    instruction_rx: mpsc::UnboundedReceiver<Instruction>,
    notification_tx: mpsc::UnboundedSender<Notification>,
    swarm: Swarm<TBehaviour>,
}

impl<TBehaviour> Network<TBehaviour>
    where
        TBehaviour: NetworkBehaviour + EventHandler + InstructionHandler,
{
    pub fn new(
        instruction_rx: mpsc::UnboundedReceiver<Instruction>,
        notification_tx: mpsc::UnboundedSender<Notification>,
        swarm: Swarm<TBehaviour>,
    ) -> Self {
        Network { instruction_rx, notification_tx, swarm }
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => self.swarm.behaviour_mut().handle_event(&self.notification_tx, event).await,
                Some(instruction) = self.instruction_rx.recv() =>  self.swarm.behaviour_mut().handle_instruction(&self.notification_tx, instruction).await,
                else => {
                    log::warn!("Both swarm and instruction receiver closed. Ending event loop");
                    break

                }
            }
        }
    }

    pub fn peer_id(&self) -> &PeerId {
        self.swarm.local_peer_id()
    }
}

/// An instruction for the network to do something. Most likely to send a message of some sort
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Instruct the network to send a message
    Send {
        destination: Address<PeerId>,
        message: Bytes,
    },
    /// Instruct the network to provide a list of all peers it is aware of
    PeerList,
}

/// A notification from the network to one of its consumers. Either data, or an error
#[derive(Debug)]
pub enum Notification {
    Data(Bytes),
    PeerList(Vec<PeerId>),
    Err(NetworkError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T: Debug + Clone> {
    pub source: PeerId,
    pub body: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Address<A: Debug + Clone> {
    Broadcast,
    Multiple(Vec<A>),
    Single(A),
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Failed to send broadcast: `{reason:?}`")]
    BroadcastError { reason: String },
    #[error("Failed to send request to party `{peer_id:?}`: `{reason:?}`")]
    SendError { peer_id: PeerId, reason: String },
    #[error("Failed to receive complete message from party `{peer_id:?}`: `{reason:?}`")]
    ReceiveError { peer_id: PeerId, reason: String },
}
