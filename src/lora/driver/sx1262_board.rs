use core::sync::atomic::{AtomicU8, Ordering};

use crate::{
    cortex::{
        IRQType,
        func::{_disable_irq, _enable_irq},
        nvic_enable_irq,
    },
    lora::radio::sx126x::{
        RadioCommands, RadioRampTimes, sx126x_check_device_ready, sx126x_set_tx_params,
    },
    lora_config::{
        CONFIG_LORA_RFSW_CTRL_PIN, CONFIG_LORA_RFSW_VDD_GPIOX, CONFIG_LORA_RFSW_VDD_PIN,
    },
    peripherals::{
        delay::delay_us,
        gpio::{GpioMode, GpioPin},
        regs::{GPIOD, LORAC},
    },
    toggle_reg_bits,
};

pub const BOARD_TCXO_WAKEUP_TIME: usize = 5;
pub static G_PA_OPT_SETTING: AtomicU8 = AtomicU8::new(0);

pub fn spi_in_out(data: u16) -> u16 {
    LORAC.ssp_dr.write(data as usize);

    loop {
        let status = LORAC.ssp_sr.read();
        if (status & 0x01 == 0x01) && (status & 0x10 == 0) {
            break;
        }
    }

    (LORAC.ssp_dr.read() & 0xFF) as u16
}

pub fn sx126x_lorac_init() {
    LORAC.cr0.write(0x00000200);

    LORAC.ssp_cr0.write(0x07);
    LORAC.ssp_cpsr.write(0x02);

    if LORAC.cr1.read() != 0x80 {
        delay_us(20);
        LORAC.nss_cr.write(0);
        delay_us(20);
        LORAC.nss_cr.write(1);
    }

    LORAC.ssp_cr1.write(0x02);

    nvic_enable_irq(IRQType::Lora);
    //nvic_set_priority(IRQType::Lora, 2);

    if matches!(CONFIG_LORA_RFSW_CTRL_PIN, GpioPin::Pin10) {
        GPIOD.set_iomux(CONFIG_LORA_RFSW_CTRL_PIN, 6);
    } else {
        GPIOD.set_iomux(CONFIG_LORA_RFSW_CTRL_PIN, 3);
    }
}

pub fn sx126x_get_board_tcxo_wakeup_time() -> usize {
    BOARD_TCXO_WAKEUP_TIME
}

pub fn sx126x_reset() {
    toggle_reg_bits!(LORAC.cr1, 1 << 5, false); // nreset
    delay_us(100);
    toggle_reg_bits!(LORAC.cr1, 1 << 5, true); // nreset release
    toggle_reg_bits!(LORAC.cr1, 1 << 7, false); // por release
    toggle_reg_bits!(LORAC.cr0, 1 << 5, true); // irq0
    toggle_reg_bits!(LORAC.cr1, 0x1, false); // tcxo

    while LORAC.sr.read() & 0x100 != 0 {}
}

pub fn sx126x_wait_on_busy() {
    delay_us(10);
    while LORAC.sr.read() & 0x100 != 0 {}
}

pub fn sx126x_wakeup() {
    _disable_irq();

    LORAC.nss_cr.write(0);
    delay_us(20);

    spi_in_out(RadioCommands::GetStatus as u16);
    spi_in_out(0x00);

    LORAC.nss_cr.write(1);

    sx126x_wait_on_busy();

    _enable_irq();
}

pub fn sx126x_write_command(command: RadioCommands, data: &[u8]) {
    sx126x_check_device_ready();

    LORAC.nss_cr.write(0);

    spi_in_out(command as u16);

    for &b in data {
        spi_in_out(b as u16);
    }

    LORAC.nss_cr.write(1);

    if !matches!(command, RadioCommands::SetSleep) {
        sx126x_wait_on_busy();
    }
}

pub fn sx126x_read_command(command: RadioCommands, data: &mut [u8]) {
    sx126x_check_device_ready();

    LORAC.nss_cr.write(0);

    spi_in_out(command as u16);
    spi_in_out(0x00);

    for b in data.iter_mut() {
        *b = spi_in_out(0) as u8;
    }

    LORAC.nss_cr.write(1);

    sx126x_wait_on_busy();
}

pub fn sx126x_write_registers(addr: u16, data: &[u8]) {
    sx126x_check_device_ready();

    LORAC.nss_cr.write(0);

    spi_in_out(RadioCommands::WriteRegister as u16);
    spi_in_out((addr & 0xFF00) >> 8);
    spi_in_out(addr & 0x00FF);

    for &b in data {
        spi_in_out(b as u16);
    }

    LORAC.nss_cr.write(1);

    sx126x_wait_on_busy();
}

pub fn sx126x_read_registers(addr: u16, data: &mut [u8]) {
    sx126x_check_device_ready();

    LORAC.nss_cr.write(0);

    spi_in_out(RadioCommands::ReadRegister as u16);
    spi_in_out((addr & 0xFF00) >> 8);
    spi_in_out(addr & 0x00FF);
    spi_in_out(0);

    for b in data.iter_mut() {
        *b = spi_in_out(0) as u8;
    }

    LORAC.nss_cr.write(1);

    sx126x_wait_on_busy();
}

pub fn sx126x_read_register(addr: u16) -> u8 {
    let mut data = [0u8];
    sx126x_read_registers(addr, &mut data);
    data[0]
}

pub fn sx126x_write_buffer(offset: u8, data: &[u8]) {
    sx126x_check_device_ready();

    LORAC.nss_cr.write(0);

    spi_in_out(RadioCommands::WriteBuffer as u16);
    spi_in_out(offset as u16);

    for &b in data {
        spi_in_out(b as u16);
    }

    LORAC.nss_cr.write(1);

    sx126x_wait_on_busy();
}

pub fn sx126x_read_buffer(offset: u8, data: &mut [u8]) {
    sx126x_check_device_ready();

    LORAC.nss_cr.write(0);

    spi_in_out(RadioCommands::ReadBuffer as u16);
    spi_in_out(offset as u16);
    spi_in_out(0);

    for b in data.iter_mut() {
        *b = spi_in_out(0) as u8;
    }

    LORAC.nss_cr.write(1);

    sx126x_wait_on_busy();
}

pub fn sx126x_set_rf_tx_power(power: i8) {
    sx126x_set_tx_params(power, RadioRampTimes::Ramp40Us);
}

pub fn sx126x_get_pa_select(_channel: usize) -> u8 {
    2 // SX1262 ??
}

pub fn sx126x_ant_sw_on() {
    CONFIG_LORA_RFSW_VDD_GPIOX.init(CONFIG_LORA_RFSW_VDD_PIN, GpioMode::OutputPPHigh);
}

pub fn sx126x_ant_sw_off() {
    CONFIG_LORA_RFSW_VDD_GPIOX.init(CONFIG_LORA_RFSW_VDD_PIN, GpioMode::OutputPPLow);
}

pub fn sx126x_check_rf_freq(_freq: usize) -> bool {
    // implement check. currently all frequencies are supported
    true
}

pub fn sx126x_get_pa_opt() -> u8 {
    G_PA_OPT_SETTING.load(Ordering::Relaxed)
}

pub fn sx126x_set_pa_opt(opt: u8) {
    if opt > 3 {
        return;
    }

    G_PA_OPT_SETTING.store(opt, Ordering::Relaxed);
}

// ── extern "C" wrappers matching sx126x-board.h ──────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn SX126xLoracInit() {
    sx126x_lorac_init();
}

/// No-op on this platform; I/O is initialised in `SX126xLoracInit`.
#[unsafe(no_mangle)]
pub extern "C" fn SX126xIoInit() {}

/// `DioIrqHandler` in C is `void (*)(void)`.
/// On this platform the LoRa IRQ is enabled in `SX126xLoracInit`; this
/// wrapper is present for link-compatibility.  The handler pointer is unused
/// because the Rust interrupt handler (`radio_on_dio_irq`) is installed
/// statically.
#[unsafe(no_mangle)]
pub extern "C" fn SX126xIoIrqInit(_dio_irq: Option<extern "C" fn()>) {}

/// No-op on this platform.
#[unsafe(no_mangle)]
pub extern "C" fn SX126xIoDeInit() {}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xReset() {
    sx126x_reset();
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xWaitOnBusy() {
    sx126x_wait_on_busy();
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xWakeup() {
    sx126x_wakeup();
}

/// # Safety
/// `buffer` must be a valid pointer to at least `size` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SX126xWriteCommand(opcode: u8, buffer: *const u8, size: u16) {
    let cmd: RadioCommands = unsafe { core::mem::transmute(opcode) };
    let data = unsafe { core::slice::from_raw_parts(buffer, size as usize) };
    sx126x_write_command(cmd, data);
}

/// # Safety
/// `buffer` must be a valid pointer to at least `size` bytes of writable memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SX126xReadCommand(opcode: u8, buffer: *mut u8, size: u16) {
    let cmd: RadioCommands = unsafe { core::mem::transmute(opcode) };
    let data = unsafe { core::slice::from_raw_parts_mut(buffer, size as usize) };
    sx126x_read_command(cmd, data);
}

/// Single-byte register write matching `SX126xWriteRegister` in C.
#[unsafe(no_mangle)]
pub extern "C" fn SX126xWriteRegister(address: u16, value: u8) {
    sx126x_write_registers(address, &[value]);
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xReadRegister(address: u16) -> u8 {
    sx126x_read_register(address)
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xSetRfTxPower(power: i8) {
    sx126x_set_rf_tx_power(power);
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xGetPaSelect(channel: usize) -> u8 {
    sx126x_get_pa_select(channel)
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xAntSwOn() {
    sx126x_ant_sw_on();
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xAntSwOff() {
    sx126x_ant_sw_off();
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xCheckRfFrequency(frequency: usize) -> bool {
    sx126x_check_rf_freq(frequency)
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xGetBoardTcxoWakeupTime() -> usize {
    sx126x_get_board_tcxo_wakeup_time()
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xGetPaOpt() -> u8 {
    sx126x_get_pa_opt()
}

#[unsafe(no_mangle)]
pub extern "C" fn SX126xSetPaOpt(opt: u8) {
    sx126x_set_pa_opt(opt);
}

#[unsafe(no_mangle)]
pub extern "C" fn BoardDisableIrq() {
    _disable_irq();
}

#[unsafe(no_mangle)]
pub extern "C" fn BoardEnableIrq() {
    _enable_irq();
}
