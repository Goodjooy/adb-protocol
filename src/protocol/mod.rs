pub mod adb_respond;
pub mod config;
mod impls;
mod service;
use std::sync::{atomic::AtomicBool, Arc};

use tokio::sync::Mutex;

pub struct Protocol {
    connect: Arc<Mutex <tokio::net::TcpStream>>,
    on_handle: AtomicBool,
}
