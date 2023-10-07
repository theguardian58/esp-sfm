use super::error::SfmError;
use esp_idf_hal::uart::UartDriver;

pub trait SfmUart {
    fn remaining_read(&self) -> Result<usize, SfmError>;
    fn flush_read(&self) -> Result<(), SfmError>;
    fn flush_write(&self) -> Result<(), SfmError>;
    fn read(&self, buf: &mut [u8], timeout_ms: u32) -> Result<usize, SfmError>;
    fn write(&self, buf: &[u8]) -> Result<usize, SfmError>;
}

pub struct SfmUartDriver<'a>(pub UartDriver<'a>);

impl SfmUart for SfmUartDriver<'_> {
    fn remaining_read(&self) -> Result<usize, SfmError> {
        Ok(self.0.remaining_read()?)
    }

    fn flush_read(&self) -> Result<(), SfmError> {
        Ok(self.0.flush_read()?)
    }

    fn flush_write(&self) -> Result<(), SfmError> {
        Ok(self.0.flush_write()?)
    }

    fn read(&self, buf: &mut [u8], timeout_ms: u32) -> Result<usize, SfmError> {
        Ok(self.0.read(buf, timeout_ms)?)
    }

    fn write(&self, buf: &[u8]) -> Result<usize, SfmError> {
        Ok(self.0.write(buf)?)
    }
}
