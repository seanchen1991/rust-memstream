//! A reader + writer stream backed by an in-memory buffer.

use std::io;
use std::io::{Read, Write};

/// `MemStream` is a reader + writer stream backed by an in-memory buffer
pub struct MemStream {
    buf: Vec<u8>
}

impl MemStream {
    /// Creates a new `MemStream` which can be read and written to
    pub fn new() -> MemStream {
        MemStream {
            buf: vec![]
        }
    }
    
    /// Unwraps this `MemStream`, returning the underlying buffer
    pub fn unwrap(self) -> Vec<u8> { self.buf }
}

impl Read for MemStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0;
        while bytes_read < buf.len() && self.buf.len() > 0 {
            buf[bytes_read] = self.buf.remove(0);
            bytes_read += 1;
        }
        Ok(bytes_read)
    }
}

impl Write for MemStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }
    
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}


#[test]
fn test_mem_stream_read_and_write() {
    let mut stream = MemStream::new();
    stream.write(&[0, 1, 2, 3]).unwrap();
    stream.write(&[4, 5, 6, 7]).unwrap();
    
    let mut buf = [];
    assert_eq!(stream.read(&mut buf).ok(), Some(0));
    
    let mut buf = [0];
    assert_eq!(stream.read(&mut buf).ok(), Some(1));
    assert_eq!(&buf, &[0]);
    
    let mut buf = [0; 4];
    assert_eq!(stream.read(&mut buf).ok(), Some(4));
    assert_eq!(&buf, &[1, 2, 3, 4]);
    
    assert_eq!(stream.read(&mut buf).ok(), Some(3));
    assert_eq!(&buf[0..3], &[5, 6, 7]);
    
    assert_eq!(stream.read(&mut buf).ok(), Some(0));
}
