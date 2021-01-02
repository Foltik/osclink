use std::task::{Context, Poll};

use libp2p::{identity::Keypair, Swarm};
use libp2p::core::PeerId;

use futures::prelude::*;

use osclink::OscLink;

#[tokio::main]
async fn main() {
    env_logger::builder().filter(None, log::LevelFilter::Debug).init();

    let args = std::env::args().collect::<Vec<_>>();
    let name = args.get(1).map(|s| &s[..]).unwrap_or("example");

    // let client = OscLink::new(name);

    let id_keys = Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // Create a transport.
    let transport = libp2p::build_development_transport(id_keys).unwrap();

    let behavior = osclink::net::Behavior::new(name.to_owned(), 1234).await;

    let mut swarm = Swarm::new(transport, behavior, peer_id);
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    let mut listening = false;
    future::poll_fn(move |cx: &mut Context<'_>| {
        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => println!("{:?}", event),
                Poll::Ready(None) => return Poll::Ready(()),
                Poll::Pending => {
                    if !listening {
                        if let Some(a) = Swarm::listeners(&swarm).next() {
                            println!("Listening on {:?}", a);
                            listening = true;
                        }
                    }
                    break
                }
            }
        }
        Poll::Pending
    }).await;

    // let mut listening = false;
    // loop {
    //     let event = swarm.next().await;

    //     log::debug!("Event: {:?}", event);

    //     if !listening {
    //         for addr in Swarm::listeners(&swarm) {
    //             log::debug!("Listening on {:?}", addr);
    //             listening = true;
    //         }
    //     }
    // }
}