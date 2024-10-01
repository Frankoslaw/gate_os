use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use owo_colors::OwoColorize;
use spin::Mutex;
use uart_16550::SerialPort;
use vte::{Params, Parser, Perform};
use x86_64::instructions::interrupts;

lazy_static! {
    pub static ref SERIAL: Mutex<Serial> = Mutex::new(Serial::new(0x3F8));
    pub static ref PARSER: Mutex<Parser> = Mutex::new(Parser::new());
}

pub struct Serial {
    port: SerialPort,
}

impl Serial {
    fn new(addr: u16) -> Self {
        Self {
            port: unsafe { SerialPort::new(addr) },
        }
    }

    fn init(&mut self) {
        self.port.init();
    }

    fn read_byte(&mut self) -> u8 {
        self.port.receive()
    }

    fn write_byte(&mut self, byte: u8) {
        self.port.send(byte);
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut parser = PARSER.lock();
        for byte in s.bytes() {
            parser.advance(self, byte); // Parse some CSI sequences
            self.write_byte(byte); // But send everything to the serial console
        }
        Ok(())
    }
}

/// See https://vt100.net/emu/dec_ansi_parser
impl Perform for Serial {
    fn csi_dispatch(&mut self, params: &Params, _: &[u8], _: bool, c: char) {
        match c {
            'h' => {
                // Enable
                #[allow(clippy::never_loop)]
                for param in params.iter() {
                    match param[0] {
                        // 12 => sys::console::enable_echo(),
                        _ => return,
                    }
                }
            }
            'l' => {
                // Disable
                #[allow(clippy::never_loop)]
                for param in params.iter() {
                    match param[0] {
                        // 12 => sys::console::disable_echo(),
                        _ => return,
                    }
                }
            }
            _ => {}
        }
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        SERIAL
            .lock()
            .write_fmt(args)
            .expect("Could not print to serial")
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        match $val {
            tmp => {
                $crate::println!(
                    "File: {} \nLine: {} \nExpression: {} \nValue: {:#?}",
                    file!(),
                    line!(),
                    stringify!($val),
                    &tmp
                );
                tmp
            }
        }
    };
    ($val:expr,) => { $crate::dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

pub fn init() {
    SERIAL.lock().init();
    println!("{}: {}", "[TRACE]".yellow(), "Serial initialized");
    // sys::idt::set_irq_handler(4, interrupt_handler);
}

fn interrupt_handler() {
    let b = SERIAL.lock().read_byte();
    if b == 0xFF {
        // Ignore invalid bytes
        return;
    }
    let c = match b as char {
        '\r' => '\n',
        '\x7F' => '\x08', // Delete => Backspace
        c => c,
    };

    // sys::console::key_handle(c);
}
