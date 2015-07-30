#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate mio;
extern crate serde;
extern crate uuid;

mod ipc_bridge;
mod plugin;
pub mod client;
pub mod ipc;
pub mod plugin_core;
pub mod server;
