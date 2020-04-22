use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    read_count: usize,
    bytes_through: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            data: wrapped,
            read_count: 0 as usize,
            bytes_through: 0 as usize
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_count += 1;
        let result = self.data.read(buf);
        match result {
            Ok(bytes) => self.bytes_through += bytes,
            _ => self.bytes_through += 0
        }
        result
    }
}

pub struct WriteStats<W> {
    data: W,
    write_count: usize,
    bytes_through: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: wrapped,
            write_count: 0 as usize,
            bytes_through: 0 as usize
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.write_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_count += 1;

        let result = self.data.write(buf);
        match result {
            Ok(bytes) => self.bytes_through += bytes,
            _ => self.bytes_through += 0
        }
        result
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
