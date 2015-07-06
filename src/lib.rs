#![deny(missing_doc)]
#![deny(warnings)]

//! A reader + writer stream backed by an in-memory buffer.

use std::cmp::min;
use std::io::{Result,Read,Write};

/// `MemStream` is a reader + writer stream backed by an in-memory buffer
#[derive(PartialEq, PartialOrd)]
pub struct MemStream {
    buf: Vec<u8>,
    pos: usize
}

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
    pub fn as_slice<'a>(&'a self) -> &'a [u8] { &self.buf[..] }
    /// Unwraps this `MemStream`, returning the underlying buffer
    pub fn unwrap(self) -> Vec<u8> { self.buf }
}

impl Read for MemStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.eof() { return Ok(0) }
        let write_len = min(buf.len(), self.buf.len() - self.pos);
        {   
            let input = &self.buf[self.pos .. self.pos + write_len];
            let output = &mut buf[0 .. write_len];
            assert_eq!(input.len(), output.len());

            unsafe {
                std::ptr::copy_nonoverlapping(
                    input.as_ptr(),
                    output.as_mut_ptr(),
                    input.len()
                );
            }
        }
        self.pos += write_len;
        assert!(self.pos <= self.buf.len());

        return Ok(write_len);
    }
}

impl Write for MemStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        for byte in buf {
            self.buf.push(*byte);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
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
