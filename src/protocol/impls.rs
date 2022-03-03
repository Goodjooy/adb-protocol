use std::sync::{atomic::AtomicBool, Arc, Mutex};
use tokio::net::TcpStream;
use tower::{util::Ready, Service, ServiceExt};

use super::{adb_respond::AdbError, commands::{Cmd, RespHandler}, config::Config, Protocol};

impl Protocol {
    pub async fn with_config(cfg: Config) -> Self {
        let Config { host, port } = cfg;
        let stream = TcpStream::connect((host, port))
            .await
            .expect(&format!("Cannot Connect To Socket {}:{}", host, port));

        Self {
            connect: Arc::new(Mutex::new(stream)),
            on_handle: AtomicBool::new(false),
        }
    }

    pub async fn queue<T: Cmd + 'static>(&mut self, cmd: T) -> Result<<T as RespHandler>::Resp, AdbError<<T as RespHandler>::Error>> {
        let fut: Ready<_, T> = self.ready();
        let p = fut.await?;
        p.call(cmd).await
    }
}
