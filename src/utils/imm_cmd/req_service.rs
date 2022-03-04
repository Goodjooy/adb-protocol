use std::{
    future::Future,
    io::{self, Write},
    task::Poll,
};

use tokio::{io::AsyncWriteExt, net::tcp::WriteHalf};
use tower::Service;

use crate::commands::Cmd;

pub struct ImmCmdReqService;

impl<'w, C> Service<(C, WriteHalf<'w>)> for ImmCmdReqService
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
            let req_cmd = cmd.cmd();
            write!(buff, "{:04X}{}", req_cmd.len(), req_cmd)?;
            let _ = writer.write_all(buff.as_slice()).await?;

            Ok(writer)
        }
    }
}
