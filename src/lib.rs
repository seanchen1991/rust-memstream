#![license = "MIT"]
#![deny(missing_docs)]
#![deny(warnings)]

//! A reader + writer stream backed by an in-memory buffer.

use std::io;
use std::slice;
use std::cmp::min;
use std::io::IoResult;

/// `MemStream` is a reader + writer stream backed by an in-memory buffer
pub struct MemStream {
    buf: Vec<u8>,
    pos: uint
}

#[deriving(PartialOrd)]
impl MemStream {
    /// Creates a new `MemStream` which can be read and written to
    pub fn new() -> MemStream {
        MemStream {
            buf: vec![],
            pos: 0
        }
    }
    /// Tests whether this stream has read all bytes in its ring buffer
    /// If `true`, then this will no longer return bytes from `read`
    pub fn eof(&self) -> bool { self.pos >= self.buf.len() }
    /// Acquires an immutable reference to the underlying buffer of
    /// this `MemStream`
    pub fn as_slice<'a>(&'a self) -> &'a [u8] { self.buf.as_slice() }
    /// Unwraps this `MemStream`, returning the underlying buffer
    pub fn unwrap(self) -> Vec<u8> { self.buf }
}

impl Reader for MemStream {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        if self.eof() { return Err(io::standard_error(io::EndOfFile)) }
        let write_len = min(buf.len(), self.buf.len() - self.pos);
        {
            let input = self.buf.slice(self.pos, self.pos + write_len);
            let output = buf.slice_mut(0, write_len);
            assert_eq!(input.len(), output.len());
            slice::bytes::copy_memory(output, input);
        }
        self.pos += write_len;
        assert!(self.pos <= self.buf.len());

        return Ok(write_len);
    }
}

impl Writer for MemStream {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        self.buf.push_all(buf);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use MemStream;

    #[test]
    fn test_mem_stream_read_and_write() {
        let mut stream = MemStream::new();
        stream.write([0, 1, 2, 3]).unwrap();
        stream.write([4, 5, 6, 7]).unwrap();
        let b: &[_] = &[0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(stream.as_slice(), b);
        let mut buf = [];
        assert_eq!(stream.read(buf), Ok(0));
        let mut buf = [0];
        assert_eq!(stream.read(buf), Ok(1));
        let b: &[_] = &[0];
        assert_eq!(buf.as_slice(), b);
        let mut buf = [0, ..4];
        assert_eq!(stream.read(buf), Ok(4));
        let b: &[_] = &[1, 2, 3, 4];
        assert_eq!(buf.as_slice(), b);
        assert_eq!(stream.read(buf), Ok(3));
        let b: &[_] = &[5, 6, 7];
        assert_eq!(buf.slice(0, 3), b);
        assert!(stream.read(buf).is_err());
    }
}
