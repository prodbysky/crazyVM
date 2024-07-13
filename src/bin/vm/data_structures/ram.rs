pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn write(&mut self, byte: u8, index: usize) -> Result<(), OutOfBoundsError> {
        if index >= self.data.len() {
            return Err(OutOfBoundsError(index));
        }
        self.data[index] = byte;
        Ok(())
    }

    pub fn write_many(&mut self, bytes: &[u8], mut index: usize) -> Result<(), OutOfBoundsError> {
        if index + bytes.len() > self.data.len() {
            return Err(OutOfBoundsError(index));
        }

        for byte in bytes {
            self.write(*byte, index)?;
            index += 1;
        }

        Ok(())
    }

    pub fn read(&self, index: usize) -> Result<u8, OutOfBoundsError> {
        if self.data.len() <= index {
            return Err(OutOfBoundsError(index));
        }

        Ok(self.data[index])
    }

    pub fn read_many(&self, mut index: usize, mut n: usize) -> Result<Vec<u8>, OutOfBoundsError> {
        if self.data.len() <= index {
            return Err(OutOfBoundsError(index));
        }

        let mut buf = Vec::with_capacity(n);

        while n > 0 {
            buf.push(self.read(index)?);
            index += 1;
            n -= 1;
        }

        Ok(buf)
    }
}
