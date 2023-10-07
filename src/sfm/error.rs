use esp_idf_sys::EspError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SfmError {
    #[error("Error from Esp32 drivers: {0}")]
    Esp(#[from] EspError),
    #[error("Timeout waiting for ACK")]
    AckTimeout,
    #[error("Failed waiting for ACK - not enought data")]
    AckMissingData,
    #[error("ACK checksum mismatch")]
    AckChecksumMismatch,
}
