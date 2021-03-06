use crate::hardware::UART;
use crate::library::logger::UartLogger;
use crate::Level;
use bincode::enc::write::Writer;
use bincode::error::EncodeError;
use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use nrf51_hal::gpio::p0::{P0_14, P0_16};
use nrf51_hal::gpio::Disconnected;
use nrf51_pac::interrupt;
use nrf51_pac::Interrupt;
use quadrupel_shared::message::MessageToComputer;
use ringbuffer::{ConstGenericRingBuffer, RingBuffer, RingBufferRead, RingBufferWrite};

const BUFFER_SIZE: usize = 256;

/// Can be used for interfacing with the UART.
/// It uses an interrupt to send bytes, when they're ready to send.
pub struct QUart {
    uart: nrf51_pac::UART0,
    rx_queue: ConstGenericRingBuffer<u8, BUFFER_SIZE>,
    tx_queue: ConstGenericRingBuffer<u8, BUFFER_SIZE>,

    /// True if the uart system is not busy, marking we can immediately
    /// write the first byte to the UART register. Subsequent bytes are put in a queue
    /// and written when UART becomes available, until none are left and this flag is set
    /// again.
    tx_data_available: bool,
}

impl QUart {
    /// Create a new instance of the UART controller. This function
    /// can only be called once cince UART0 only exists once.
    pub fn new(
        uart: nrf51_pac::UART0,
        tx_pin: P0_14<Disconnected>,
        rx_pin: P0_16<Disconnected>,
        nvic: &mut NVIC,
    ) -> Self {
        let _tx_pin = tx_pin.into_push_pull_output(Level::Low);
        let _rx_pin = rx_pin.into_floating_input();

        // Specify tx/rx pins
        uart.pseltxd.write(|w| unsafe { w.bits(14) });
        uart.pselrxd.write(|w| unsafe { w.bits(16) });

        //Set baudrate and start
        uart.baudrate.write(|w| w.baudrate().baud115200());
        uart.enable.write(|w| w.enable().enabled());

        // We want to use the interrupt system
        // First, clear any existing interrupts
        uart.events_rxdrdy.reset();
        uart.events_txdrdy.reset();
        uart.events_error.reset();

        // Enable the tx/rx interrupt sources
        uart.tasks_starttx.write(|w| unsafe { w.bits(1) });
        uart.tasks_startrx.write(|w| unsafe { w.bits(1) });

        // Disable all interrupts, then enable txdrdy rxdrdry & error, so we have only those
        uart.intenclr.write(|w| unsafe { w.bits(u32::MAX) });
        uart.intenset
            .write(|w| w.rxdrdy().set_bit().txdrdy().set_bit().error().set_bit());

        //Configure interrupt
        NVIC::unpend(Interrupt::UART0);
        unsafe { nvic.set_priority(Interrupt::UART0, 3) };

        //Init global state
        QUart {
            uart,
            rx_queue: ConstGenericRingBuffer::new_const(),
            tx_queue: ConstGenericRingBuffer::new_const(),
            tx_data_available: true,
        }
    }

    pub fn enable(&mut self) {
        //Start interrupt
        unsafe {
            NVIC::unmask(Interrupt::UART0);
        }

        //Configure logging crate
        UartLogger::initialize();

        log::info!("UART init.");
    }

    /// Pushes a single byte over uart
    pub fn put_byte(&mut self, byte: u8) {
        cortex_m::interrupt::free(|_| {
            if self.tx_data_available {
                self.tx_data_available = false;
                self.uart.txd.write(|w| unsafe { w.txd().bits(byte) });
            } else {
                self.tx_queue.push(byte);
            }
        })
    }

    /// Pushes multiple bytes over uart
    pub fn put_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.put_byte(*byte);
        }
    }

    pub fn get_byte(&mut self) -> Option<u8> {
        cortex_m::interrupt::free(|_| self.rx_queue.dequeue())
    }

    pub fn send_message(&mut self, msg: MessageToComputer) {
        msg.encode(self).expect("encoding error");
    }

    pub fn buffer_left_tx(&mut self) -> usize {
        BUFFER_SIZE - self.tx_queue.len()
    }

    pub fn buffer_left_rx(&mut self) -> usize {
        BUFFER_SIZE - self.rx_queue.len()
    }
}

impl Write for QUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.put_bytes(s.as_bytes());
        Ok(())
    }
}

impl Writer for QUart {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.put_bytes(bytes);
        Ok(())
    }
}

#[interrupt]
unsafe fn UART0() {
    let uart = UART.as_mut_ref();

    //Ready to read a bit
    if uart.uart.events_rxdrdy.read().bits() != 0 {
        uart.uart.events_rxdrdy.reset();
        let byte = uart.uart.rxd.read().rxd().bits();

        uart.rx_queue.push(byte);
    }

    //Ready to write a bit
    if uart.uart.events_txdrdy.read().bits() != 0 {
        uart.uart.events_txdrdy.reset();
        match uart.tx_queue.dequeue() {
            Some(byte) => uart.uart.txd.write(|w| w.txd().bits(byte)),
            None => uart.tx_data_available = true,
        }
    }

    //Ready to process an error
    if uart.uart.events_error.read().bits() != 0 {
        uart.uart.events_error.reset();
        //TODO log somehow
        panic!("Uart error")
    }
}
