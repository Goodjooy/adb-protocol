use std::io;

use super::{Cmd, SyncHandler};

pub enum SwitchShell {
    Shell,
    Exit,
}

impl Cmd for SwitchShell {
    fn cmd(self) -> std::borrow::Cow<'static, str> {
        match self {
            SwitchShell::Shell => "shell:".into(),
            SwitchShell::Exit => "exit".into(),
        }
    }
}

impl SyncHandler for SwitchShell {
    type Resp = String;

    type Err = io::Error;

    fn sync_handle<R: io::Read>(mut reader: R) -> Result<Self::Resp, Self::Err> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
