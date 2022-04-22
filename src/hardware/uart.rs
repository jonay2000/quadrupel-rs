use crate::hardware::gpio::QuadrupelGPIOPin;
use crate::utils::array_queue::ArrayQueue;
use cortex_m::peripheral::NVIC;
use nrf51822::interrupt;
use nrf51822::{Interrupt, Peripherals};

pub struct QuadrupelUART {
    uart: nrf51822::UART0,
}

static mut TXD_AVAILABLE: bool = true;
static mut RX_QUEUE: ArrayQueue<256> = ArrayQueue::new();
static mut TX_QUEUE: ArrayQueue<256> = ArrayQueue::new();

impl QuadrupelUART {
    pub fn new(
        uart: nrf51822::UART0,
        mut tx_pin: QuadrupelGPIOPin,
        mut rx_pin: QuadrupelGPIOPin,
        nvic: &mut NVIC,
    ) -> Self {
        tx_pin.set_mode_write();
        rx_pin.set_mode_read();

        // Specify tx/rx pins
        uart.pseltxd
            .write(|w| unsafe { w.bits(tx_pin.pin() as u32) });
        uart.pselrxd
            .write(|w| unsafe { w.bits(rx_pin.pin() as u32) });

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

        //Configure NVIC correctly
        NVIC::unpend(Interrupt::UART0);
        unsafe { nvic.set_priority(Interrupt::UART0, 3) };
        unsafe {
            NVIC::unmask(Interrupt::UART0);
        }

        QuadrupelUART { uart }
    }

    pub fn put_byte(&mut self, byte: u8) {
        cortex_m::interrupt::free(|_| unsafe {
            // We are in a no-interrupts section, so we can safely mutate globals!
            // Can we put it in the UART immediately?
            if TXD_AVAILABLE {
                TXD_AVAILABLE = false;
                self.uart.txd.write(|w| w.txd().bits(byte));
            } else {
                TX_QUEUE.enqueue(byte);
            }
        });
    }

    pub fn put_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.put_byte(*byte);
        }
    }

    pub fn get_byte(&mut self) -> Option<u8> {
        cortex_m::interrupt::free(|_| unsafe {
            // We are in a no-interrupts section, so we can safely read the RX queue
            RX_QUEUE.dequeue()
        })
    }
}

#[interrupt]
unsafe fn UART0() {
    //We are the only thing running, so we can access the uart safely
    let uart = Peripherals::steal().UART0;

    //Ready to read a bit
    if uart.events_rxdrdy.read().bits() != 0 {
        uart.events_rxdrdy.reset();
        RX_QUEUE.enqueue(uart.rxd.read().rxd().bits());
    }

    //Ready to write a bit
    if uart.events_txdrdy.read().bits() != 0 {
        uart.events_txdrdy.reset();
        match TX_QUEUE.dequeue() {
            Some(byte) => uart.txd.write(|w| w.txd().bits(byte)),
            None => TXD_AVAILABLE = true,
        }
    }

    //Ready to process an error
    if uart.events_error.read().bits() != 0 {
        uart.events_error.reset();
        panic!(
            "Uart error: (Framing: {}) (Overrun: {}) (Parity: {})",
            uart.errorsrc.read().framing().bit(),
            uart.errorsrc.read().overrun().bit(),
            uart.errorsrc.read().parity().bit()
        )
    }
}
