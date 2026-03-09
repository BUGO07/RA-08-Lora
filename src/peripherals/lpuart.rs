use crate::{
    cortex::VolatileRW,
    define_reg,
    peripherals::{
        rcc::RCC_PERIPHERAL_LPUART,
        regs::{RCC, RCC_CR1_LPUART_CLK_SEL_RCO32K, RCC_CR1_LPUART_CLK_SEL_XO32K},
    },
    toggle_reg_bits,
};

/// LPUART baudrate options
#[repr(usize)]
pub enum LpuartBaudrate {
    /// 110 baud
    Baud110 = 110,
    /// 300 baud
    Baud300 = 300,
    /// 600 baud
    Baud600 = 600,
    /// 1200 baud
    Baud1200 = 1200,
    /// 2400 baud
    Baud2400 = 2400,
    /// 4800 baud
    Baud4800 = 4800,
    /// 9600 baud
    Baud9600 = 9600,
}

/// Baudrate integer mask
pub const LPUART_BAUD_RATE_INT_MASK: usize = 0x3ffc00;
/// Baudrate integer position
pub const LPUART_BAUD_RATE_INT_POS: usize = 10;
/// Baudrate fraction mask
pub const LPUART_BAUD_RATE_FRA_MASK: usize = 0x3c0;
/// Baudrate fraction position
pub const LPUART_BAUD_RATE_FRA_POS: usize = 6;

/// LPUART data width
#[repr(usize)]
pub enum LpuartDataWidth {
    /// 5-bit data width
    Data5Bit,
    /// 6-bit data width
    Data6Bit,
    /// 7-bit data width
    Data7Bit,
    /// 8-bit data width
    Data8Bit,
}

/// LPUART parity mode
#[repr(usize)]
pub enum LpuartParity {
    /// Even parity
    Even = 0x0,
    /// Odd parity
    Odd = 0x4,
    /// Stick 0 parity
    Stick0 = 0x8,
    /// Stick 1 parity
    Stick1 = 0xC,
    /// No parity
    None = 0x1c,
}

/// LPUART stop bits
#[repr(usize)]
pub enum LpuartStopBits {
    /// 1 stop bit
    Stop1Bit,
    /// 2 stop bits
    Stop2Bit,
}

/// LPUART CR0 register bit definitions
#[repr(usize)]
pub enum LpuartCr0 {
    /// RX low level wakeup
    LowLevelWakeup = 0x400000,
    /// Start bit wakeup
    StartWakeup = 0x800000,
    /// Receive done wakeup
    RxDoneWakeup = 0x1000000,
    /// Receive enable
    RxEnable = 0x2000000,
    /// RTS enable
    RtsEnable = 0x4000000,
}

/// LPUART interrupt definitions
#[repr(usize)]
pub enum LpuartInterrupt {
    /// RX start valid interrupt
    Cr1StartValid = 0x1,
    /// RX done interrupt
    Cr1RxDone = 0x2,
    /// RX start invalid interrupt
    Cr1StartInvalid = 0x4,
    /// RX parity error interrupt
    Cr1ParityError = 0x8,
    /// RX stop error interrupt
    Cr1StopError = 0x10,
    /// RX overflow interrupt
    Cr1RxOverflow = 0x20,
    /// RX not empty interrupt
    Cr1RxNotEmpty = 0x40,
    /// TX empty interrupt
    Cr1TxEmpty = 0x80,
    /// TX done interrupt
    Cr1TxDone = 0x100,
}

/// LPUART CR1 register bit definitions
#[repr(usize)]
pub enum LpuartCr1 {
    /// TX enable
    TxEnable = 0x200,
    /// CTS enable
    CtsEnable = 0x1000,
}

/// LPUART SR0 RX status flags
#[repr(usize)]
pub enum LpuartRxStatus {
    /// RX start valid
    Sr0StartValid = 0x1,
    /// RX done
    Sr0RxDone = 0x2,
    /// RX start invalid
    Sr0StartInvalid = 0x4,
    /// RX parity error
    Sr0ParityError = 0x8,
    /// RX stop error
    Sr0StopError = 0x10,
    /// RX overflow
    Sr0RxOverflow = 0x20,
}

/// LPUART SR1 register status flags
#[repr(usize)]
pub enum LpuartSr1 {
    /// Write SR0 register state
    WriteSr0 = 0x2,
    /// Write CR0 register state
    WriteCr0 = 0x4,
    /// RX not empty
    RxNotEmpty = 0x8,
    /// TX empty
    TxEmpty = 0x10,
    /// TX done
    TxDone = 0x20,
}

/// LPUART initialization configuration
pub struct LpuartConfig {
    /// Baudrate
    pub baudrate: usize,
    /// Data width
    pub data_width: LpuartDataWidth,
    /// Parity mode
    pub parity: LpuartParity,
    /// Stop bits
    pub stop_bits: LpuartStopBits,
    /// Low level wakeup enable
    pub low_level_wakeup: bool,
    /// Start wakeup enable
    pub start_wakeup: bool,
    /// RX done wakeup enable
    pub rx_done_wakeup: bool,
}

define_reg! {
    Lpuart
    __Lpuart {
        cr0: VolatileRW<usize>,
        cr1: VolatileRW<usize>,
        sr0: VolatileRW<usize>,
        sr1: VolatileRW<usize>,
        data: VolatileRW<usize>,
    }
}

impl Lpuart {
    /// Receive a byte of data, returns 0 if RX buffer is empty
    pub fn receive_data(&self) -> u8 {
        if self.get_rx_not_empty() {
            self.data.read() as u8
        } else {
            0
        }
    }

    /// Send a byte of data if the TX buffer is empty
    pub fn send_data(&self, data: u8) {
        if self.get_tx_empty() {
            self.data.write(data as usize);
        }
    }

    /// Enable or disable an interrupt source
    pub fn config_interrupt(&self, interrupt: LpuartInterrupt, enable: bool) {
        toggle_reg_bits!(self.cr1, interrupt as usize, enable);
    }

    /// Enable or disable RTS
    pub fn config_rts(&self, enable: bool) {
        while self.sr1.read() & (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
            != (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
        {}
        toggle_reg_bits!(self.cr0, LpuartCr0::RtsEnable as usize, enable);
        while self.sr1.read() & (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
            != (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
        {}
    }

    /// Enable or disable RX
    pub fn config_rx(&self, enable: bool) {
        while self.sr1.read() & (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
            != (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
        {}
        toggle_reg_bits!(self.cr0, LpuartCr0::RxEnable as usize, enable);
        while self.sr1.read() & (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
            != (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
        {}
    }

    /// Enable or disable CTS
    pub fn config_cts(&self, enable: bool) {
        toggle_reg_bits!(self.cr1, LpuartCr1::CtsEnable as usize, enable);
    }

    /// Enable or disable TX
    pub fn config_tx(&self, enable: bool) {
        toggle_reg_bits!(self.cr1, LpuartCr1::TxEnable as usize, enable);
    }

    /// Get the RX status for a given flag
    pub fn get_rx_status(&self, status: LpuartRxStatus) -> bool {
        while self.sr1.read() & LpuartSr1::WriteSr0 as usize != LpuartSr1::WriteSr0 as usize {}
        self.sr0.read() & (status as usize) != 0
    }

    /// Clear the specified RX status flag
    pub fn clear_rx_status(&self, status: LpuartRxStatus) {
        while self.sr1.read() & (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
            != (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
        {}
        self.sr0.write(status as usize);
        while self.sr1.read() & (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
            != (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
        {}
    }

    /// Check if RX buffer is not empty
    pub fn get_rx_not_empty(&self) -> bool {
        self.sr1.read() & LpuartSr1::RxNotEmpty as usize != 0
    }

    /// Check if TX buffer is empty
    pub fn get_tx_empty(&self) -> bool {
        self.sr1.read() & LpuartSr1::TxEmpty as usize != 0
    }

    /// Check if TX is done
    pub fn get_tx_done(&self) -> bool {
        self.sr1.read() & LpuartSr1::TxDone as usize != 0
    }

    /// Clear the TX done status flag
    pub fn clear_tx_done(&self) {
        toggle_reg_bits!(self.sr1, LpuartSr1::TxDone as usize, true);
    }

    /// Initialize the LPUART with the given configuration
    pub fn init(&self, config: LpuartConfig) {
        let lpuart_clk_freq = RCC.get_lpuart_clk_src();
        let freq: usize = match lpuart_clk_freq {
            RCC_CR1_LPUART_CLK_SEL_XO32K => 32768,
            RCC_CR1_LPUART_CLK_SEL_RCO32K => 32000,
            _ => 4_000_000,
        };

        let mut tmp_value: usize = 0;

        if config.low_level_wakeup {
            tmp_value |= LpuartCr0::LowLevelWakeup as usize;
        } else if config.start_wakeup {
            tmp_value |= LpuartCr0::StartWakeup as usize;
        } else if config.rx_done_wakeup {
            tmp_value |= LpuartCr0::RxDoneWakeup as usize;
        }

        // baudrate
        let ibaud = (freq / config.baudrate) as u16;
        let fbaud =
            (((freq % config.baudrate) * 16 + config.baudrate / 2) / config.baudrate) as u16;

        tmp_value |= ((ibaud as usize) << LPUART_BAUD_RATE_INT_POS)
            | ((fbaud as usize) << LPUART_BAUD_RATE_FRA_POS)
            | (config.stop_bits as usize)
            | (config.parity as usize)
            | (config.data_width as usize);

        while self.sr1.read() & (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
            != (LpuartSr1::WriteSr0 as usize | LpuartSr1::WriteCr0 as usize)
        {}
        self.cr0.write(tmp_value);
    }

    /// Deinitialize the LPUART peripheral
    pub fn deinit(&self) {
        RCC.enable_peripheral_clk(RCC_PERIPHERAL_LPUART, false);
        RCC.rst_peripheral(RCC_PERIPHERAL_LPUART, true);
        RCC.rst_peripheral(RCC_PERIPHERAL_LPUART, false);
    }
}
