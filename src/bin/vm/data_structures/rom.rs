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
}
