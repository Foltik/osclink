use libp2p::{core::UpgradeInfo, swarm::NegotiatedSubstream, InboundUpgrade, OutboundUpgrade};

use futures::prelude::*;

pub struct LinkProtocol;

impl UpgradeInfo for LinkProtocol {
    type Info = &'static [u8];
    type InfoIter = std::iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        std::iter::once(b"/osclink/link/1.0.0")
    }
}

impl InboundUpgrade<NegotiatedSubstream> for LinkProtocol {
    type Output = NegotiatedSubstream;
    type Error = ();
    type Future = future::Ready<Result<Self::Output, Self::Error>>;

    fn upgrade_inbound(self, socket: NegotiatedSubstream, _: Self::Info) -> Self::Future {
        future::ok(socket)
    }
}

impl OutboundUpgrade<NegotiatedSubstream> for LinkProtocol {
    type Output = NegotiatedSubstream;
    type Error = ();
    type Future = future::Ready<Result<Self::Output, Self::Error>>;

    fn upgrade_outbound(self, socket: NegotiatedSubstream, _: Self::Info) -> Self::Future {
        future::ok(socket)
    }
}

// pub enum Reply {
//     Queued {
//         peer: PeerId,
//         socket: NegotiatedSubstream,
//     },
//     Sending {
//         peer: PeerId,
//         socket: Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>,
//     },
// }

// pub struct ReplySubstream<T> {
//     inner: T,
// }

// impl<T> std::fmt::Debug for ReplySubstream<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("ReplySubstream").finish()
//     }
// }

// impl<T> ReplySubstream<T>
// where
//     T: AsyncWrite + Unpin,
// {
//     /// Sends back the requested information on the substream.
//     ///
//     /// Consumes the substream, returning a `ReplyFuture` that resolves
//     /// when the reply has been sent on the underlying connection.
//     pub fn send(mut self, info: IdentifyInfo) -> impl Future<Output = Result<(), Error>> {
//         log::debug!("Sending identify info to client");
//         log::debug!("Sending: {:?}", info);

//         let message = structs::Identify {
//             name: info.name,
//             port: info.port as u32,
//         };

//         async move {
//             let mut bytes = Vec::with_capacity(message.encoded_len());
//             message
//                 .encode(&mut bytes)
//                 .expect("Vec<u8> provides capacity as needed");
//             libp2p::core::upgrade::write_one(&mut self.inner, &bytes).await
//         }
//     }
// }

// Box::pin(async move {
//     socket.close().await?;
//     let msg = libp2p::core::upgrade::read_one(&mut socket, 4096).await?;

//     let info = IdentifyInfo::decode(msg.as_ref())?;
//     log::debug!("Information received: {:?}", info);

//     Ok(info)
// })
