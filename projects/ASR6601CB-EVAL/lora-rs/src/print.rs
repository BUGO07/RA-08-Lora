use core::fmt::Write;

use crate::peripherals::regs::Uart;

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.send_data(byte);
        }
        Ok(())
    }
}

/// Print to UART0 serial
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        #[allow(unused_unsafe)] // rust bug
        ::core::fmt::Write::write_fmt(unsafe { &mut $crate::peripherals::regs::UART0} , format_args!($($arg)*)).unwrap();
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
