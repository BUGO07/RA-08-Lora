use crate::{
    cortex::{VolatileRO, VolatileRW},
    define_reg,
    peripherals::{
        rcc::{
            RCC_PCLK0, RCC_PCLK1, RCC_PERIPHERAL_SSP0, RCC_PERIPHERAL_SSP1, RCC_PERIPHERAL_SSP2,
        },
        regs::{RCC, SSP0_BASE, SSP1_BASE, SSP2_BASE},
    },
    toggle_reg_bits,
};

pub const SSP_ROLE_MASTER: u32 = 0x0;
pub const SSP_ROLE_SLAVE: u32 = 0x4;

pub const SSP_FRAME_FORMAT_SPI: u32 = 0x0;
pub const SSP_FRAME_FORMAT_TI: u32 = 0x1 << 4;
pub const SSP_FRAME_FORMAT_MW: u32 = 0x2 << 4;

pub const SSP_SLAVE_OUTPUT_DISABLE: u32 = 1 << 3;
/// Clock polarity position
pub const SPI_CLK_POLARITY_POS: u32 = 0x6;
/// Clock polarity low
pub const SPI_CLK_POLARITY_LOW: u32 = 0x0;
/// Clock polarity high
pub const SPI_CLK_POLARITY_HIGH: u32 = 0x1 << SPI_CLK_POLARITY_POS;

/// Clock phase position
pub const SPI_CLK_PHASE_POS: u32 = 0x7;
/// Clock phase 1edge
pub const SPI_CLK_PHASE_1EDGE: u32 = 0x0;
/// Clock phase 2edge
pub const SPI_CLK_PHASE_2EDGE: u32 = 0x1 << SPI_CLK_PHASE_POS;

/// Data size 4bit
pub const SSP_DATA_SIZE_4BIT: u32 = 0x3;
/// Data size 8bit
pub const SSP_DATA_SIZE_8BIT: u32 = 0x7;
/// Data size 16bit
pub const SSP_DATA_SIZE_16BIT: u32 = 0xF;

/// TX fifo empty flag
pub const SSP_FLAG_TX_FIFO_EMPTY: u32 = 0x1;
/// TX fifo not full flag
pub const SSP_FLAG_TX_FIFO_NOT_FULL: u32 = 1 << 1;
/// RX fifo not empty flag
pub const SSP_FLAG_RX_FIFO_NOT_EMPTY: u32 = 1 << 2;
/// RX fifo full flag
pub const SSP_FLAG_RX_FIFO_FULL: u32 = 1 << 3;
/// Busy flag
pub const SSP_FLAG_BUSY: u32 = 1 << 4;

/// RX fifo overrun interrupt
pub const SSP_INTERRUPT_RX_FIFO_OVERRUN: u32 = 1 << 0;
/// RX timeout interrupt
pub const SSP_INTERRUPT_RX_TIMEOUT: u32 = 1 << 1;
/// RX fifo trigger interrupt
pub const SSP_INTERRUPT_RX_FIFO_TRIGGER: u32 = 1 << 2;
/// TX fifo trigger interrupt
pub const SSP_INTERRUPT_TX_FIFO_TRIGGER: u32 = 1 << 3;
/// RX fifo overrun and RX timeout interrupt
pub const SSP_INTERRUPT_RX_OVERRUN_AND_TIMEOUT: u32 = 0x3;
/// All interrupt
pub const SSP_INTERRUPT_ALL: u32 = 0xf;

/// TX DMA enable
pub const SSP_DMA_TX_EN: u32 = 1 << 1;
/// RX DMA enable
pub const SSP_DMA_RX_EN: u32 = 1 << 2;

pub struct SspConfig {
    pub sclk: u32,
    pub role: u32,
    pub format: u32,
    pub clk_pol: u32,
    pub clk_phase: u32,
    pub data_size: u32,
    pub dma_tx_en: bool,
    pub dma_rx_en: bool,
}

impl Default for SspConfig {
    fn default() -> Self {
        Self {
            sclk: 1_000_000,
            role: SSP_ROLE_MASTER,
            format: SSP_FRAME_FORMAT_SPI,
            clk_pol: SPI_CLK_POLARITY_HIGH,
            clk_phase: SPI_CLK_PHASE_2EDGE,
            data_size: SSP_DATA_SIZE_8BIT,
            dma_tx_en: false,
            dma_rx_en: false,
        }
    }
}

define_reg! {
    Ssp
    __Ssp {
        cr0: VolatileRW<u32>,
        cr1: VolatileRW<u32>,
        dr: VolatileRW<u32>,
        sr: VolatileRO<u32>,
        cpsr: VolatileRW<u32>,
        imsc: VolatileRW<u32>,
        ris: VolatileRO<u32>,
        mis: VolatileRO<u32>,
        icr: VolatileRW<u32>,
        dma_cr: VolatileRW<u32>,
        resv: [VolatileRO<u32>; 1006],
        periph_id0: VolatileRO<u32>,
        periph_id1: VolatileRO<u32>,
        periph_id2: VolatileRO<u32>,
        periph_id3: VolatileRO<u32>,
        pcell_id0: VolatileRO<u32>,
        pcell_id1: VolatileRO<u32>,
        pcell_id2: VolatileRO<u32>,
        pcell_id3: VolatileRO<u32>,
    }
}

impl Ssp {
    #[inline]
    pub fn clear_interrupt(&self, interrupt: u32) {
        toggle_reg_bits!(self.icr, interrupt, true);
    }

    #[inline]
    pub fn get_flag_status(&self, flag: u32) -> bool {
        self.sr.read() & flag != 0
    }

    #[inline]
    pub fn get_interrupt_status(&self, flag: u32) -> bool {
        self.mis.read() & flag != 0
    }

    #[inline]
    pub fn get_raw_interrupt_status(&self, flag: u32) -> u32 {
        self.ris.read() & flag
    }

    pub fn init(&self, config: SspConfig) {
        self.cmd(false);
        self.config_interrupt(SSP_INTERRUPT_ALL, false);
        self.clear_interrupt(SSP_INTERRUPT_ALL);

        RCC.get_clk_freq(RCC_PCLK0);
        let clk_freq = if self.ptr() as u32 == SSP0_BASE {
            RCC.get_clk_freq(RCC_PCLK0)
        } else {
            RCC.get_clk_freq(RCC_PCLK1)
        };

        toggle_reg_bits!(self.cr0, 0x3 << 4, false);
        toggle_reg_bits!(self.cr0, config.format, true);

        let scr = (clk_freq / 2 / config.sclk) - 1; // check if > 0 instead of != 0 
        toggle_reg_bits!(self.cpsr, 0xff, false);
        toggle_reg_bits!(self.cpsr, 0x2, true);
        toggle_reg_bits!(self.cr0, 0xff << 8, false);
        toggle_reg_bits!(self.cr0, scr << 8, true);

        toggle_reg_bits!(self.cr0, 0x3 << 6, false);
        toggle_reg_bits!(self.cr0, config.clk_pol | config.clk_phase, true);

        toggle_reg_bits!(self.cr0, 0xf, false);
        toggle_reg_bits!(self.cr0, config.data_size, true);

        toggle_reg_bits!(self.cr1, 0x1 << 2, false);
        if config.role != SSP_ROLE_MASTER {
            // ? maybe discard the if statement at all, since x | 0 == x
            toggle_reg_bits!(self.cr1, SSP_ROLE_SLAVE, true);
        }

        // dma handshake config,
        // should be enabled after dmac has been configured and ready
        toggle_reg_bits!(self.dma_cr, SSP_DMA_TX_EN, config.dma_tx_en);
        toggle_reg_bits!(self.dma_cr, SSP_DMA_RX_EN, config.dma_rx_en);
    }

    pub fn deinit(&self) {
        let periph = match self.ptr() as u32 {
            SSP0_BASE => RCC_PERIPHERAL_SSP0,
            SSP1_BASE => RCC_PERIPHERAL_SSP1,
            SSP2_BASE => RCC_PERIPHERAL_SSP2,
            _ => unreachable!(),
        };

        RCC.enable_peripheral_clk(periph, false);
        RCC.rst_peripheral(periph, true);
        RCC.rst_peripheral(periph, false);
    }

    pub fn config_interrupt(&self, interrupt: u32, enable: bool) {
        toggle_reg_bits!(self.imsc, interrupt, enable);
    }

    pub fn cmd(&self, enable: bool) {
        toggle_reg_bits!(self.cr1, 0x1 << 1, enable);
    }

    // TODO: use slices
    pub fn send_data(&self, mut tx_data: *const u8, mut len: u16) {
        let data_size = self.cr0.read() & 0xf;

        while len > 0 {
            len -= 1;
            while !self.get_interrupt_status(SSP_FLAG_RX_FIFO_NOT_EMPTY) {}
            if data_size > SSP_DATA_SIZE_8BIT {
                self.dr.write(unsafe { *(tx_data as *const u16) } as u32);
                tx_data = unsafe { tx_data.add(2) };
            } else {
                self.dr.write(unsafe { *tx_data as u32 });
                tx_data = unsafe { tx_data.add(1) };
            }
        }
    }

    // TODO: use slices
    pub fn receive_data(&self, mut rx_data: *mut u8, mut len: u16) {
        let data_size = self.cr0.read() & 0xf;

        while len > 0 {
            len -= 1;
            while !self.get_interrupt_status(SSP_FLAG_RX_FIFO_NOT_EMPTY) {}
            if data_size > SSP_DATA_SIZE_8BIT {
                unsafe { *(rx_data as *mut u16) = self.dr.read() as u16 };
                rx_data = unsafe { rx_data.add(2) };
            } else {
                unsafe { *rx_data = self.dr.read() as u8 };
                rx_data = unsafe { rx_data.add(1) };
            }
        }
    }
}
