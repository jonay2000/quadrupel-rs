use crate::spi_flash::FlashError;
use crate::FLASH;
use bincode::config::standard;
use bincode::de::read::Reader;
use bincode::error::DecodeError;
use quadrupel_shared::message::FlashPacket;

pub struct FlashProtocol {
    write_address: u32,
    read_address: u32,
    done: bool,
}

impl FlashProtocol {
    pub fn new() -> Self {
        Self {
            write_address: 0,
            read_address: 0,
            done: false,
        }
    }

    #[allow(unused)]
    pub fn write(&mut self, packet: FlashPacket) {
        if self.done {
            return;
        }
        let mut slice = [0u8; 256];
        let bytes = bincode::encode_into_slice(packet, &mut slice, standard()).unwrap();
        match FLASH
            .as_mut_ref()
            .flash_write_bytes(self.write_address, &slice[..bytes])
        {
            Ok(_) => {
                self.write_address += bytes as u32;
            }
            Err(FlashError::SpiError(err)) => {
                self.done = true;
                log::error!("Flash error: {:?}. (stopping flash write)", err);
            }
            Err(FlashError::OutOfSpace) => {
                self.done = true;
                log::warn!("Flash error: Flash is full. (stopping flash write)");
            }
        }
    }

    pub fn reset(&mut self) {
        FLASH.as_mut_ref().flash_chip_erase().unwrap();
        self.write_address = 0;
        self.read_address = 0;
    }

    pub fn read(&mut self) -> Option<FlashPacket> {
        if self.read_address == self.write_address {
            return None;
        }
        Some(bincode::decode_from_reader(self, standard()).unwrap())
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Reader for FlashProtocol {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        if self.read_address == self.write_address {
            return Err(DecodeError::UnexpectedEnd);
        }
        FLASH
            .as_mut_ref()
            .flash_read_bytes(self.read_address, bytes)
            .unwrap();
        self.read_address += bytes.len() as u32;
        Ok(())
    }
}
