use core::fmt::Write;
use log::{Level, LevelFilter, Metadata, Record};

pub(crate) struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = writeln!(
                crate::stdio::Descriptor::stderr(),
                "{} - {}",
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

impl Logger {
    pub(crate) fn init() {
        static LOGGER: Logger = Logger;
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Info))
            .expect("Could not initialize logger");
    }
}
