//! Nakamoto's client library.
#![deny(missing_docs, unsafe_code)]
pub mod client;
pub mod error;
pub mod handle;

use std::net;
use std::time;

pub use nakamoto_common::network::Network;
pub use nakamoto_p2p::address_book::AddressBook;

#[cfg(test)]
mod tests;

/// Run the light-client. Takes an initial list of peers to connect to, a list of listen addresses
/// and the Bitcoin network to connect to.
pub fn run(
    connect: &[net::SocketAddr],
    listen: &[net::SocketAddr],
    network: Network,
) -> Result<(), error::Error> {
    use client::*;

    let address_book = if connect.is_empty() {
        match AddressBook::load("peers") {
            Ok(peers) if peers.is_empty() => {
                log::info!("Address book is empty. Trying DNS seeds..");
                AddressBook::bootstrap(network.seeds(), network.port())?
            }
            Ok(peers) => peers,
            Err(err) => {
                return Err(error::Error::AddressBook(err));
            }
        }
    } else {
        AddressBook::from(connect)?
    };

    let mut cfg = ClientConfig {
        network,
        listen: if listen.is_empty() {
            vec![([0, 0, 0, 0], 0).into()]
        } else {
            listen.to_vec()
        },
        address_book,
        timeout: time::Duration::from_secs(30),
        ..ClientConfig::default()
    };
    if !connect.is_empty() {
        cfg.target_outbound_peers = connect.len();
    }

    Client::new(cfg)?.run()
}
