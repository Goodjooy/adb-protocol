use std::{borrow::Cow, io};

use futures_util::Future;
use tokio::{io::AsyncReadExt, net::tcp::ReadHalf};

use super::{Cmd, RespHandler};

pub struct ShellCmd<const WITH_RESP: bool> {
    pub(crate) cmd: Cow<'static, str>,
    pub(crate) args: Vec<Cow<'static, str>>,
}

impl<const WITH_RESP: bool> Cmd for ShellCmd<WITH_RESP> {
    fn cmd(self) -> Cow<'static, str> {
        // shell:command arg1 arg2 ...
        format!(
            "shell:{} {}",
            self.cmd,
            self.args
                .into_iter()
                .map(|s| s.to_string())
                .reduce(|l, r| format!("\"{}\" \"{}\"", l, r))
                .unwrap_or_default()
        )
        .into()
    }
}

impl<const WITH_RESP: bool> RespHandler for ShellCmd<WITH_RESP> {
    type Fut<'r> = impl Future<Output = Result<(Self::Resp, ReadHalf<'r>), Self::Error>>;

    type Resp = Option<String>;

    type Error = io::Error;

    fn handle<'r>(reader: ReadHalf<'r>) -> Self::Fut<'r> {
        let mut reader = reader;
        async move {
            if WITH_RESP {
                let mut buf = [0u8; 4];
                let _ = reader.read_exact(&mut buf).await?;

                let len_str = String::from_utf8_lossy(&mut buf);
                match u16::from_str_radix(&len_str, 16) {
                    Ok(size) => {
                        let mut res = vec![0; size as usize];

                        let _res = reader.read_exact(&mut res).await?;

                        let full = unsafe { String::from_utf8_unchecked(res) };
                        Ok((Some(full), reader))
                    }
                    Err(_) => Ok((None, reader)),
                }
            } else {
                Ok((None, reader))
            }
        }
    }
}

pub struct ShellCmdBuilder<const WITH_RESP: bool> {
    inner: ShellCmd<WITH_RESP>,
}

impl ShellCmdBuilder<false> {
    pub fn with_no_resp(cmd: impl Into<Cow<'static, str>>)->ShellCmdBuilder<false>{
        ShellCmdBuilder::<false>::new(cmd)
    }
}
impl ShellCmdBuilder<true> {
    pub fn with_resp(cmd: impl Into<Cow<'static, str>>)->ShellCmdBuilder<true>{
        ShellCmdBuilder::<true>::new(cmd)
    }
}

impl<const WITH_RESP: bool> ShellCmdBuilder<WITH_RESP> {
    fn new(cmd: impl Into<Cow<'static, str>>) -> Self {
        Self {
            inner: ShellCmd {
                cmd: cmd.into(),
                args: vec![],
            },
        }
    }

    pub fn arg(mut self, arg: impl Into<Cow<'static, str>>) -> Self {
        self.inner.args.push(arg.into());
        self
    }


    pub fn build(self) -> ShellCmd<WITH_RESP> {
        self.inner
    }
}
