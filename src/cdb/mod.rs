//! `cdb/mod.rs`: C DataBase (CDB)
//!
//! This should probably be replaced with the `cdb` crate:
//!
//! https://github.com/bruceg/cdb-rs

mod cdb;
mod hash;
mod make;

pub use self::cdb::Cdb;
pub use self::make::{CdbHp, CdbHpList, CdbMake};
pub use self::hash::hash;
