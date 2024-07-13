use crate::data_structures::error::OutOfBoundsError;

pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
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
