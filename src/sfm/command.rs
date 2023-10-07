use std::time::Duration;

use crate::sfm::driver::SfmUart;
use crate::SfmError;
use log::info;

use super::ack::Ack;

pub type CommandBuffer = [u8; 8];

/**
 * Implementation based on:
 * https://github.com/Matrixchung/SFM-V1.7/blob/96612f1b4581ca5a088e9c6f9fffc8d0c1fb499c/src/sfm.cpp#L178
 */
pub fn send_command(
    driver: &dyn SfmUart,
    command: u8,
    params: [u8; 3],
    timeout: Duration,
) -> Result<(), SfmError> {
    // flush rx buffer
    let mut command_buffer = CommandBuffer::default();

    command_buffer[1] = command;
    command_buffer[2] = params[0];
    command_buffer[3] = params[1];
    command_buffer[4] = params[2];
    command_buffer[6] = get_check_sum(&command_buffer);

    driver.flush_write()?;
    let write = driver.write(&command_buffer)?;

    info!("write {}: {:?}", write, command_buffer);

    Ok(())

    //Ack::read(driver, timeout)
}

/**
 * Implementation based on:
 * https://github.com/Matrixchung/SFM-V1.7/blob/96612f1b4581ca5a088e9c6f9fffc8d0c1fb499c/src/sfm.cpp#L248
 */
pub fn get_check_sum(buffer: &CommandBuffer) -> u8 {
    let mut result = 0x00;

    for i in 1..=5 {
        result ^= buffer[i];
    }

    result
}
