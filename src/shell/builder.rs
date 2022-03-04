use std::{convert::Infallible, io, time::Duration};

use tokio::time;

use crate::{commands::{TransPort, switch_shell::SwitchShell}, AdbError, Config, Protocol};

use super::Shell;

pub struct ShellBuilder {
    inner: Protocol,
}

impl ShellBuilder {
    pub async fn with_config(cfg: Config) -> Self {
        Self {
            inner: Protocol::with_config(cfg).await,
        }
    }

    pub async fn connect_to_device(
        mut self,
        transport: TransPort,
    ) -> Result<Shell, AdbError<io::Error>> {
        let _ = self.inner.imm_queue(transport).await.unwrap();
        // let _ = self.inner.imm_queue(SwitchShell::Shell).await.unwrap();
        Ok(Shell { proto: self.inner })
    }
}
