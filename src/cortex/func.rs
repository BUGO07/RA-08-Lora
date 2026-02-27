/// Enable IRQ Interrupts
///
/// This function enables IRQ interrupts by clearing the I-bit in the CPSR.
/// Can only be executed in Privileged modes.
#[inline(always)]
pub fn _enable_irq() {
    unsafe { core::arch::asm!("cpsie i", options(nomem, nostack)) };
}

/// Disable IRQ Interrupts
///
/// This function disables IRQ interrupts by setting the I-bit in the CPSR.
/// Can only be executed in Privileged modes.
#[inline(always)]
pub fn _disable_irq() {
    unsafe { core::arch::asm!("cpsid i", options(nomem, nostack)) };
}

/// Get Control Register
///
/// This function returns the content of the Control Register.
///
/// Returns the Control Register value.
#[inline(always)]
pub fn _get_control() -> u32 {
    let result: u32;
    unsafe { core::arch::asm!("MRS {}, control", out(reg) result, options(nomem, nostack)) };
    result
}

/// Set Control Register
///
/// This function writes the given value to the Control Register.
///
/// # Parameters
/// - `control`: Control Register value to set
#[inline(always)]
pub fn _set_control(control: u32) {
    unsafe { core::arch::asm!("MSR control, {}", in(reg) control, options(nostack)) };
}

/// Get IPSR Register
///
/// This function returns the content of the IPSR Register.
///
/// Returns the IPSR Register value.
#[inline(always)]
pub fn _get_ipsr() -> u32 {
    let result: u32;
    unsafe { core::arch::asm!("MRS {}, ipsr", out(reg) result, options(nomem, nostack)) };
    result
}

/// Get APSR Register
///
/// This function returns the content of the APSR Register.
///
/// Returns the APSR Register value.
#[inline(always)]
pub fn _get_apsr() -> u32 {
    let result: u32;
    unsafe { core::arch::asm!("MRS {}, apsr", out(reg) result, options(nomem, nostack)) };
    result
}

/// Get xPSR Register
///
/// This function returns the content of the xPSR Register.
///
/// Returns the xPSR Register value.
#[inline(always)]
pub fn _get_xpsr() -> u32 {
    let result: u32;
    unsafe { core::arch::asm!("MRS {}, xpsr", out(reg) result, options(nomem, nostack)) };
    result
}

/// Get Process Stack Pointer
///
/// This function returns the current value of the Process Stack Pointer (PSP).
///
/// Returns the PSP Register value.
#[inline(always)]
pub fn _get_psp() -> u32 {
    let result: u32;
    unsafe { core::arch::asm!("MRS {}, psp", out(reg) result, options(nomem, nostack)) };
    result
}

/// Set Process Stack Pointer
///
/// This function assigns the given value to the Process Stack Pointer (PSP).
///
/// # Parameters
/// - `top_of_proc_stack`: Process Stack Pointer value to set
#[inline(always)]
pub fn _set_psp(top_of_proc_stack: u32) {
    unsafe { core::arch::asm!("MSR psp, {}", in(reg) top_of_proc_stack, options(nomem, nostack)) };
}

/// Get Main Stack Pointer
///
/// This function returns the current value of the Main Stack Pointer (MSP).
///
/// Returns the MSP Register value.
#[inline(always)]
pub fn _get_msp() -> u32 {
    let result: u32;
    unsafe { core::arch::asm!("MRS {}, msp", out(reg) result, options(nomem, nostack)) };
    result
}

/// Set Main Stack Pointer
///
/// This function assigns the given value to the Main Stack Pointer (MSP).
///
/// # Parameters
/// - `top_of_main_stack`: Main Stack Pointer value to set
#[inline(always)]
pub fn _set_msp(top_of_main_stack: u32) {
    unsafe { core::arch::asm!("MSR msp, {}", in(reg) top_of_main_stack, options(nomem, nostack)) };
}

/// Get Priority Mask
///
/// This function returns the current state of the priority mask bit from the Priority Mask Register.
///
/// Returns the Priority Mask value.
#[inline(always)]
pub fn _get_primask() -> u32 {
    let result: u32;
    unsafe { core::arch::asm!("MRS {}, primask", out(reg) result, options(nomem, nostack)) };
    result
}

/// Set Priority Mask
///
/// This function assigns the given value to the Priority Mask Register.
///
/// # Parameters
/// - `pri_mask`: Priority Mask
#[inline(always)]
pub fn _set_primask(pri_mask: u32) {
    unsafe { core::arch::asm!("MSR primask, {}", in(reg) pri_mask, options(nostack)) };
}
