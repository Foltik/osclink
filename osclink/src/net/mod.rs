use libp2p::{
    swarm::{NetworkBehaviour, NetworkBehaviourEventProcess},
    NetworkBehaviour,
};

use libp2p::mdns::{Mdns, MdnsEvent};

mod discover;
use discover::{Discover, DiscoverEvent};

#[derive(NetworkBehaviour)]
pub struct Behavior {
    mdns: Mdns,
    discover: Discover,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Behavior {
    fn inject_event(&mut self, event: MdnsEvent) {}
}

impl NetworkBehaviourEventProcess<DiscoverEvent> for Behavior {
    fn inject_event(&mut self, event: DiscoverEvent) {}
}
