use alloc::collections::VecDeque;

const RX_PIN_NUMBER: usize = 16;
const TX_PIN_NUMBER: usize = 14;


pub struct QuadrupelUART {
    uart: nrf51822::UART0,
    txd_available: bool,
    rx_queue: VecDeque<u8>,
    tx_queue: VecDeque<u8>,
}

impl QuadrupelUART {
    pub fn new(uart: nrf51822::UART0) {
        let mut obj = QuadrupelUART { uart, txd_available: true, rx_queue: VecDeque::new(), tx_queue: VecDeque::new() };


    }
    pub fn uart_put(&mut self, byte: u8) {

    }
}