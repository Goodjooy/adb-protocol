use std::sync::{atomic::AtomicBool, Arc};
use tokio::{net::TcpStream, sync::Mutex};
use tower::{util::Ready, Service, ServiceExt};

use crate::commands::{CmdExt, ImmCmd, RespHandler};

use super::{adb_respond::AdbError, config::Config, Protocol};

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

    pub async fn imm_queue<T>(
        &mut self,
        cmd: T,
    ) -> Result<<T as RespHandler>::Resp, AdbError<<T as RespHandler>::Error>>
    where
        T: RespHandler + 'static,
    {
        let fut: Ready<_, ImmCmd<T>> = self.ready();
        let p = fut.await?;
        p.call(cmd.as_imm()).await
    }
}
