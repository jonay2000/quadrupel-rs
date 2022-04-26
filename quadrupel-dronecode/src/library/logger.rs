use crate::hardware::uart::QUart;
use core::fmt::Write;
use log::{set_logger_racy, set_max_level, LevelFilter, Log, Metadata, Record};

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
            let mut uart = QUart::get().writer();

            let _ = writeln!(&mut uart, "[{}] {}", record.level().as_str(), record.args());
        }
    }

    fn flush(&self) {}
}