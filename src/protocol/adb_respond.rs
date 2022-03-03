use std::num::ParseIntError;


#[derive(Debug)]
pub enum AdbError<E> {
    Cmd(E),
    Io(std::io::Error),
    Parse(ParseIntError),
    Failure(String),
    Unknown
}