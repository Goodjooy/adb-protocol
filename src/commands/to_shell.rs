use std::io;

use futures_util::Future;
use tokio::{net::tcp::ReadHalf, io::AsyncReadExt};

use crate::commands::RespHandler;

use super::Cmd;


pub struct ToShell;

impl RespHandler for ToShell {
    type Fut<'r>=impl Future<Output = Result<(Self::Resp,ReadHalf<'r>),Self::Error>>;

    type Resp=Option<String>;

    type Error=io::Error;

    fn handle<'r>(reader: tokio::net::tcp::ReadHalf<'r>) -> Self::Fut<'r> {
        let mut reader = reader;
        async move {
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
        }
    }
}

impl Cmd for ToShell {
    fn cmd(self) -> std::borrow::Cow<'static, str> {
        "shell:".into()
    }
}