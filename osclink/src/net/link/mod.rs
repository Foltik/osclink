use libp2p::{
    core::{connection::ConnectionId, upgrade::ReadOneError, Multiaddr, PeerId, UpgradeError},
    swarm::{
        NetworkBehaviour, NetworkBehaviourAction, NotifyHandler, PollParameters, ProtocolsHandler,
        ProtocolsHandlerUpgrErr,
    },
};

use futures::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::time::Duration;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Interval;

mod handler;
mod protocol;
mod structs;

use handler::{LinkHandler, LinkHandlerEvent};

#[derive(Clone, Debug)]
pub struct LinkInfo {
    port: u16,
    name: String,
}

#[derive(Debug)]
pub enum LinkEvent {
    Linked {
        peer_id: PeerId,
        info: LinkInfo,
    },
    Unlinked {
        peer_id: PeerId,
    },
    Error {
        peer_id: PeerId,
        error: ProtocolsHandlerUpgrErr<ReadOneError>,
    },
}

pub struct Link {
    info: LinkInfo,
    peers: HashSet<PeerId>,
    heartbeat: Interval,

    pending: VecDeque<()>,
    events: VecDeque<NetworkBehaviourAction<(), LinkEvent>>,
}

impl Link {
    pub fn new(name: String, port: u16) -> Self {
        Self {
            info: LinkInfo { name, port },
            peers: HashSet::new(),
            heartbeat: tokio::time::interval(Duration::from_secs(1)),

            pending: VecDeque::new(),
            events: VecDeque::new(),
        }
    }

    pub fn connect(&mut self, peer_id: PeerId) {
        self.events.push_back(NetworkBehaviourAction::DialPeer {
            peer_id,
            condition: libp2p::swarm::DialPeerCondition::Disconnected,
        });
    }
}

impl NetworkBehaviour for Link {
    type ProtocolsHandler = LinkHandler;

    type OutEvent = LinkEvent;

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        LinkHandler::new()
    }

    fn addresses_of_peer(&mut self, peer_id: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, peer_id: &PeerId) {
        log::info!("Peer {:?} connected.", peer_id);
        self.events
            .push_back(NetworkBehaviourAction::NotifyHandler {
                peer_id: *peer_id,
                event: (),
                handler: NotifyHandler::Any,
            })
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId) {
        log::info!("Peer {:?} disconnected.", peer_id);
        self.peers.remove(peer_id);
        self.events.push_back(NetworkBehaviourAction::GenerateEvent(
            LinkEvent::Unlinked { peer_id: *peer_id },
        ));
    }

    fn inject_event(
        &mut self,
        peer_id: PeerId,
        _connection: ConnectionId,
        event: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent,
    ) {
        // log::info!("inject from behavior {:?}", event);
        match event {
            LinkHandlerEvent::Identified(info) => {
                self.peers.insert(peer_id);
                self.events
                    .push_back(NetworkBehaviourAction::GenerateEvent(LinkEvent::Linked {
                        peer_id,
                        info,
                    }));
            }
            LinkHandlerEvent::Identify(_socket) => {}
            _ => {}
            // IdentifyHandlerEvent::Identify(socket) => self
            //     .replies
            //     .push_back(Reply::Queued { peer, socket }),
            // IdentifyHandlerEvent::IdentificationError(error) => self.events.push_back(
            //     NetworkBehaviourAction::GenerateEvent(LinkEvent::Error { peer, error }),
            // ),
        }
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
        params: &mut impl PollParameters,
    ) -> Poll<
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::InEvent,
            Self::OutEvent,
        >,
    > {
        if let Some(event) = self.events.pop_front() {
            return Poll::Ready(event);
        }

        while let Poll::Ready(_) = self.heartbeat.poll_tick(cx) {
            log::info!(
                "Sending heartbeat to {} identified peers.",
                self.peers.len()
            );

            for peer_id in &self.peers {
                self.events
                    .push_back(NetworkBehaviourAction::NotifyHandler {
                        peer_id: *peer_id,
                        event: (),
                        handler: NotifyHandler::Any,
                    })
            }
        }

        // if let Some(r) = self.replies.pop_front() {
        //     let mut sending = 0;
        //     let to_send = self.replies.len() + 1;
        //     let mut reply = Some(r);

        //     loop {
        //         match reply {
        //             Some(Reply::Queued { peer, socket }) => {
        //                 let socket = Box::pin(socket.send(self.info.clone()));
        //                 reply = Some(Reply::Sending { peer, socket });
        //             }
        //             Some(Reply::Sending { peer, mut socket }) => {
        //                 sending += 1;
        //                 match Future::poll(Pin::new(&mut socket), cx) {
        //                     Poll::Ready(Ok(())) => {
        //                         let event = LinkEvent::Sent { peer };
        //                         return Poll::Ready(NetworkBehaviourAction::GenerateEvent(event))
        //                     },
        //                     Poll::Ready(Err(err)) => {
        //                         let event = LinkEvent::Error {
        //                             peer,
        //                             error: ProtocolsHandlerUpgrErr::Upgrade(UpgradeError::Apply(err.into()))
        //                         };
        //                         return Poll::Ready(NetworkBehaviourAction::GenerateEvent(event));
        //                     }
        //                     Poll::Pending => {
        //                         self.replies.push_back(Reply::Sending { peer, socket });
        //                         if sending == to_send {
        //                             break
        //                         } else {
        //                             reply = self.replies.pop_front();
        //                         }
        //                     }
        //                 }
        //             },
        //             None => unreachable!()
        //         }
        //     }
        // }

        Poll::Pending
    }
}
