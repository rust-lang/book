use std::fmt;

// ANCHOR: here
type Result<T> = std::result::Result<T, std::io::Error>;
// ANCHOR_END: here

// ANCHOR: there
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
// ANCHOR_END: there
