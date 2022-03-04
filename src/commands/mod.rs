pub mod ad_hoc;
pub mod shell_cmd;
pub mod derives;
pub mod transport;
pub mod  to_shell;
use tokio::net::tcp::ReadHalf;

use std::{borrow::Cow, future::Future};

pub trait Cmd: RespHandler {
    fn cmd(self) -> Cow<'static, str>;
    fn done_status() -> ConnectStatus {
        ConnectStatus::KeepAlive
    }
}

pub trait RespHandler {
    type Fut<'r>: Future<Output = Result<(Self::Resp, ReadHalf<'r>), Self::Error>>;
    type Resp;
    type Error;
    fn handle<'r>(reader: ReadHalf<'r>) -> Self::Fut<'r>;
}

pub enum ConnectStatus {
    KeepAlive,
    Close,
}
pub use derives::Derives;
pub use transport::TransPort;
pub use shell_cmd::ShellCmd;
pub use shell_cmd::ShellCmdBuilder;

pub use to_shell::ToShell;