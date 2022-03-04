pub mod builder;
mod impls;
mod service;
use core::future::Future;
use std::io;

use tokio::net::tcp::ReadHalf;
use tower::{Service, ServiceExt};

use crate::{
    commands::{shell_cmd::ShellCmd, Cmd, RespHandler},
    AdbError, Protocol,
};

pub struct Shell {
    pub(super) proto: Protocol,
}

impl Shell {
    // pub async fn queue<const T: bool>(
    //     &mut self,
    //     cmd: ShellCmd<T>,
    // ) -> Result<<ShellCmd<T> as RespHandler>::Resp, AdbError<<ShellCmd<T> as RespHandler>::Error>>
    // {
    //     let fut: tower::util::Ready<Shell, ShellCmd<T>> = self.ready();
    //     let p = fut.await?;
    //     p.call(cmd).await
    // }
}

pub struct WCmd<const R: bool> {
    inner: ShellCmd<R>,
}

impl<const R: bool> Cmd for WCmd<R> {
    fn cmd(self) -> std::borrow::Cow<'static, str> {
        format!(
            "{} {}",
            self.inner.cmd,
            self.inner
                .args
                .into_iter()
                .map(|s| s.to_string())
                .reduce(|l, r| format!("\"{}\" \"{}\"", l, r))
                .unwrap_or_default()
        )
        .into()
    }
}

impl<const R: bool> RespHandler for WCmd<R> {
    type Fut<'r> = impl Future<Output = Result<(Self::Resp, ReadHalf<'r>), Self::Error>>;

    type Resp = Option<String>;

    type Error = io::Error;

    fn handle<'r>(reader: tokio::net::tcp::ReadHalf<'r>) -> Self::Fut<'r> {
        <ShellCmd<R> as RespHandler>::handle(reader)
    }
}
