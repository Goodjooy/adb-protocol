pub mod ad_hoc;
pub mod derives;
pub mod shell_cmd;
pub mod switch_shell;
pub mod transport;
use tokio::net::tcp::ReadHalf;

use std::io::Read;
use std::{borrow::Cow, future::Future};

pub trait Cmd {
    fn cmd(self) -> Cow<'static, str>;
}

pub trait RespHandler: Cmd {
    type Fut<'r>: Future<Output = Result<(Self::Resp, ReadHalf<'r>), Self::Error>>;
    type Resp;
    type Error;
    fn handle<'r>(reader: ReadHalf<'r>) -> Self::Fut<'r>;
}

pub trait SyncHandler: Cmd {
    type Resp;
    type Err;
    fn sync_handle<R: Read>(reader: R) -> Result<Self::Resp, Self::Err>;
}

/// Immediate Command 立即完成的指令类型
/// 将会在发送后立即解析Resp
pub struct ImmCmd<C>(pub(crate) C);

/// Interactive Command 交互式指令类型
/// 将会在Shell模式下才会使用
pub struct InteractCmd<C>(pub(crate) C);

/// Live Update Command 持续接收指令类型
/// 将会持续等待stream 的结果
pub struct LiveUpdateCmd<C>(pub(crate) C);


pub trait CmdExt: Cmd + Sized {
    fn as_imm(self) -> ImmCmd<Self> {
        ImmCmd(self)
    }
    fn as_interact(self) -> InteractCmd<Self> {
        InteractCmd(self)
    }
    fn as_live(self) -> LiveUpdateCmd<Self> {
        LiveUpdateCmd(self)
    }
}

impl<C: Cmd + Sized> CmdExt for C {}

pub use derives::Derives;
pub use shell_cmd::ShellCmd;
pub use shell_cmd::ShellCmdBuilder;
pub use transport::TransPort;

pub use switch_shell::SwitchShell;
