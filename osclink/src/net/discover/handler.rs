use libp2p::{
    core::{
        PeerId,
        Multiaddr,
    },
    swarm::{
        ProtocolsHandler,
        ProtocolsHandlerEvent,
        ProtocolsHandlerUpgrErr,
        SubstreamProtocol,
    },
};


struct DiscoverHandlerEvent {

}

pub struct DiscoverHandler {

}

impl ProtocolsHandler for DiscoverHandler {
    type InEvent = ();
    type OutEvent = DiscoverHandlerEvent;

    type Error;

    type InboundProtocol;

    type OutboundProtocol;

    type InboundOpenInfo;

    type OutboundOpenInfo;

    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol, Self::InboundOpenInfo> {
        todo!()
    }

    fn inject_fully_negotiated_inbound(
        &mut self,
        protocol: InboundUpgradeSend::Output,
        info: Self::InboundOpenInfo
    ) {
        todo!()
    }

    fn inject_fully_negotiated_outbound(
        &mut self,
        protocol: libp2p::swarm::protocols_handler::OutboundUpgradeSend::Output,
        info: Self::OutboundOpenInfo
    ) {
        todo!()
    }

    fn inject_event(&mut self, event: Self::InEvent) {
        todo!()
    }

    fn inject_dial_upgrade_error(
        &mut self,
        info: Self::OutboundOpenInfo,
        error: libp2p::swarm::ProtocolsHandlerUpgrErr<
            libp2p::swarm::protocols_handler::OutboundUpgradeSend::Error
        >
    ) {
        todo!()
    }

    fn connection_keep_alive(&self) -> libp2p::swarm::KeepAlive {
        todo!()
    }

    fn poll(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<
        libp2p::swarm::ProtocolsHandlerEvent<Self::OutboundProtocol, Self::OutboundOpenInfo, Self::OutEvent, Self::Error>
    > {
        todo!()
    }
}

enum DiscoverEvent {

}