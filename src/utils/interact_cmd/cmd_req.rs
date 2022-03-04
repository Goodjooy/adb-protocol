use std::{io, task::Poll};

use futures_util::Future;
use tokio::{io::AsyncWriteExt, net::tcp::WriteHalf};
use tower::Service;

use crate::commands::Cmd;
use std::io::Write;
pub struct InteractCmdReqService;

impl<'w, C> Service<(C, WriteHalf<'w>)> for InteractCmdReqService
where
    C: Cmd,
{
    type Response = WriteHalf<'w>;

    type Error = io::Error;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, (cmd, mut writer): (C, WriteHalf<'w>)) -> Self::Future {
        async move {
            let mut buff = Vec::new();
            writeln!(buff, "{}", cmd.cmd())?;
            writer.write_all(&buff).await?;

            Ok(writer)
        }
    }
}
