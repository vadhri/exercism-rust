#[derive(Debug, PartialEq, Clone)]
pub struct CircularBuffer <T> {
    capacity: usize,
    buffer: Vec<T>
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: std::clone::Clone + std::fmt::Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let vector: Vec<T> = Vec::with_capacity(capacity);

        CircularBuffer {
            buffer: vector,
            capacity: capacity
        }
    }
    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            Ok(self.buffer.remove(0usize))
        }
    }
    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.buffer.len() >= self.capacity {
            Err(Error::FullBuffer)
        } else {
            self.buffer.push(_element);
            Ok(())
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.buffer.len() < self.capacity {
            let _wasted_ret = self.write(_element);
        } else {
            let _wasted_ret = self.read();
            let _wasted_ret = self.write(_element);
        }
    }
}
