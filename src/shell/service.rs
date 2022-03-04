use futures_util::Future;
use tower::Service;

use crate::{commands::shell_cmd::ShellCmd, Protocol};

use super::{Shell, WCmd};

impl<const WITH_RESP: bool> Service<ShellCmd<WITH_RESP>> for Shell {
    type Response = <Protocol as Service<ShellCmd<WITH_RESP>>>::Response;

    type Error = <Protocol as Service<ShellCmd<WITH_RESP>>>::Error;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Service::<ShellCmd<WITH_RESP>>::poll_ready(&mut self.proto, cx)
    }

    fn call(&mut self, req: ShellCmd<WITH_RESP>) -> Self::Future {
        let fut = self.proto.call(WCmd { inner: req });
        async move { fut.await }
    }
}
