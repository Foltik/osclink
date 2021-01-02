use libp2p::{
    NetworkBehaviour,
    swarm::NetworkBehaviourEventProcess,
};

use libp2p::mdns::{Mdns, MdnsEvent};

mod link;
use link::{Link, LinkEvent};

#[derive(NetworkBehaviour)]
pub struct Behavior {
    mdns: Mdns,
    link: Link,
}

impl Behavior {
    pub async fn new(name: String, port: u16) -> Self {
        Self {
            mdns: Mdns::new().await.unwrap(),
            link: Link::new(name, port)
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Behavior {
    fn inject_event(&mut self, event: MdnsEvent) {
        log::debug!("{:?}", event);
        if let MdnsEvent::Discovered(addrs) = event {
            for addr in addrs {
                log::debug!("Discovered: {:?}", addr);
                self.link.connect(addr.0);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<LinkEvent> for Behavior {
    fn inject_event(&mut self, event: LinkEvent) {
        log::debug!("{:?}", event);
    }
}
