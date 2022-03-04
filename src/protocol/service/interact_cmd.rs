use std::{sync::Arc, task::Poll};

use futures_util::Future;
use tower::Service;

use crate::{
    commands::{InteractCmd, SyncHandler},
    utils::interact_cmd::{
        cmd_req::InteractCmdReqService,
        cmd_resp::{InteractCmdRespService, ReaderWrap},
    },
    AdbError, Protocol,
};

impl<C> Service<InteractCmd<C>> for Protocol
where
    C: SyncHandler,
{
    type Response = C::Resp;

    type Error = AdbError<C::Err>;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, InteractCmd(req): InteractCmd<C>) -> Self::Future {
        let mut req_service = InteractCmdReqService;
        let mut resp_service = InteractCmdRespService;
        let connect = Arc::clone(&self.connect);

        async move {
            let mut conn = connect.lock().await;
            let (reader, writer) = conn.split();

            let _ww = Service::call(&mut req_service, (req, writer))
                .await
                .map_err(AdbError::Io)?;

            let (resp, _rr) =
                Service::call(&mut resp_service, ReaderWrap::<C>::new(reader)).await?;

            Ok(resp)
        }
    }
}
