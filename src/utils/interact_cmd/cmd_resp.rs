use std::future::Future;
use std::io::Cursor;
use std::marker::PhantomData;
use std::task::Poll;

use tokio::io::AsyncReadExt;
use tokio::net::tcp::ReadHalf;
use tower::Service;

use crate::{commands::SyncHandler, AdbError};

pub struct InteractCmdRespService;

pub struct ReaderWrap<'r, C>(ReadHalf<'r>, PhantomData<C>);

impl<'r, C> ReaderWrap<'r, C> {
    pub fn new(reader: ReadHalf<'r>) -> Self {
        Self(reader, Default::default())
    }
}

impl<'r, C> Service<ReaderWrap<'r, C>> for InteractCmdRespService
where
    C: SyncHandler,
{
    type Response = (C::Resp, ReadHalf<'r>);

    type Error = AdbError<C::Err>;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, ReaderWrap(mut reader, _): ReaderWrap<'r, C>) -> Self::Future {
        async move {
            let mut buff = Vec::<u8>::new();
            let mut key_buf = [0u8; 4];

            loop {
                reader
                    .read_exact(&mut key_buf)
                    .await
                    .map_err(AdbError::Io)?;
                if &key_buf == b"/ $ " {
                    break;
                } else if key_buf.ends_with(b"/ $") {
                    let mut tb = [0u8; 1];
                    reader.read_exact(&mut tb).await.map_err(AdbError::Io)?;
                    if &tb == b" " {
                        buff.push(key_buf[0]);
                        break;
                    } else {
                        buff.extend(&key_buf);
                        buff.extend(&tb);
                    }
                } else if key_buf.ends_with(b"/ ") {
                    let mut tb = [0u8; 2];
                    reader.read_exact(&mut tb).await.map_err(AdbError::Io)?;
                    if &tb == b"$ " {
                        buff.extend(&key_buf[0..2]);
                        break;
                    } else {
                        buff.extend(&key_buf);
                        buff.extend(&tb);
                    }
                } else if key_buf.ends_with(b"/") {
                    let mut tb = [0u8; 3];
                    reader.read_exact(&mut tb).await.map_err(AdbError::Io)?;
                    if &tb == b" $ " {
                        buff.extend(&key_buf[0..3]);
                        break;
                    } else {
                        buff.extend(&key_buf);
                        buff.extend(&tb);
                    }
                } else {
                    buff.extend(&key_buf);
                }
            }

            let sync_reader = Cursor::new(buff);
            let res = C::sync_handle(sync_reader).map_err(AdbError::Cmd)?;

            Ok((res, reader))
        }
    }
}
