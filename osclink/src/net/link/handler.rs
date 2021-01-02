use libp2p::{
    core::{
        upgrade::{InboundUpgrade, OutboundUpgrade, ReadOneError},
        Multiaddr, PeerId,
    },
    swarm::{
        protocols_handler::InboundUpgradeSend, KeepAlive, NegotiatedSubstream, ProtocolsHandler,
        ProtocolsHandlerEvent, ProtocolsHandlerUpgrErr, SubstreamProtocol,
    },
};

use futures::prelude::*;
use std::{pin::Pin, task::Poll};
// use tokio::time::Interval;

use smallvec::SmallVec;

use super::{
    protocol::LinkProtocol,
    LinkInfo,
};

pub struct LinkHandler {
    inbound: Option<NegotiatedSubstream>,
    outbound: Option<NegotiatedSubstream>,

    pending: bool,
    events: SmallVec<[LinkHandlerEvent; 4]>,
    keep_alive: KeepAlive,
}

#[derive(Debug)]
pub enum LinkHandlerEvent {
    Identify(NegotiatedSubstream),
    Identified(LinkInfo),
    // Identified((Multiaddr, PeerId, IdentifyInfo)),
    IdentificationError(ProtocolsHandlerUpgrErr<ReadOneError>),
}

impl LinkHandler {
    pub fn new() -> Self {
        Self {
            inbound: None,
            outbound: None,

            pending: false,
            events: SmallVec::new(),
            keep_alive: KeepAlive::Yes,
        }
    }
}

impl ProtocolsHandler for LinkHandler {
    type InEvent = ();
    type OutEvent = LinkHandlerEvent;

    type InboundProtocol = LinkProtocol;
    type OutboundProtocol = LinkProtocol;

    type InboundOpenInfo = ();
    type OutboundOpenInfo = ();

    type Error = ReadOneError;

    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol, Self::InboundOpenInfo> {
        SubstreamProtocol::new(LinkProtocol, ())
    }

    fn inject_fully_negotiated_inbound(
        &mut self,
        protocol: <Self::InboundProtocol as InboundUpgrade<NegotiatedSubstream>>::Output,
        _info: Self::InboundOpenInfo,
    ) {
        self.inbound = Some(protocol);
        // self.events.push(IdentifyHandlerEvent::Identify(protocol));
    }

    fn inject_fully_negotiated_outbound(
        &mut self,
        protocol: <Self::OutboundProtocol as OutboundUpgrade<NegotiatedSubstream>>::Output,
        _info: Self::OutboundOpenInfo,
    ) {
        self.outbound = Some(protocol);
        // self.events.push(IdentifyHandlerEvent::Identified(protocol));
    }

    fn inject_event(&mut self, _event: Self::InEvent) {
        log::info!("handler inject event");
    }

    fn inject_dial_upgrade_error(
        &mut self,
        _info: Self::OutboundOpenInfo,
        err: ProtocolsHandlerUpgrErr<
            <Self::OutboundProtocol as OutboundUpgrade<NegotiatedSubstream>>::Error,
        >,
    ) {
        log::error!("dial upgrade error");
        // self.events
        //     .push(IdentifyHandlerEvent::IdentificationError(err));
        self.keep_alive = KeepAlive::No;
    }

    fn connection_keep_alive(&self) -> libp2p::swarm::KeepAlive {
        self.keep_alive
    }

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<
        libp2p::swarm::ProtocolsHandlerEvent<
            Self::OutboundProtocol,
            Self::OutboundOpenInfo,
            Self::OutEvent,
            Self::Error,
        >,
    > {
        if !self.events.is_empty() {
            Poll::Ready(ProtocolsHandlerEvent::Custom(self.events.remove(0)))
        } else {
            Poll::Pending
        }

        // match Future::poll(Pin::new(&mut self.delay), cx) {
        //     Poll::Pending => Poll::Pending,
        //     Poll::Ready(Ok(())) => {
        //         self.delay.reset(std::time::Duration::from_secs(1));
        //         let ev = ProtocolsHandlerEvent::OutboundSubstreamRequest {
        //             protocol: SubstreamProtocol::new(IdentifyProtocol, ())
        //         };
        //         Poll::Ready(ev)
        //     },
        //     Poll::Ready(Err(err)) => Poll::Ready(ProtocolsHandlerEvent::Close(err.into()))
        // }
    }
}