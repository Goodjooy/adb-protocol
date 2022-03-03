use std::io;
use tokio::net::tcp::ReadHalf;

use futures_util::Future;
use tokio::io::AsyncReadExt;

use super::{Cmd, RespHandler};

pub struct Derives;

impl Cmd for Derives {
    fn cmd(self) -> std::borrow::Cow<'static, str> {
        "host:devices".into()
    }
}

impl RespHandler for Derives {
    type Fut<'s> = impl Future<Output = Result<(Self::Resp, ReadHalf<'s>), Self::Error>>;

    type Resp = Vec<(String, String)>;

    type Error = io::Error;

    fn handle<'s>(reader: ReadHalf<'s>) -> Self::Fut<'s> {
        let mut reader = reader;
        async move {
            let mut buf = [0u8; 4];

            let _res = reader.read_exact(&mut buf).await;

            let len_str = String::from_utf8_lossy(&mut buf);
            let size = u16::from_str_radix(&len_str, 16).unwrap();

            let mut res = vec![0; size as usize];

            let _res = reader.read_exact(&mut res).await?;

            let full = unsafe { String::from_utf8_unchecked(res) };

            let iter = full.split('\n').filter_map(|s| {
                let mut inner = s.split('\t');
                let id = inner.next()?.to_owned();
                let ty = inner.next()?.to_owned();
                Some((id, ty))
            });
            Ok((iter.collect(), reader))
        }
    }
}
