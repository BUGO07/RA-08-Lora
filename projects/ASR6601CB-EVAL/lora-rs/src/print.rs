use core::fmt::Write;

use crate::uart::Uart;

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.send_data(byte);
        }
        Ok(())
    }
}

pub struct StringBuffer<'a> {
    buffer: &'a mut [u8],
    pos: usize,
}

impl<'a> StringBuffer<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, pos: 0 }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.pos]).unwrap_or("")
    }
}

impl<'a> Write for StringBuffer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            if self.pos < self.buffer.len() {
                self.buffer[self.pos] = byte;
                self.pos += 1;
            }
        }
        Ok(())
    }
}

/// Print to UART0 serial
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        #[allow(unused_unsafe)] // rust bug
        ::core::fmt::Write::write_fmt(unsafe { &mut $crate::regs::UART0} , format_args!($($arg)*)).unwrap();
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

/// Format into a buffer (sprintf equivalent)
#[macro_export]
macro_rules! sprintf {
    ($buf:expr, $($arg:tt)*) => {{
        let mut sb = $crate::print::StringBuffer::new($buf);
        ::core::fmt::Write::write_fmt(&mut sb, format_args!($($arg)*)).unwrap();
        sb.as_str()
    }};
}

/// Format into a buffer with size limit (snprintf equivalent)
#[macro_export]
macro_rules! snprintf {
    ($buf:expr, $size:expr, $($arg:tt)*) => {{
        let mut sb = $crate::print::StringBuffer::new(&mut $buf[..$size]);
        ::core::fmt::Write::write_fmt(&mut sb, format_args!($($arg)*)).unwrap();
        sb.as_str()
    }};
}
