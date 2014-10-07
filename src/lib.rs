#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! A reader + writer stream backed by an in-memory buffer.

pub use self::memstream::Memstream;

mod memstream;
