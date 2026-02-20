#![allow(clippy::empty_loop)]

unsafe extern "C" {
    fn RadioOnDioIrq();
    fn RtcOnIrq();
}

/// This function handles the NMI exception
#[unsafe(no_mangle)]
pub extern "C" fn NMI_Handler() {}

/// This function handles Hard Fault exception.
#[unsafe(no_mangle)]
pub extern "C" fn HardFault_Handler() -> ! {
    /* Go to infinite loop when Hard Fault exception occurs */
    loop {}
}

/// This function handles Memory Manage exception.
#[unsafe(no_mangle)]
pub extern "C" fn MemManage_Handler() -> ! {
    /* Go to infinite loop when Memory Manage exception occurs */
    loop {}
}

/// This function handles Bus Fault exception.
#[unsafe(no_mangle)]
pub extern "C" fn BusFault_Handler() -> ! {
    /* Go to infinite loop when Bus Fault exception occurs */
    loop {}
}

/// This function handles Usage Fault exception.
#[unsafe(no_mangle)]
pub extern "C" fn UsageFault_Handler() -> ! {
    /* Go to infinite loop when Usage Fault exception occurs */
    loop {}
}

/// This function handles SVCall exception.
#[unsafe(no_mangle)]
pub extern "C" fn SVC_Handler() {}

/// This function handles Debug Monitor exception.
#[unsafe(no_mangle)]
pub extern "C" fn PendSV_Handler() {}

/// This function handles SysTick Handler.
#[unsafe(no_mangle)]
pub extern "C" fn SysTick_Handler() {}

/// This function handles PWR Handler.
#[unsafe(no_mangle)]
pub extern "C" fn PWR_IRQHandler() {}

/******************************************************************************/
/*                 Tremo Peripherals Interrupt Handlers                   */
/*  Add here the Interrupt Handler for the used peripheral(s) (PPP), for the  */
/*  available peripheral interrupt handler's name please refer to the startup */
/*  file (startup_cm4.S).                                               */
/******************************************************************************/

/// This function handles LORA Interrupts.
#[unsafe(no_mangle)]
pub extern "C" fn LORA_IRQHandler() {
    unsafe { RadioOnDioIrq() };
}

/// This function handles RTC Interrupts.
#[unsafe(no_mangle)]
pub extern "C" fn RTC_IRQHandler() {
    unsafe { RtcOnIrq() };
}
