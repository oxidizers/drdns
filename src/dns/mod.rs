//! `dns/mod.rs`: The Domain Name System module

mod domain;
mod ip4;
mod mx;
mod name;
mod packet;
mod random;
mod rcip;
mod rcrw;
mod resolve;
mod sortip;
mod transmit;
mod txt;

pub use self::transmit::DnsTransmit;
