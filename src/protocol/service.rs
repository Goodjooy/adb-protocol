use std::{
    future::Future,
    sync::{atomic::Ordering, Arc},
    task::Poll,
};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::Service;

use super::{
    adb_respond::{self, AdbError},
    commands::Cmd,
    Protocol,
};

impl<Req> Service<Req> for Protocol
where
    Req: Cmd,
{
    type Response = Req::Resp;

    type Error = adb_respond::AdbError<Req::Error>;

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

    fn call(&mut self, req: Req) -> Self::Future {
        let req_cmd = req.cmd();
        let cmd = format!("{:04X}{}", req_cmd.len(), req_cmd);
        let connect = Arc::clone(&self.connect);
        async move {
            let mut conn = connect.lock().unwrap();
            let (mut reader, mut writer) = conn.split();
            let _ = writer
                .write_all(cmd.as_bytes())
                .await
                .map_err(AdbError::Io)?;

            let mut resp = [0u8; 4];
            let _ = reader.read_exact(&mut resp).await.map_err(AdbError::Io)?;
            match &resp {
                b"OKAY" => {
                    let resp = Req::handle(reader).await.map_err(AdbError::Cmd)?;
                    let (resp, _rr) = resp;
                    Ok(resp)
                }
                b"FAIL" => {
                    let mut len_str = [0u8; 4];
                    reader
                        .read_exact(&mut len_str)
                        .await
                        .map_err(AdbError::Io)?;
                    let len_str = String::from_utf8_lossy(&len_str);

                    let le = u16::from_str_radix(&len_str, 16).map_err(AdbError::Parse)?;

                    let mut info = String::with_capacity(le as usize);

                    reader
                        .read_exact(unsafe { info.as_bytes_mut() })
                        .await
                        .map_err(AdbError::Io)?;
                    Err(AdbError::Failure(info))
                }
                _ => Err(AdbError::Unknown),
            }
        }
    }
}
