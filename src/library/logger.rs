use log::{LevelFilter, Log, Metadata, Record, set_logger_racy};
use crate::QuadrupelUART;
use core::fmt::Write;

#[cfg(not(test))]
pub static LOGGER: UartLogger = UartLogger::with_level(LevelFilter::Info);
#[cfg(test)]
pub static LOGGER: UartLogger = UartLogger::with_level(LevelFilter::Debug);

pub struct UartLogger {
    level: LevelFilter
}

impl UartLogger {
    pub const fn with_level(level: LevelFilter) -> Self {
        Self {level}
    }

    pub fn initialize() {
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
            let mut uart = QuadrupelUART::get().writer();

            let _ = write!(&mut uart, "{}: {}", record.level().as_str(), record.args());
        }
    }

    fn flush(&self) {}
}


