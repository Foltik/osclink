use libp2p::{
    core::{
        PeerId,
        Multiaddr,
    },
    swarm::{
        NetworkBehaviour, 
        NetworkBehaviourEventProcess,
        ProtocolsHandler,
    },
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
};

mod handler;
use handler::DiscoverHandler;

pub enum DiscoverEvent {

}

pub struct Discover {

}

impl NetworkBehaviour for Discover {
    type ProtocolsHandler = DiscoverHandler;

    type OutEvent = ();

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        DiscoverHandler::new()
    }

    fn addresses_of_peer(&mut self, peer_id: &libp2p::PeerId) -> Vec<libp2p::Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, peer_id: &PeerId) {}

    fn inject_disconnected(&mut self, peer_id: &libp2p::PeerId) {}

    fn inject_event(
        &mut self,
        peer_id: libp2p::PeerId,
        connection: libp2p::core::connection::ConnectionId,
        event: Self::OutEvent,
    ) {
        todo!()
    }

    fn poll(&mut self, cx: &mut std::task::Context<'_>, params: &mut impl libp2p::swarm::PollParameters)
        -> std::task::Poll<libp2p::swarm::NetworkBehaviourAction<<Self::ProtocolsHandler as ProtocolsHandler>::InEvent, Self::OutEvent>> {
        todo!()
    }
}
