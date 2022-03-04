use std::{convert::Infallible, time::Duration, io};

use tokio::time;

use crate::{commands::{TransPort, to_shell::ToShell}, AdbError, Config, Protocol};

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
        let _ = self.inner.queue(transport).await.unwrap();
        let _ = self.inner.queue(ToShell).await.unwrap();
        Ok(Shell { proto: self.inner })
    }
}
