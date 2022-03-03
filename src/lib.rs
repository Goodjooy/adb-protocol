//! ref
//! [Protocol](https://github.com/aosp-mirror/platform_system_core/blob/34a0e57a257f0081c672c9be0e87230762e677ca/adb/OVERVIEW.TXT)
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

mod protocol;
#[cfg(test)]
mod tests {
    use crate::protocol::{commands::derives::Derives, config::Config, Protocol};

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

        let fut = protocol.queue(Derives);
        let res = fut.await;

        println!("Res {:#?}", &res);
    }
}
