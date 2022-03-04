//! ref
//! [Protocol](https://github.com/aosp-mirror/platform_system_core/blob/34a0e57a257f0081c672c9be0e87230762e677ca/adb/OVERVIEW.TXT)
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

pub mod commands;
mod protocol;
mod shell;
mod utils;
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        commands::{shell_cmd::ShellCmdBuilder, Derives},
        protocol::{config::Config, Protocol},
    };

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[tokio::test]
    async fn test_adb() {
        let config = Config {
            host: [127, 0, 0, 1].into(),
            port: 5037,
        };
        let mut protocol = Protocol::with_config(config).await;

        let fut = protocol.imm_queue(Derives);
        let res = fut.await;

        println!("Res {:#?}", &res);
    }

    #[tokio::test]
    async fn test_adb_transport() {
        let mut trigger = tokio::time::interval(Duration::from_millis(500));
        let config = Config {
            host: [127, 0, 0, 1].into(),
            port: 5037,
        };
        let mut protocol = Protocol::with_config(config).await;

        // trigger.tick().await;
        // let fut = protocol.queue(TransPort::Usb);
        // let _res = fut.await.unwrap();

        trigger.tick().await;
        let cmd = ShellCmdBuilder::with_no_resp("").build();

        let res = protocol.imm_queue(cmd).await.unwrap();

        println!("Res {:#?}", &res);

        // trigger.tick().await;
        // let cmd=ShellCmdBuilder::with_no_resp("input").arg("keyevent").arg("3").build();

        // let res=protocol.queue(cmd).await.unwrap();

        println!("Res {:#?}", &res);
    }
}

pub use protocol::adb_respond::AdbError;
pub use protocol::config::Config;
pub use protocol::Protocol;
