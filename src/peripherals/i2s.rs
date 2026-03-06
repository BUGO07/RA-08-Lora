use crate::{
    cortex::{VolatileRO, VolatileRW},
    define_reg,
    peripherals::{rcc::RCC_PERIPHERAL_I2S, regs::RCC},
    toggle_reg_bits,
};

/// I2S interrupt sources.
#[repr(u32)]
pub enum I2sInterrupt {
    /// TX FIFO overflow interrupt.
    TxFo = 1 << 5,
    /// TX FIFO empty interrupt.
    TxFe = 1 << 4,
    /// RX FIFO overflow interrupt.
    RxFo = 1 << 1,
    /// RX available data interrupt.
    RxDa = 1 << 0,
}

/// I2S audio mode.
#[repr(u32)]
pub enum I2sMode {
    /// Left alignment mode.
    LeftAlign,
    /// Right alignment mode.
    RightAlign,
    /// Philips mode.
    Philips,
}

/// I2S FIFO trigger level.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum I2sFifoTriggerLevel {
    /// FIFO trigger level 1.
    Level1 = 1,
    /// FIFO trigger level 2.
    Level2 = 2,
    /// FIFO trigger level 3.
    Level3 = 3,
    /// FIFO trigger level 4.
    Level4 = 4,
}

/// FIFO trigger level mask.
pub const I2S_FIFO_TRIGGER_LEVEL_MASK: u32 = 0xf;

/// I2S word size.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum I2sWordSize {
    /// Don't care word size.
    DontCare,
    /// 12-bit word size.
    Bits12,
    /// 16-bit word size.
    Bits16,
    /// 20-bit word size.
    Bits20,
    /// 24-bit word size.
    Bits24,
    /// 32-bit word size.
    Bits32,
}

/// FIFO depth.
pub const FIFO_DEPTH: u32 = 4;
/// RX and TX word size mask.
pub const I2S_WORDSIZE_MASK: u32 = 0x7;

/// I2S role.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum I2sRole {
    /// I2S slave.
    Slave,
    /// I2S master.
    Master,
}

/// I2S channel selection.
#[repr(u32)]
pub enum I2sChannel {
    /// Left channel.
    Left,
    /// Right channel.
    Right,
}

/// I2S master clock source frequency for 96KHz sample rate.
pub const I2S_MCLK_SRC_FREQ17: u32 = 16934400;
/// I2S master clock source frequency for 44.1KHz sample rate.
pub const I2S_MCLK_SRC_FREQ12: u32 = 12288000;
/// I2S 44.1KHz sample rate.
pub const I2S_SAMPLE_RATE_44P1K: u32 = 44100;
/// I2S 96KHz sample rate.
pub const I2S_SAMPLE_RATE_96K: u32 = 96000;

/// I2S register bit positions.
#[repr(u32)]
pub enum I2sPos {
    /// I2S master enable position.
    MasterEnable,
    /// I2S WS frequency position.
    WsFreq,
    /// I2S WS enable position.
    WsEnable,
    /// I2S WS delay position.
    WsDelay,
}

/// I2S initialization configuration.
#[derive(Clone, Copy)]
pub struct I2sConfig {
    /// I2S role (master or slave).
    pub role: I2sRole,
    /// I2S word size.
    pub word_size: I2sWordSize,
    /// I2S FIFO trigger threshold level.
    pub fifo_threshold: I2sFifoTriggerLevel,
}

impl Default for I2sConfig {
    fn default() -> Self {
        Self {
            role: I2sRole::Master,
            word_size: I2sWordSize::Bits32,
            fifo_threshold: I2sFifoTriggerLevel::Level1,
        }
    }
}

define_reg! {
    I2s
    __I2s {
        /// enable register, offset 0x00
        ier: VolatileRW<u32>,
        /// receiver block enable register, offset 0x04
        irer: VolatileRW<u32>,
        /// transmitter block enable register, offset 0x08
        iter: VolatileRW<u32>,
        /// clock enable register, offset 0x0c
        cer: VolatileRW<u32>,
        /// clock configuration register, offset 0x10
        ccr: VolatileRW<u32>,
        /// receiver block FIFO reset register, offset
        rxffr: VolatileRW<u32>,
        /// transmitter block FIFO reset register, offset 0x18
        txffr: VolatileRW<u32>,
        /// reserved
        resv0: VolatileRO<u32>,

        /// right receive buffer register, offset 0x20
        lrbr_lthr: VolatileRW<u32>,
        /// right transmit holding register, offset 0x24
        rrbr_rthr: VolatileRW<u32>,
        /// receiver enable register, offset 0x28
        rer: VolatileRW<u32>,
        /// transmitter enable register, offset 0x2c
        ter: VolatileRW<u32>,
        /// receiver configuration register, offset 0x30
        rcr: VolatileRW<u32>,
        /// transmitter configuration register, offset 0x34
        tcr: VolatileRW<u32>,
        /// interrupt status register, offset 0x38
        isr: VolatileRO<u32>,
        /// interrupt mask register, offset 0x3c
        imr: VolatileRW<u32>,
        /// receiver overrun register, offset 0x40
        ror: VolatileRO<u32>,
        /// transmitter overrun register, offset 0x44
        tor: VolatileRO<u32>,
        /// receiver FIFO configuration register, offset 0x48
        rfcr: VolatileRW<u32>,
        /// transmitter FIFO configuration register, offset 0x4c
        tfcr: VolatileRW<u32>,
        /// receiver FIFO flush register, offset 0x50
        rff: VolatileRW<u32>,
        /// transmitter FIFO flush register, offset 0x54
        tff: VolatileRW<u32>,
        /// reserved
        resv1: [VolatileRO<u32>; 0x5a],
        /// receiver block dma register, offset 0x1c0
        rxdma: VolatileRW<u32>,
        /// reset receiver block dma register, offset 0x1c4
        rrxdma: VolatileRW<u32>,
        /// transmitter block dma register, offset 0x1c8
        txdma: VolatileRW<u32>,
        /// reset transmitter block dma register, offset 0x1cc
        rtxdma: VolatileRW<u32>,
        /// reserved
        resv2: [VolatileRO<u32>; 8],
        /// component parameter register 2, offset 0x1f0
        i2s_comp_param_2: VolatileRO<u32>,
        /// component parameter register 1, offset 0x1f4
        i2s_comp_param_1: VolatileRO<u32>,
        /// component version register, offset 0x1f8
        i2s_comp_version: VolatileRO<u32>,
        /// component type register, offset 0x1fc
        i2s_comp_type: VolatileRO<u32>,
    }
}

impl I2s {
    /// Enable or disable an I2S interrupt.
    pub fn config_interrupt(&self, interrupt: I2sInterrupt, enable: bool) {
        toggle_reg_bits!(self.imr, interrupt as u32, enable);
    }

    /// Clear an I2S interrupt by reading the corresponding register.
    pub fn clear_interrupt(&self, interrupt: I2sInterrupt) {
        // clear by reading
        match interrupt {
            I2sInterrupt::TxFo => {
                self.tor.read();
            }
            I2sInterrupt::RxFo => {
                self.ror.read();
            }
            _ => {}
        }
    }

    /// Get the interrupt status for a given I2S interrupt.
    ///
    /// Returns `true` if the interrupt is active.
    pub fn get_interrupt_status(&self, interrupt: I2sInterrupt) -> bool {
        (self.isr.read() & (interrupt as u32)) != 0
    }

    /// Enable or disable I2S.
    pub fn cmd(&self, enable: bool) {
        toggle_reg_bits!(self.ier, !0, enable);
    }

    /// Enable or disable I2S TX block.
    pub fn tx_block_cmd(&self, enable: bool) {
        toggle_reg_bits!(self.iter, !0, enable);
    }

    /// Enable or disable I2S RX block.
    pub fn rx_block_cmd(&self, enable: bool) {
        toggle_reg_bits!(self.irer, !0, enable);
    }

    /// Enable or disable I2S TX channel.
    pub fn tx_channel_cmd(&self, enable: bool) {
        toggle_reg_bits!(self.ter, !0, enable);
    }

    /// Enable or disable I2S RX channel.
    pub fn rx_channel_cmd(&self, enable: bool) {
        toggle_reg_bits!(self.rer, !0, enable);
    }

    /// Enable or disable the I2S master clock.
    pub fn master_clock_cmd(&self, enable: bool) {
        toggle_reg_bits!(self.cer, !0, enable);
    }

    /// Initialize the I2S peripheral with the given configuration.
    pub fn init(&self, config: I2sConfig) {
        self.cmd(true);

        toggle_reg_bits!(self.rcr, I2S_WORDSIZE_MASK, false);
        toggle_reg_bits!(self.rcr, config.word_size as u32, true);

        toggle_reg_bits!(self.tcr, I2S_WORDSIZE_MASK, false);
        toggle_reg_bits!(self.tcr, config.word_size as u32, true);

        toggle_reg_bits!(self.rfcr, I2S_FIFO_TRIGGER_LEVEL_MASK, false);
        toggle_reg_bits!(self.rfcr, config.fifo_threshold as u32, true);

        toggle_reg_bits!(self.tfcr, I2S_FIFO_TRIGGER_LEVEL_MASK, false);
        toggle_reg_bits!(self.tfcr, config.fifo_threshold as u32, true);
    }

    /// Deinitialize the I2S peripheral, disabling its clock and resetting registers.
    pub fn deinit(&self) {
        RCC.enable_peripheral_clk(RCC_PERIPHERAL_I2S, false);
        RCC.rst_peripheral(RCC_PERIPHERAL_I2S, true);
        RCC.rst_peripheral(RCC_PERIPHERAL_I2S, false);
    }

    /// Receive data from the specified I2S channel.
    pub fn receive_data(&self, channel: I2sChannel) -> u32 {
        match channel {
            I2sChannel::Left => self.lrbr_lthr.read(),
            I2sChannel::Right => self.rrbr_rthr.read(),
        }
    }

    /// Send data over I2S on both left and right channels.
    ///
    /// Both slices must have the same length.
    pub fn send_data(&self, left_chan_data: &[u32], right_chan_data: &[u32]) {
        let word_size = self.tcr.read() & I2S_WORDSIZE_MASK;

        for i in 0..left_chan_data.len() {
            while !self.get_interrupt_status(I2sInterrupt::TxFe) {
                // wait till tx fifo empties
            }
            if word_size > I2sWordSize::Bits16 as u32 || word_size == I2sWordSize::DontCare as u32 {
                self.lrbr_lthr.write(left_chan_data[i]);
                self.rrbr_rthr.write(right_chan_data[i]);
            } else {
                self.lrbr_lthr.write(left_chan_data[i] & 0xFFFF);
                self.rrbr_rthr.write(right_chan_data[i] & 0xFFFF);
            }
        }
    }
}

/// Calculate the SCLK frequency division factor for the given word size.
#[allow(unused)]
fn calculate_devision(word_size: I2sWordSize) -> u8 {
    match word_size {
        I2sWordSize::DontCare => 21,
        I2sWordSize::Bits12 => 13,
        I2sWordSize::Bits16 => 17,
        I2sWordSize::Bits20 => 21,
        I2sWordSize::Bits24 => 25,
        I2sWordSize::Bits32 => 33,
    }
}
