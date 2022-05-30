use crate::hardware;
use quadrupel_shared::message::MessageToDrone;

pub enum UartProtocolState {
    WaitingForMessage,
    ReceivingMessage { len: u8, received_count: u8 },
}

pub struct UartProtocol {
    state: UartProtocolState,
    buffer: [u8; 256],
}

impl UartProtocol {
    pub fn new() -> Self {
        Self {
            state: UartProtocolState::WaitingForMessage,
            buffer: [0; 256],
        }
    }
    pub fn update(&mut self) -> Option<MessageToDrone> {
        let uart = hardware::UART.as_mut_ref();
        while let Some(byte) = uart.get_byte() {
            match &mut self.state {
                UartProtocolState::WaitingForMessage => {
                    if byte != 0 {
                        self.state = UartProtocolState::ReceivingMessage {
                            len: byte,
                            received_count: 0,
                        }
                    }
                }
                UartProtocolState::ReceivingMessage {
                    len,
                    received_count,
                } => {
                    self.buffer[*received_count as usize] = byte;
                    *received_count += 1;
                    if received_count == len {
                        match MessageToDrone::decode(&self.buffer[..*len as usize]) {
                            Err(e) => {
                                log::error!(
                                    "{:?} from decoding {:?}",
                                    e,
                                    &self.buffer[..*len as usize]
                                );
                                self.state = UartProtocolState::WaitingForMessage;
                            }
                            Ok((msg, _)) => {
                                self.state = UartProtocolState::WaitingForMessage;
                                return Some(msg);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
