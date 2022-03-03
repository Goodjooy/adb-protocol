use futures_util::future::{ok, Ready};
use tokio::net::tcp::ReadHalf;

use crate::protocol::commands::Cmd;

use super::RespHandler;

pub enum TransPort {
    Serial(String),
    Usb,
    Local,
    Any,
}

impl Cmd for TransPort {
    fn cmd(self) -> std::borrow::Cow<'static, str> {
        match self {
            TransPort::Serial(s) => format!("host:transport:{}", s).into(),
            TransPort::Usb => "host:transport-usb".into(),
            TransPort::Local => "host:transport-local".into(),
            TransPort::Any => "host:transport-any".into(),
        }
    }
}

impl RespHandler for TransPort {
    type Fut<'r> = Ready<Result<((), ReadHalf<'r>), std::convert::Infallible>>;

    type Resp = ();

    type Error = std::convert::Infallible;

    fn handle<'r>(reader: ReadHalf<'r>) -> Self::Fut<'r> {
        ok(((), reader))
    }
}
