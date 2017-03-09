#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![deny(clippy)]
#![deny(clippy_pedantic)]
#![allow(unseparated_literal_suffix)] // necessary due to serde.
#![allow(missing_docs_in_private_items)] // FIXME: should be removed.

extern crate reqwest;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate error_chain;

pub mod client;
pub mod entity;
pub mod entities;
pub mod errors;
