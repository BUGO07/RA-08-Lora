use core::fmt::Write;

use crate::peripherals::regs::UART0;

pub struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            UART0.send_data(byte);
        }
        Ok(())
    }
}

/// Print to UART0 serial
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        ::core::fmt::Write::write_fmt(&mut $crate::print::SerialWriter, format_args!($($arg)*)).unwrap();
    };
}

/// Print to UART0 serial with newline
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::print!($($arg)*);
        $crate::print!("\n");
    };
}
