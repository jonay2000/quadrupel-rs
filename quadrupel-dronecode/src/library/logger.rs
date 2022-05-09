use crate::hardware::UART;
use alloc::format;
use log::{set_logger_racy, set_max_level, LevelFilter, Log, Metadata, Record};
use quadrupel_shared::message::MessageToComputer;

#[cfg(not(test))]
pub static LOGGER: UartLogger = UartLogger::with_level(LevelFilter::Info);
#[cfg(test)]
pub static LOGGER: UartLogger = UartLogger::with_level(LevelFilter::Debug);

pub struct UartLogger {
    level: LevelFilter,
}

impl UartLogger {
    pub const fn with_level(level: LevelFilter) -> Self {
        Self { level }
    }

    pub fn initialize() {
        set_max_level(LevelFilter::Trace);
        unsafe {
            set_logger_racy(&LOGGER).expect("failed to initialize logger");
        }
    }
}

impl Log for UartLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let x = format!("[{}] {}\n", record.level().as_str(), record.args());
            let mut f = x.as_bytes();
            let uart = UART.as_mut_ref();

            while f.len() > 200 {
                uart.send_message(MessageToComputer::Log(f[..200].to_vec()));
                f = &f[200..];
            }
            uart.send_message(MessageToComputer::Log(f.to_vec()));
        }
    }

    fn flush(&self) {}
}
