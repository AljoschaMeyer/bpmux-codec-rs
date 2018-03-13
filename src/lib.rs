//! TODO docs
#![deny(missing_docs)]

/// The type of a packet is indicated via a tag as its first byte.
pub type PacketTag = u8;

#[allow(missing_docs)]
pub const MAIN_CREDIT: PacketTag = 0;
#[allow(missing_docs)]
pub const CLOSE: PacketTag = 1;
#[allow(missing_docs)]
pub const MINIMUM: PacketTag = 2;
#[allow(missing_docs)]
pub const CANCEL_ALL: PacketTag = 3;
#[allow(missing_docs)]
pub const MESSAGE: PacketTag = 4;
#[allow(missing_docs)]
pub const MESSAGES: PacketTag = 5;
#[allow(missing_docs)]
pub const REQUEST: PacketTag = 6;
#[allow(missing_docs)]
pub const REQUESTS: PacketTag = 7;
#[allow(missing_docs)]
pub const REQUEST_CANCEL: PacketTag = 8;
#[allow(missing_docs)]
pub const RESPONSE: PacketTag = 9;
#[allow(missing_docs)]
pub const RESPONSE_ERROR: PacketTag = 10;
#[allow(missing_docs)]
pub const CREATE_STREAM: PacketTag = 11;
#[allow(missing_docs)]
pub const CREATE_SINK: PacketTag = 12;
#[allow(missing_docs)]
pub const GREEDY_SINK: PacketTag = 13;
#[allow(missing_docs)]
pub const CREATE_DUPLEX: PacketTag = 14;
#[allow(missing_docs)]
pub const GREEDY_DUPLEX: PacketTag = 15;
#[allow(missing_docs)]
pub const INNER_CREDIT: PacketTag = 16;
#[allow(missing_docs)]
pub const INNER_CANCEL: PacketTag = 17;
#[allow(missing_docs)]
pub const INNER_ITEM: PacketTag = 18;
#[allow(missing_docs)]
pub const INNER_ITEMS: PacketTag = 19;
#[allow(missing_docs)]
pub const INNER_END: PacketTag = 20;
#[allow(missing_docs)]
pub const INNER_ERROR: PacketTag = 21;

#[macro_use]
extern crate futures_core;
extern crate futures_sink;
extern crate futures_io;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
