use std::{marker::PhantomData, task::Poll};

use futures_util::Future;
use tokio::{io::AsyncReadExt, net::tcp::ReadHalf};
use tower::Service;

use crate::{commands::Cmd, utils::adb_ext::read_resp_body, AdbError};

pub struct ImmCmdRespService;

pub struct ImmCmdReq<C, R>(pub(crate) R, PhantomData<C>);

impl<C, R> ImmCmdReq<C, R> {
    pub(crate) fn new(reader: R) -> Self {
        Self(reader, Default::default())
    }
}

impl<'r, C> Service<ImmCmdReq<C, ReadHalf<'r>>> for ImmCmdRespService
where
    C: Cmd,
{
    type Response = (C::Resp, ReadHalf<'r>);

    type Error = AdbError<C::Error>;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, ImmCmdReq(mut reader, _): ImmCmdReq<C, ReadHalf<'r>>) -> Self::Future {
        async move {
            let mut resp = [0u8; 4];
            let _ = reader.read_exact(&mut resp).await.map_err(AdbError::Io)?;
            match &resp {
                b"OKAY" => {
                    let resp = C::handle(reader).await.map_err(AdbError::Cmd)?;
                    let (resp, rr) = resp;
                    Ok((resp, rr))
                }
                b"FAIL" => {
                    let info = read_resp_body(&mut reader).await?;
                    Err(AdbError::Failure(info))
                }
                _ => Err(AdbError::Unknown(
                    String::from_utf8(resp.into()).unwrap().into(),
                )),
            }
        }
    }
}
