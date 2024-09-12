use conquer_once::spin::OnceCell;
use log::LevelFilter;

use crate::print;

pub struct Logger;

pub static LOGGER: OnceCell<Logger> = OnceCell::uninit();

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        print!("[{:5}]: ", record.level());
        print!("{}\r\n", record.args());
    }

    fn flush(&self) {}
}

const LOG_LEVEL: LevelFilter = LevelFilter::Trace;

pub fn init() {
    let logger = LOGGER.get_or_init(|| Logger {});

    log::set_logger(logger).expect("logger already set");
    log::set_max_level(LOG_LEVEL);

    log::trace!("Logger initialized")
}
