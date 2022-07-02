pub struct Logger;
use log::Log;

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() >= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{}, {}", record.level(), record.args());
        }
    }

    //do nothing since we logged the message using console
    fn flush(&self) {}
}
