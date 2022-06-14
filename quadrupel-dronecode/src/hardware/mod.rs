pub mod adc;
pub mod i2c;
pub mod leds;
pub mod motors;
pub mod mpu6050;
pub mod ms5611;
pub mod spi_flash;
pub mod uart;
pub mod time;

use crate::hardware::adc::QAdc;
use crate::hardware::leds::QLeds;
use crate::hardware::mpu6050::QMpu6050;
use crate::hardware::ms5611::QMs5611;
use crate::hardware::uart::QUart;
use crate::i2c::I2C;
use crate::library::once_cell::OnceCell;
use crate::spi_flash::SpiFlash;
use crate::Motors;
use cortex_m::asm;
use nrf51_hal::pac::TWI0;
use nrf51_hal::Twi;
use crate::library::hwcell::{HWCell, SafeFromInterrupt, SafeWhenInterruptsOff, UnsafeFromInterrupt};
use crate::time::GlobalTime;

pub static TIME: OnceCell<HWCell<UnsafeFromInterrupt, GlobalTime>> = OnceCell::new();
pub static LEDS: OnceCell<HWCell<SafeFromInterrupt, QLeds>> = OnceCell::new();
pub static UART: OnceCell<HWCell<SafeFromInterrupt, QUart>> = OnceCell::new();
pub static I2C: OnceCell<HWCell<UnsafeFromInterrupt, Twi<TWI0>>> = OnceCell::new();
pub static MPU: OnceCell<HWCell<UnsafeFromInterrupt, QMpu6050<TWI0>>> = OnceCell::new();
pub static BARO: OnceCell<HWCell<UnsafeFromInterrupt, QMs5611<TWI0>>> = OnceCell::new();
pub static ADC: OnceCell<HWCell<SafeWhenInterruptsOff, QAdc>> = OnceCell::new();
pub static MOTORS: OnceCell<HWCell<SafeWhenInterruptsOff, Motors>> = OnceCell::new();
pub static FLASH: OnceCell<HWCell<UnsafeFromInterrupt, SpiFlash>> = OnceCell::new();

pub fn init_hardware(mut pc: cortex_m::Peripherals, mut pn: nrf51_hal::pac::Peripherals) {
    let gpio = nrf51_hal::gpio::p0::Parts::new(pn.GPIO);

    UART.initialize(HWCell::new(
        QUart::new(pn.UART0, gpio.p0_14, gpio.p0_16, &mut pc.NVIC)
    ));
    UART.update_main(|uart| uart.enable());
    log::info!("UART OK");
    asm::delay(100_000);

    LEDS.initialize(HWCell::new(
        QLeds::new(gpio.p0_22, gpio.p0_24, gpio.p0_28, gpio.p0_30)
    ));
    log::info!("LEDS OK");
    asm::delay(100_000);

    TIME.initialize(HWCell::new(
        GlobalTime::new(pn.TIMER0)
    ));
    log::info!("TIME OK");
    asm::delay(100_000);

    MOTORS.initialize(HWCell::new(
        Motors::new(
            pn.TIMER1,
            pn.TIMER2,
            &mut pc.NVIC,
            &mut pn.PPI,
            &mut pn.GPIOTE,
            gpio.p0_20,
        )
    ));
    MOTORS.update_main(|motors| motors.enable());
    log::info!("MOTORS OK");
    asm::delay(100_000);

    I2C.initialize(HWCell::new(
        I2C::new(pn.TWI0, gpio.p0_04, gpio.p0_02)
    ));
    log::info!("I2C OK");
    asm::delay(100_000);

    MPU.initialize(HWCell::new(
        QMpu6050::new(I2C.as_mut_ref())
    ));
    log::info!("MPU OK");
    asm::delay(100_000);

    BARO.initialize(HWCell::new(
        QMs5611::new(I2C.as_mut_ref())
    ));
    log::info!("BARO OK");
    asm::delay(100_000);

    ADC.initialize(HWCell::new(
        QAdc::new(pn.ADC, &mut pc.NVIC),
    ));
    ADC.update_main(|adc| adc.enable());
    log::info!("ADC OK");
    asm::delay(100_000);

    FLASH.initialize(HWCell::new(
        SpiFlash::new(
            pn.SPI1, gpio.p0_17, gpio.p0_18, gpio.p0_00, gpio.p0_13, gpio.p0_11, gpio.p0_09,
        )
        .unwrap(),
    ));
    log::info!("FLASH OK");
    asm::delay(100_000);
}
