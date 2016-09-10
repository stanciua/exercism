pub struct CircularBuffer<T> {
    buffer: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize) -> Self {
        CircularBuffer { buffer: Vec::with_capacity(size) }
    }
    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        Ok(self.buffer.remove(0))
    }

    pub fn write(&mut self, val: T) -> Result<(), Error> {
        if self.buffer.capacity() == self.buffer.len() {
            return Err(Error::FullBuffer);
        }
        let lgth = self.buffer.len();
        self.buffer.insert(lgth, val);


        Ok(())
    }

    pub fn overwrite(&mut self, val: T) {
        if self.buffer.capacity() != self.buffer.len() {
            self.write(val);
            return;
        }

        let _val = self.read();
        self.write(val);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}
