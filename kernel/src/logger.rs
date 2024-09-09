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
        // TODO: Add support for ansi codes
        // match record.level() {
        //     Level::Trace => set_color(Color::Pink, BG),
        //     Level::Debug => set_color(Color::LightCyan, BG),
        //     Level::Info => set_color(Color::LightGreen, BG),
        //     Level::Warn => set_color(Color::Yellow, BG),
        //     Level::Error => set_color(Color::LightRed, BG),
        // }

        print!("[{:5}]: ", record.level());

        // set_color(FG, BG);

        // let mut fb = framebuffer.lock();
        print!("{}\r\n", record.args());
    }

    fn flush(&self) {}
}

// TODO: read from bootloader init
const LOG_LEVEL: LevelFilter = LevelFilter::Trace;

pub fn init() {
    let logger = LOGGER.get_or_init(|| Logger {});

    log::set_logger(logger).expect("logger already set");
    log::set_max_level(LOG_LEVEL);
}
