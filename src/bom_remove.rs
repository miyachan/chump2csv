use std::io::{Read, Result};

use memchr::memchr;

// mysqlchump inserts a BOM marker in the dumps
// right before the INSERT. It needs to be removed
// or the parser and mysql will choke on it.
pub struct BOMRemoveRead<T: Read> {
    inner: T,
}

impl<T: Read> BOMRemoveRead<T> {
    pub fn new(r: T) -> Self {
        Self { inner: r }
    }
}

impl<T: Read> Read for BOMRemoveRead<T> {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        let result = self.inner.read(&mut buf)?;
        match memchr(0xEF, buf) {
            Some(p) if p + 2 < buf.len() => {
                if buf[p + 1] == 0xBB && buf[p + 2] == 0xBF {
                    buf.copy_within(p + 3.., p);
                    let result = result - 3;
                    Ok(result)
                } else {
                    Ok(result)
                }
            }
            _ => Ok(result),
        }
    }
}
