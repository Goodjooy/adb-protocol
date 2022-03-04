pub mod adb_respond;
pub mod config;
mod impls;
mod service;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

pub struct Protocol {
    connect: Arc<Mutex<tokio::net::TcpStream>>,
    on_handle: AtomicBool,
}
