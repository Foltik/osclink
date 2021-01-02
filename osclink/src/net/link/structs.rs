use prost::Message;
use std::io::{Error, ErrorKind};

use super::LinkInfo;

include!(concat!(env!("OUT_DIR"), "/link.proto.rs"));

impl LinkInfo {
    pub fn encode(info: Self) -> Vec<u8> {
        let msg = Link {
            name: info.name,
            port: info.port as u32,
        };

        let mut data = Vec::with_capacity(msg.encoded_len());
        msg.encode(&mut data).unwrap();
        data
    }

    pub fn decode(msg: &[u8]) -> Result<Self, Error> {
        match Link::decode(msg) {
            Ok(msg) => Ok(LinkInfo {
                port: msg.port as u16,
                name: msg.name,
            }),
            Err(err) => Err(Error::new(ErrorKind::InvalidData, err).into()),
        }
    }
}