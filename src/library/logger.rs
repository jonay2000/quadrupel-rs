// use log::{LevelFilter, Log, Metadata, Record};
//
// #[cfg(not(test))]
// pub static LOGGER: UartLogger = UartLogger::with_level(LevelFilter::Info);
// #[cfg(test)]
// pub static LOGGER: UartLogger = UartLogger::with_level(LevelFilter::Debug);
//
// pub struct UartLogger {
//     level: LevelFilter
// }
//
// impl UartLogger {
//     pub const fn with_level(level: LevelFilter) -> Self {
//         Self {level}
//     }
// }
//
// impl Log for UartLogger {
//     fn enabled(&self, metadata: &Metadata) -> bool {
//         metadata.level() <= self.level
//     }
//
//     fn log(&self, record: &Record) {
//         if self.enabled(record.metadata()) {
//
//         }
//     }
//
//     fn flush(&self) {}
// }
//
//
