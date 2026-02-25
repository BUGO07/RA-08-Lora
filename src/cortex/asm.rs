/// No Operation
///
/// No Operation does nothing. This instruction can be used for code alignment purposes.
#[inline]
pub fn _nop() {
    unsafe { core::arch::asm!("nop") };
}

/// Wait For Interrupt
///
/// Wait For Interrupt is a hint instruction that suspends execution
/// until one of a number of events occurs.
#[inline]
pub fn _wfi() {
    unsafe { core::arch::asm!("wfi") };
}

/// Wait For Event
///
/// Wait For Event is a hint instruction that permits the processor to enter
/// a low-power state until one of a number of events occurs.
#[inline]
pub fn _wfe() {
    unsafe { core::arch::asm!("wfe") };
}

/// Send Event
///
/// Send Event is a hint instruction. It causes an event to be signaled to the CPU.
#[inline]
pub fn _sev() {
    unsafe { core::arch::asm!("sev") };
}

/// Instruction Synchronization Barrier
///
/// Instruction Synchronization Barrier flushes the pipeline in the processor,
/// so that all instructions following the ISB are fetched from cache or
/// memory, after the instruction has been completed.
#[inline]
pub fn _isb() {
    unsafe { core::arch::asm!("isb") };
}

/// Data Synchronization Barrier
///
/// This function acts as a special kind of Data Memory Barrier.
/// It completes when all explicit memory accesses before this instruction complete.
#[inline]
pub fn _dsb() {
    unsafe { core::arch::asm!("dsb") };
}

/// Data Memory Barrier
///
/// This function ensures the apparent order of the explicit memory operations before
/// and after the instruction, without ensuring their completion.
#[inline]
pub fn _dmb() {
    unsafe { core::arch::asm!("dmb") };
}

/// Reverse byte order (32 bit)
///
/// # Arguments
/// * `value` - Value to reverse
///
/// # Returns
/// Reversed value
#[inline]
pub fn _rev(value: u32) -> u32 {
    value.swap_bytes()
}

/// Rotate Right in unsigned value (32 bit)
///
/// # Arguments
/// * `op1` - Value to rotate
/// * `op2` - Number of bits to rotate
///
/// # Returns
/// Rotated value
#[inline]
pub fn _ror(op1: u32, op2: u32) -> u32 {
    op1.rotate_right(op2)
}

/// Breakpoint
///
/// This function causes the processor to enter Debug state.
/// Debug tools can use this to investigate system state when the instruction at a particular address is reached.
#[macro_export]
macro_rules! bkpt {
    ($value:expr) => {
        unsafe { core::arch::asm!("bkpt #{}", const $value) }
    };
}
