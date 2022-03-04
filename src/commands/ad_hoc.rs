use std::convert::Infallible;

#[macro_export]
macro_rules! cmd_generate {
    (
        $(#[$mm:meta])*
        $vv:vis $name:ident 
        [$cmd:literal]
        <$out:ty,$err:ty>
        $handle:expr
    ) => {
        /// 通过宏生成的Cmd
        ///
        /// ---
        ///
        $(#[$mm])*
        $vv struct $name;

        impl $crate::commands::RespHandler for $name {
            type Fut<'r> = impl std::future::Future<Output=core::result::Result<(Self::Resp, tokio::net::tcp::ReadHalf<'r>), Self::Error>>;
            type Resp = $out;
            type Error = $err;
            fn handle<'r>(reader: tokio::net::tcp::ReadHalf<'r>) -> Self::Fut<'r> {
                $handle(reader)
            }
        }

        impl $crate::commands::Cmd for $name{
            fn cmd(self) -> std::borrow::Cow<'static, str> {
                $cmd.into()
             }
        }
    };
}

cmd_generate!(
    /// 不是真的
    pub Mock 
    ["host:mock"]
    <(),Infallible>
    |reader|async{
        Ok(((),reader))
    }
);
