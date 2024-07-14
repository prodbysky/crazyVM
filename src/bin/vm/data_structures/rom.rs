use crate::data_structures::error::OutOfBoundsError;

pub struct Rom {
    data: Vec<u32>,
}

impl Rom {
    pub fn new(data: Vec<u32>) -> Self {
        Self { data }
    }

    pub fn read(&self, index: usize) -> Result<u32, OutOfBoundsError> {
        if self.data.len() <= index {
            return Err(OutOfBoundsError(index));
        }

        Ok(self.data[index])
    }

    pub fn read_many(&self, mut index: usize, mut n: usize) -> Result<Vec<u32>, OutOfBoundsError> {
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

impl From<&[u32]> for Rom {
    fn from(value: &[u32]) -> Self {
        Self { data: value.into() }
    }
}
