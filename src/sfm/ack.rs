use std::time::Duration;

use crate::sfm::command::get_check_sum;
use crate::sfm::command::CommandBuffer;
use crate::sfm::driver::SfmUart;
use esp_idf_hal::delay::FreeRtos;

use crate::SfmError;

static ACK_START: u8 = 0xF5;
static DELAY_DURATION_MS: u32 = 100;

/**
 * Implementation based on:
 * https://github.com/Matrixchung/SFM-V1.7/blob/96612f1b4581ca5a088e9c6f9fffc8d0c1fb499c/src/sfm.cpp#L178
 */
pub struct Ack {
    type_received: u8,
    q1: u8,
    q2: u8,
}

impl Ack {
    pub fn read(driver: &dyn SfmUart, timeout: Duration) -> Result<Self, SfmError> {
        let mut timeout_ms = timeout.as_millis() as u32;
        let mut buffer = CommandBuffer::default();
        let mut message: Vec<u8> = Vec::with_capacity(8);
        let mut read_ack_start = false;

        while message.len() < 7 && timeout_ms > 0 {
            // Waits for data to be available:
            let ready_to_read_count = driver.remaining_read()?;
            if (ready_to_read_count < 7 && !read_ack_start)
                || (ready_to_read_count < 1 && read_ack_start)
            {
                timeout_ms -= DELAY_DURATION_MS;
                FreeRtos::delay_ms(DELAY_DURATION_MS);
                continue;
            }
            match driver.read(&mut buffer, timeout_ms)? {
                0 => {
                    driver.flush_read()?;
                    return Err(SfmError::AckTimeout);
                }
                // Reads bytes until the ACK is found:
                _ => {
                    for bite in buffer {
                        if !read_ack_start {
                            read_ack_start = bite == ACK_START;
                        }
                        if read_ack_start {
                            message.push(bite);
                        }
                        if message.len() >= 8 {
                            break;
                        }
                    }
                    if message.len() < 7 {
                        buffer = CommandBuffer::default();
                        timeout_ms -= DELAY_DURATION_MS;
                        FreeRtos::delay_ms(DELAY_DURATION_MS);
                        continue;
                    }
                    if message[6] != get_check_sum(&buffer) {
                        driver.flush_read()?;
                        return Err(SfmError::AckChecksumMismatch);
                    }
                }
            }
        }
        if timeout_ms <= 0 {
            driver.flush_read()?;
            return Err(SfmError::AckTimeout);
        }

        Ok(Ack {
            type_received: message[1],
            q1: message[2],
            q2: message[3],
        })
    }
}
