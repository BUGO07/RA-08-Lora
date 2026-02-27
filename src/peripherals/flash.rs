use crate::{
    cortex::func::{_disable_irq, _enable_irq},
    peripherals::regs::{
        EFC, EFC_CR_ECC_DISABLE_MASK, EFC_CR_MASS_ERASE_EN_MASK, EFC_CR_PAGE_ERASE_EN_MASK,
        EFC_CR_PREFETCH_EN_MASK, EFC_CR_PROG_EN_MASK, EFC_CR_PROG_MODE_WLINE,
        EFC_CR_WRITE_RELEASE_EN_MASK, EFC_SR_OPERATION_DONE, EFC_SR_PROGRAM_DATA_WAIT, FLASH_BASE,
        SEC, SEC_SR_FLASH_ACCESS_ERROR_MASK,
    },
};

/// The size of the flash word line
pub const FLASH_LINE_SIZE: u32 = 0x200;
/// The size of a flash page
pub const FLASH_PAGE_SIZE: u32 = 0x1000;

/// The flash protect sequence0 to unlock the flash CR access
pub const FLASH_CR_PROTECT_SEQ0: u32 = 0x8C9DAEBF;
/// The flash protect sequence1 to unlock the flash CR access
pub const FLASH_CR_PROTECT_SEQ1: u32 = 0x13141516;

/// The address of the flash OTP area
pub const FLASH_OTP_ADDR_START: u32 = 0x10001C00;
/// The end address of the flash OTP area
pub const FLASH_OTP_ADDR_END: u32 = 0x10002000;
/// The size of the flash OTP area
pub const FLASH_OTP_SIZE: u32 = FLASH_OTP_ADDR_END - FLASH_OTP_ADDR_START;

/// Macro for performing a volatile read from a memory address
macro_rules! volatile_read {
    ($addr:expr, $type:ty) => {
        unsafe { core::ptr::read_volatile($addr as *const $type) }
    };
}

/// Macro for performing a volatile write to a memory address
macro_rules! volatile_write {
    ($addr:expr, $value:expr, $type:ty) => {
        unsafe { core::ptr::write_volatile($addr as *mut $type, $value) }
    };
}

/// Errors that can occur during flash operations
pub enum FlashError {
    InvalidAddress,
    InvalidSize,
    OtpReflash,
    SecError,
}

/// Lock the flash control register to prevent accidental modifications
#[inline]
pub fn flash_cr_lock() {
    EFC.protect_seq.write(FLASH_CR_PROTECT_SEQ0);
    EFC.protect_seq.write(0);
}

/// Unlock the flash control register to allow modifications
#[inline]
pub fn flash_cr_unlock() {
    EFC.protect_seq.write(FLASH_CR_PROTECT_SEQ0);
    EFC.protect_seq.write(FLASH_CR_PROTECT_SEQ1);
}

/// Erase all the flash main area
///
/// Returns Ok if everything went well, or a FlashError if an error occurred (e.g. invalid address, size, or flash access error)
pub fn flash_erase_all() -> Result<(), FlashError> {
    // clear sr
    if SEC.sr.read() & SEC_SR_FLASH_ACCESS_ERROR_MASK != 0 {
        SEC.sr.write(SEC_SR_FLASH_ACCESS_ERROR_MASK);
    }

    flash_cr_unlock();
    EFC.cr.write(
        (EFC.cr.read() & EFC_CR_ECC_DISABLE_MASK)
            | EFC_CR_MASS_ERASE_EN_MASK
            | EFC_CR_PREFETCH_EN_MASK,
    );
    flash_cr_lock();

    volatile_write!(FLASH_BASE, 0xFFFFFFFF, u32);

    if SEC.sr.read() & SEC_SR_FLASH_ACCESS_ERROR_MASK != 0 {
        SEC.sr.write(SEC_SR_FLASH_ACCESS_ERROR_MASK);
        return Err(FlashError::SecError);
    }

    while EFC.sr.read() & EFC_SR_OPERATION_DONE == 0 {}

    EFC.sr.write(EFC_SR_OPERATION_DONE);

    Ok(())
}

/// Erase one page
///
/// Returns Ok if everything went well, or a FlashError if an error occurred (e.g. invalid address, size, or flash access error)
pub fn flash_erase_page(addr: u32) -> Result<(), FlashError> {
    // clear sr
    if SEC.sr.read() & SEC_SR_FLASH_ACCESS_ERROR_MASK != 0 {
        SEC.sr.write(SEC_SR_FLASH_ACCESS_ERROR_MASK);
    }

    flash_cr_unlock();
    EFC.cr.write(
        (EFC.cr.read() & EFC_CR_ECC_DISABLE_MASK)
            | EFC_CR_PAGE_ERASE_EN_MASK
            | EFC_CR_PREFETCH_EN_MASK,
    );
    flash_cr_lock();

    volatile_write!(addr, 0xFFFFFFFF, u32);

    if SEC.sr.read() & SEC_SR_FLASH_ACCESS_ERROR_MASK != 0 {
        SEC.sr.write(SEC_SR_FLASH_ACCESS_ERROR_MASK);
        return Err(FlashError::SecError);
    }

    while EFC.sr.read() & EFC_SR_OPERATION_DONE == 0 {}

    EFC.sr.write(EFC_SR_OPERATION_DONE);

    Ok(())
}

pub fn flash_program_bytes(addr: u32, data: &[u8], size: u32) -> Result<(), FlashError> {
    let mut tmp = [0u8; 8];
    let p = tmp.as_mut_ptr();

    // clear sr
    if SEC.sr.read() & SEC_SR_FLASH_ACCESS_ERROR_MASK != 0 {
        SEC.sr.write(SEC_SR_FLASH_ACCESS_ERROR_MASK);
    }

    flash_cr_unlock();
    EFC.cr.write(
        (EFC.cr.read() & EFC_CR_ECC_DISABLE_MASK) | EFC_CR_PROG_EN_MASK | EFC_CR_PREFETCH_EN_MASK,
    );
    flash_cr_lock();

    let aligned_size = size & 0xFFFFFFF8;
    tmp.fill(0xFF);
    for i in aligned_size..size {
        tmp[(i - aligned_size) as usize] = data[i as usize];
    }

    for i in (0..size).step_by(8) {
        if i < aligned_size {
            EFC.program_data0
                .write(volatile_read!(data.as_ptr().add(i as usize), u32));
            EFC.program_data1
                .write(volatile_read!(data.as_ptr().add(i as usize + 4), u32));
        } else {
            EFC.program_data0.write(volatile_read!(p, u32));
            EFC.program_data1.write(volatile_read!(p.add(4), u32));
        }

        volatile_write!(addr + i, 0xFFFFFFFF, u32);

        if SEC.sr.read() & SEC_SR_FLASH_ACCESS_ERROR_MASK != 0 {
            SEC.sr.write(SEC_SR_FLASH_ACCESS_ERROR_MASK);
            return Err(FlashError::SecError);
        }

        while EFC.sr.read() & EFC_SR_OPERATION_DONE == 0 {}

        EFC.sr.write(EFC_SR_OPERATION_DONE);
    }

    Ok(())
}

pub fn flash_program_line(addr: u32, data: &[u8]) -> Result<(), FlashError> {
    _disable_irq();

    // clear sr
    if SEC.sr.read() & SEC_SR_FLASH_ACCESS_ERROR_MASK != 0 {
        SEC.sr.write(SEC_SR_FLASH_ACCESS_ERROR_MASK);
    }

    flash_cr_unlock();
    EFC.cr.write(
        (EFC.cr.read() & EFC_CR_ECC_DISABLE_MASK)
            | EFC_CR_WRITE_RELEASE_EN_MASK
            | EFC_CR_PROG_MODE_WLINE
            | EFC_CR_PROG_EN_MASK
            | EFC_CR_PREFETCH_EN_MASK,
    );
    flash_cr_lock();

    while EFC.sr.read() & EFC_SR_PROGRAM_DATA_WAIT == 0 {}

    EFC.program_data0.write(volatile_read!(data.as_ptr(), u32));
    EFC.program_data1
        .write(volatile_read!(data.as_ptr().add(4), u32));
    volatile_write!(addr, 0xFFFFFFFF, u32);

    if SEC.sr.read() & SEC_SR_FLASH_ACCESS_ERROR_MASK != 0 {
        SEC.sr.write(SEC_SR_FLASH_ACCESS_ERROR_MASK);
        _enable_irq();

        return Err(FlashError::SecError);
    }

    for j in (8..512).step_by(8) {
        while EFC.sr.read() & EFC_SR_PROGRAM_DATA_WAIT == 0 {}

        EFC.program_data0
            .write(volatile_read!(data.as_ptr().add(j as usize), u32));
        EFC.program_data1
            .write(volatile_read!(data.as_ptr().add(j as usize + 4), u32));
    }

    while EFC.sr.read() & EFC_SR_OPERATION_DONE == 0 {}
    EFC.sr.write(EFC_SR_OPERATION_DONE);

    _enable_irq();
    Ok(())
}

pub fn flash_otp_program_data(addr: u32, data: &[u8], size: u32) -> Result<(), FlashError> {
    if addr < FLASH_OTP_ADDR_START || (addr + size) > FLASH_OTP_ADDR_END {
        return Err(FlashError::InvalidAddress);
    }

    if size == 0 || size > FLASH_OTP_SIZE {
        return Err(FlashError::InvalidSize);
    }

    let aligned_size = (size + 7) & 0xFFFFFFF8;

    for i in 0..aligned_size {
        if volatile_read!(addr + i, u8) != 0xFF {
            return Err(FlashError::OtpReflash);
        }
    }

    flash_program_bytes(addr, data, size)
}
