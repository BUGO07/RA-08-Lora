use crate::{
    cortex::{VolatileRO, VolatileRW},
    define_reg,
    peripherals::{
        rcc::{
            RCC_PCLK0, RCC_PCLK1, RCC_PERIPHERAL_I2C0, RCC_PERIPHERAL_I2C1, RCC_PERIPHERAL_I2C2,
        },
        regs::{
            I2C_CR_ACKNAK_MASK, I2C_CR_FIFO_EN_MASK, I2C_CR_MASTER_ABORT_MASK,
            I2C_CR_MASTER_STOP_DET_EN_MASK, I2C_CR_SCL_EN_MASK, I2C_CR_START_MASK,
            I2C_CR_STOP_MASK, I2C_CR_TRANS_BEGIN_MASK, I2C_CR_TRANS_BYTE_MASK,
            I2C_CR_TWSI_UNIT_EN_MASK, I2C_CR_UNIT_RESET_MASK, I2C_RFIFO_STATUS_SIZE_MASK,
            I2C_SR_UNIT_BUSY_MASK, I2C_WFIFO_CONTROL_ACKNAK_MASK, I2C_WFIFO_CONTROL_START_MASK,
            I2C_WFIFO_CONTROL_STOP_MASK, I2C_WFIFO_CONTROL_TB_MASK, I2C0, I2C0_BASE, I2C1_BASE,
            I2C2_BASE, RCC,
        },
    },
    toggle_reg_bits,
};

/// I2C mode.
#[repr(usize)]
pub enum I2cMode {
    /// Master mode.
    Master,
    /// Slave mode.
    Slave,
}

/// I2C speed.
#[repr(usize)]
pub enum I2cSpeed {
    /// Standard speed (100K).
    Standard = 0x00000000,
    /// Fast speed (400K).
    Fast = 0x00000100,
}

/// I2C read/write direction.
#[repr(usize)]
pub enum I2cRW {
    /// I2C write data.
    Write,
    /// I2C read data.
    Read,
}

/// I2C ACK flag.
#[repr(usize)]
pub enum I2cAck {
    /// I2C NAK.
    Nak,
    /// I2C ACK.
    Ack,
}

/// I2C interrupts.
#[repr(usize)]
pub enum I2cInterrupt {
    /// Arbitration loss.
    ArbitrationLoss = 18,
    /// Transmit data empty.
    TransEmpty = 19,
    /// New data received.
    RecvFull = 20,
    /// Bus error is detected.
    BusErrorDet = 22,
    /// Address is detected in slave mode.
    SlaveAddrDet = 23,
    /// Stop is detected in slave mode.
    SlaveStopDet = 24,
    /// Stop is sent in master mode.
    MasterStopDet = 25,
    /// Transaction is done.
    TransDone = 27,
    /// Transmit FIFO is empty.
    TfifoEmpty = 28,
    /// Receive FIFO is half-full.
    RFifoHFull = 29,
    /// Receive FIFO is full.
    RFifoFull = 30,
    /// Receive FIFO is overrun.
    RFifoOverrun = 31,
}

/// I2C flags.
#[repr(usize)]
pub enum I2cFlag {
    /// Receive FIFO is empty.
    RFifoEmpty = 0,
    /// I2C unit is busy.
    UnitBusy = 15,
    /// I2C bus is busy.
    BusBusy = 16,
    /// Arbitration loss.
    ArbitrationLoss = 18,
    /// Transmit data empty.
    TransEmpty = 19,
    /// New data received.
    RecvFull = 20,
    /// Bus error is detected.
    BusErrorDet = 22,
    /// Address is detected in slave mode.
    SlaveAddrDet = 23,
    /// Stop is detected in slave mode.
    SlaveStopDet = 24,
    /// Stop is sent in master mode.
    MasterStopDet = 25,
    /// Transaction is done.
    TransDone = 27,
    /// Transmit FIFO is empty.
    TfifoEmpty = 28,
    /// Receive FIFO is half-full.
    RFifoHFull = 29,
    /// Receive FIFO is full.
    RFifoFull = 30,
    /// Receive FIFO is overrun.
    RFifoOverrun = 31,
}

/// I2C mode-specific settings.
pub enum I2cSettings {
    /// Master mode settings.
    Master { speed: I2cSpeed },
    /// Slave mode settings.
    Slave { slave_addr: usize },
}

/// I2C configuration.
pub struct I2cConfig {
    /// I2C mode (master or slave).
    pub mode: I2cMode,
    /// Enable FIFO mode.
    pub fifo_mode_en: bool,
    /// Mode-specific settings.
    pub settings: I2cSettings,
}

impl Default for I2cConfig {
    fn default() -> Self {
        Self {
            mode: I2cMode::Master,
            fifo_mode_en: false,
            settings: I2cSettings::Master {
                speed: I2cSpeed::Standard,
            },
        }
    }
}

/// Check whether FIFO mode is enabled in the CR register.
fn is_fifo_mode(cr: usize) -> bool {
    cr & I2C_CR_FIFO_EN_MASK != 0
}

/// I2C error types.
pub enum I2cError {
    /// Unit reset timed out (unit busy).
    UnitResetError,
}

define_reg! {
    I2c
    __I2c {
        cr: VolatileRW<usize>,
        sr: VolatileRW<usize>,
        sar: VolatileRW<usize>,
        dbr: VolatileRW<usize>,
        lcr: VolatileRW<usize>,
        wcr: VolatileRW<usize>,
        rst_cycl: VolatileRW<usize>,
        bmr: VolatileRO<usize>,
        wfifo: VolatileRW<usize>,
        wfifo_wprt: VolatileRW<usize>,
        wfifo_rptr: VolatileRW<usize>,
        rfifo: VolatileRW<usize>,
        rfifo_wptr: VolatileRW<usize>,
        rfifo_rptr: VolatileRW<usize>,
        resv: [VolatileRW<usize>; 2],
        wfifo_status: VolatileRO<usize>,
        rfifo_status: VolatileRO<usize>,
    }
}

impl I2c {
    /// Reset the I2C unit.
    ///
    /// Waits up to `timeout` iterations for the unit to become non-busy,
    /// then performs a reset sequence. Returns an error if the unit is still
    /// busy after the timeout expires.
    pub fn unit_reset(&self, timeout: usize) -> Result<(), I2cError> {
        let mut temp = timeout;
        while (self.sr.read() & I2C_SR_UNIT_BUSY_MASK != 0) && temp != 0 {
            temp -= 1;
        }

        if temp == 0 {
            return Err(I2cError::UnitResetError);
        }

        toggle_reg_bits!(self.cr, !I2C_CR_UNIT_RESET_MASK, false);
        toggle_reg_bits!(self.cr, I2C_CR_UNIT_RESET_MASK, true);

        self.sr.write(self.sr.read());
        toggle_reg_bits!(self.cr, I2C_CR_UNIT_RESET_MASK, false);

        Ok(())
    }

    /// Deinitialize the I2C peripheral registers to the reset values.
    pub fn deinit(&self) {
        let peripheral = match self.ptr() as usize {
            I2C0_BASE => RCC_PERIPHERAL_I2C0,
            I2C1_BASE => RCC_PERIPHERAL_I2C1,
            I2C2_BASE => RCC_PERIPHERAL_I2C2,
            _ => unreachable!(),
        };

        RCC.enable_peripheral_clk(peripheral, false);
        RCC.rst_peripheral(peripheral, true);
        RCC.rst_peripheral(peripheral, false);
    }

    /// Initialize the I2C peripheral according to the specified configuration.
    pub fn init(&self, config: I2cConfig) {
        self.unit_reset(1000).ok(); // c code discards error too

        toggle_reg_bits!(self.cr, I2C_CR_FIFO_EN_MASK, config.fifo_mode_en);

        let clk_freq = RCC.get_clk_freq(if self.ptr() as usize == I2C0_BASE {
            RCC_PCLK0
        } else {
            RCC_PCLK1
        });

        const STANDARD_SPEED: usize = 100000;
        const FAST_SPEED: usize = 400000;

        let slv = (clk_freq / STANDARD_SPEED - 8) / 2;
        let flv = (clk_freq / FAST_SPEED) / 2 - 1;

        self.lcr.write(slv | flv << 9);
        self.wcr.write(flv / 3);
    }

    /// Enable or disable the I2C peripheral.
    pub fn cmd(&self, enable: bool) {
        toggle_reg_bits!(self.cr, I2C_CR_TWSI_UNIT_EN_MASK, enable);
        toggle_reg_bits!(self.cr, I2C_CR_SCL_EN_MASK, enable);
    }

    /// Enable or disable the specified I2C interrupt.
    pub fn config_interrupt(&self, interrupt: I2cInterrupt, enable: bool) {
        if matches!(interrupt, I2cInterrupt::MasterStopDet) {
            toggle_reg_bits!(self.cr, I2C_CR_MASTER_STOP_DET_EN_MASK, enable);
        }
        toggle_reg_bits!(self.cr, 1 << interrupt as usize, enable);
    }

    /// Clear the I2C interrupt status.
    pub fn clear_interrupt(&self, interrupt: I2cInterrupt) {
        self.sr.write(1 << interrupt as usize);
    }

    /// Send the start request for I2C master.
    ///
    /// * `slave_addr` - The slave address.
    /// * `bit_rw` - The read/write bit (use [`I2cRW`]).
    pub fn master_send_start(&self, slave_addr: u8, bit_rw: u8) {
        let data = (slave_addr << 1) | bit_rw;

        toggle_reg_bits!(self.cr, I2C_CR_MASTER_ABORT_MASK, false);
        if is_fifo_mode(self.cr.read()) {
            toggle_reg_bits!(self.cr, I2C_CR_TRANS_BEGIN_MASK, true);

            self.wfifo
                .write(data as usize | I2C_WFIFO_CONTROL_START_MASK | I2C_WFIFO_CONTROL_TB_MASK);
        } else {
            self.dbr.write(data as usize);
            toggle_reg_bits!(self.cr, I2C_CR_STOP_MASK, false);
            toggle_reg_bits!(self.cr, I2C_CR_START_MASK | I2C_CR_TRANS_BYTE_MASK, true);
        }
    }

    /// Send the stop request for I2C master.
    pub fn master_send_stop(&self) {
        toggle_reg_bits!(self.cr, I2C_CR_START_MASK, false);
        toggle_reg_bits!(self.cr, I2C_CR_MASTER_ABORT_MASK, true);
    }

    /// Send the stop request with data for I2C master.
    pub fn master_send_stop_with_data(&self, data: u8) {
        if is_fifo_mode(self.cr.read()) {
            self.wfifo
                .write(data as usize | I2C_WFIFO_CONTROL_TB_MASK | I2C_WFIFO_CONTROL_STOP_MASK);
        } else {
            self.dbr.write(data as usize);

            toggle_reg_bits!(self.cr, I2C_CR_START_MASK, false);
            toggle_reg_bits!(self.cr, I2C_CR_TRANS_BYTE_MASK, true);
            toggle_reg_bits!(self.cr, I2C_CR_STOP_MASK, true);
        }
    }

    /// Write a byte to send.
    pub fn send_data(&self, data: u8) {
        if is_fifo_mode(self.cr.read()) {
            self.wfifo.write(data as usize | I2C_WFIFO_CONTROL_TB_MASK);
        } else {
            self.dbr.write(data as usize);

            toggle_reg_bits!(self.cr, I2C_CR_START_MASK, false);
            toggle_reg_bits!(self.cr, I2C_CR_TRANS_BYTE_MASK, true);
        }
    }

    /// Read a byte.
    ///
    /// Call [`receive_mode`](I2c::receive_mode) first to initiate the receive transaction.
    pub fn receive_data(&self) -> u8 {
        if is_fifo_mode(self.cr.read()) {
            self.rfifo.read() as u8
        } else {
            self.dbr.read() as u8
        }
    }

    /// Configure the receive mode and initiate the receive transaction.
    pub fn receive_mode(&self, ack: I2cAck) {
        if is_fifo_mode(self.cr.read()) {
            self.wfifo.write(
                I2C_WFIFO_CONTROL_TB_MASK
                    | (if matches!(ack, I2cAck::Nak) {
                        I2C_WFIFO_CONTROL_ACKNAK_MASK
                    } else {
                        0
                    }),
            );
        } else {
            toggle_reg_bits!(self.cr, I2C_CR_ACKNAK_MASK, matches!(ack, I2cAck::Nak));
            toggle_reg_bits!(self.cr, I2C_CR_START_MASK, false);
            toggle_reg_bits!(self.cr, I2C_CR_TRANS_BYTE_MASK, true);
        }
    }

    /// Clear the flag status of the specified I2C flag.
    pub fn clear_flag_status(&self, flag: I2cFlag) {
        self.sr.write(1 << flag as usize);
    }

    /// Get the flag status of the specified I2C flag.
    pub fn get_flag_status(&self, flag: I2cFlag) -> bool {
        if matches!(flag, I2cFlag::RFifoEmpty) {
            I2C0.rfifo_status.read() & I2C_RFIFO_STATUS_SIZE_MASK == 0
        } else {
            self.sr.read() & (1 << flag as usize) != 0
        }
    }

    /// Get the interrupt status of the specified I2C interrupt.
    pub fn get_interrupt_status(&self, interrupt: I2cInterrupt) -> bool {
        self.sr.read() & (1 << interrupt as usize) != 0
    }
}
