pub mod adc;
pub mod i2c;
pub mod leds;
pub mod motors;
pub mod mpu6050;
pub mod ms5611;
pub mod uart;

use core::cell::{UnsafeCell};
use crate::hardware::adc::QAdc;
use crate::hardware::leds::QLeds;
use crate::hardware::uart::QUart;
use crate::Motors;
use nrf51_hal::{Twi};
use nrf51_pac::{TWI0};
use crate::hardware::mpu6050::QMpu6050;
use crate::hardware::ms5611::QMs5611;
use crate::library::once_cell::OnceCell;

pub static LEDS: OnceCell<HWCellType2<QLeds>> = OnceCell::new();
pub static UART: OnceCell<HWCellType2<QUart>> = OnceCell::new();
pub static I2C: OnceCell<HWCellType3<Twi<TWI0>>> = OnceCell::new();
pub static MPU: OnceCell<HWCellType3<QMpu6050<TWI0>>> = OnceCell::new();
pub static BARO: OnceCell<HWCellType3<QMs5611<TWI0>>> = OnceCell::new();
pub static ADC: OnceCell<HWCellType1<QAdc>> = OnceCell::new();
pub static MOTORS: OnceCell<HWCellType1<Motors>> = OnceCell::new();

pub fn init_hardware(
    mut pc: cortex_m::Peripherals,
    mut pn: nrf51_hal::pac::Peripherals,
) {
    let gpio = nrf51_hal::gpio::p0::Parts::new(pn.GPIO);

    //Motors first so we can use the timer
    MOTORS.initialize(HWCellType1 { cell: UnsafeCell::new(
        Motors::new(
            pn.TIMER1,
            pn.TIMER2,
            &mut pc.NVIC,
            &mut pn.PPI,
            &mut pn.GPIOTE,
            gpio.p0_20,
        )
    )});
    MOTORS.update_main(|motors| motors.enable());

    LEDS.initialize(HWCellType2{ cell: UnsafeCell::new(
        QLeds::new(gpio.p0_22, gpio.p0_24, gpio.p0_28, gpio.p0_30)
    )});

    UART.initialize(HWCellType2{ cell: UnsafeCell::new(
        QUart::new(pn.UART0, gpio.p0_14, gpio.p0_16, &mut pc.NVIC)
    )});
    UART.update_main(|uart| uart.enable());

    // I2C.initialize(HWCellType3{ cell: UnsafeCell::new(
    //     I2C::new(pn.TWI0, gpio.p0_04, gpio.p0_02)
    // )});
    // MPU.initialize(HWCellType3{ cell: UnsafeCell::new(
    //     QMpu6050::new(I2C.as_mut_ref())
    // )});
    // BARO.initialize(HWCellType3{ cell: UnsafeCell::new(
    //     QMs5611::new(I2C.as_mut_ref())
    // )});
    //
    // ADC.initialize(HWCellType1 { cell: UnsafeCell::new(
    //     QAdc::new(pn.ADC, &mut pc.NVIC)
    // )});
    // ADC.update_main(|adc| adc.enable());
}

pub trait HWCell<T> {
    /// Read the content of the cell.
    /// # SAFETY: This should not be called from an interrupt
    fn read_main<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        self.update_main(|t| f(t))
    }

    /// Update the content of the cell.
    /// # SAFETY: This should not be called from an interrupt
    fn update_main<U>(&self, f: impl FnOnce(&mut T) -> U) -> U;

    /// Update the content of the cell.
    /// # SAFETY: This should only be called from an interrupt
    fn update_interrupt<U>(&self, f: impl FnOnce(&mut T) -> U) -> U;

    /// Read the content of the cell from the main thread, returns a copy.
    /// # SAFETY: This should not be called from an interrupt
    fn get(&self) -> T where T: Copy {
        self.update_main(|t| *t)
    }

    fn ptr(&self) -> *mut T;
}

/// Will be accessed by both main thread + interrupts, needs sync
/// # SAFETY: Only use on single-threaded machines
pub struct HWCellType1<T> {
    cell: UnsafeCell<T>,
}
impl<T> HWCell<T> for HWCellType1<T> {
    fn update_main<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        // When accessing from main thread, we need to turn of interrupts to have sync
        // cortex_m::interrupt::free(|_| unsafe {
        //     f(&mut *self.cell.get())
        // })
        //TODO temp
        unsafe {
            f(&mut *self.cell.get())
        }
    }

    fn update_interrupt<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        unsafe { f(&mut *self.cell.get()) }
    }

    fn ptr(&self) -> *mut T {
        self.cell.get()
    }
}

/// Will be accessed by both main thread + interrupts, needs no sync (internal state is safe)
/// # SAFETY: Only use on single-threaded machines
/// # SAFETY: Only use when no internal sync is needed
pub struct HWCellType2<T> {
    cell: UnsafeCell<T>,
}
impl<T> HWCell<T> for HWCellType2<T> {
    fn update_main<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        unsafe { f(&mut *self.cell.get()) }
    }

    fn update_interrupt<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        unsafe { f(&mut *self.cell.get()) }
    }
    fn ptr(&self) -> *mut T {
        self.cell.get()
    }
}
impl<T> HWCellType2<T> {
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.cell.get() }
    }
    pub fn as_mut_ref(&self) -> &mut T {
        unsafe { &mut *self.cell.get() }
    }
}

/// Will only be accessed by main thread, so needs no sync
/// # SAFETY: Only use on single-threaded machines
/// # SAFETY: Do not access from interrupts
pub struct HWCellType3<T> {
    cell: UnsafeCell<T>,
}
impl<T> HWCell<T> for HWCellType3<T> {
    fn update_main<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        unsafe { f(&mut *self.cell.get()) }
    }

    fn update_interrupt<U>(&self, _f: impl FnOnce(&mut T) -> U) -> U {
        panic!("Tried to access hardware from interrupt for which that is not safe.");
    }

    fn ptr(&self) -> *mut T {
        self.cell.get()
    }
}
impl<T> HWCellType3<T> {
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.cell.get() }
    }
    pub fn as_mut_ref(&self) -> &mut T {
        unsafe { &mut *self.cell.get() }
    }
}