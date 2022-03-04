use tokio::io::{AsyncRead, AsyncReadExt};

use crate::AdbError;

pub async fn read_size<R, E>(reader: &mut R) -> Result<usize, AdbError<E>>
where
    R: AsyncRead + Unpin,
{
    let mut len_str = [0u8; 4];
    reader
        .read_exact(&mut len_str)
        .await
        .map_err(AdbError::Io)?;
    let len_str = String::from_utf8_lossy(&len_str);

    let le = u16::from_str_radix(&len_str, 16).map_err(AdbError::Parse)?;

    Ok(le as usize)
}

pub async fn read_resp_body<R, E>(reader: &mut R) -> Result<String, AdbError<E>>
where
    R: AsyncRead + Unpin,
{
    let size = read_size(reader).await?;
    let mut string = vec![0u8; size];

    reader.read_exact(&mut string).await.map_err(AdbError::Io)?;

    Ok(String::from_utf8(string).expect("Error Reading String"))
}
