use crate::{
    commands::ImmCmd,
    protocol::{adb_respond::AdbError, service::Cmd},
    utils::imm_cmd::{
        req_service::ImmCmdReqService,
        resp_service::{ImmCmdReq, ImmCmdRespService},
    },
};
use std::{
    future::Future,
    sync::{atomic::Ordering, Arc},
    task::Poll,
};

use tower::Service;

use crate::Protocol;

impl<Req> Service<ImmCmd<Req>> for Protocol
where
    Req: Cmd,
{
    type Response = Req::Resp;

    type Error = AdbError<Req::Error>;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        if self.on_handle.load(Ordering::Relaxed) {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn call(&mut self, ImmCmd(req): ImmCmd<Req>) -> Self::Future {
        let connect = Arc::clone(&self.connect);
        let mut resp_service = ImmCmdRespService;
        let mut req_service = ImmCmdReqService;
        async move {
            let mut conn = connect.lock().await;
            let (reader, writer) = conn.split();
            let _ww = Service::call(&mut req_service, (req, writer))
                .await
                .map_err(AdbError::Io)?;

            let (resp, _rr) =
                Service::call(&mut resp_service, ImmCmdReq::<Req, _>::new(reader)).await?;

            Ok(resp)
        }
    }
}
