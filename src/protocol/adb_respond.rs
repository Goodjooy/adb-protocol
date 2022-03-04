use std::{num::ParseIntError, borrow::Cow};


#[derive(Debug)]
pub enum AdbError<E> {
    Cmd(E),
    Io(std::io::Error),
    Parse(ParseIntError),
    Failure(String),
    Unknown(Cow<'static,str>)
}
