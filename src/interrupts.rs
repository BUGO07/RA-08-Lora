#![allow(clippy::empty_loop)]

use crate::{lora::radio::radio_on_dio_irq, peripherals::regs::RTC};

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

#[unsafe(no_mangle)]
pub extern "C" fn SEC_IRQHandler() {
    loop {}
}

/// This function handles RTC Interrupts.
#[unsafe(no_mangle)]
pub extern "C" fn RTC_IRQHandler() {
    RTC.on_irq();
}

#[unsafe(no_mangle)]
pub extern "C" fn WDG_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn EFC_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn UART3_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn I2C2_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn UART0_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn UART1_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn UART2_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn LPUART_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn SSP0_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn SSP1_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn QSPI_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn I2C0_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn I2C1_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn SCC_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn ADC_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn AFEC_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn SSP2_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn DMA1_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn DAC_IRQHandler() {
    loop {}
}

/// This function handles LORA Interrupts.
#[unsafe(no_mangle)]
pub extern "C" fn LORA_IRQHandler() {
    radio_on_dio_irq();
}

#[unsafe(no_mangle)]
pub extern "C" fn GPIO_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn TIMER0_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn TIMER1_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn TIMER2_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn TIMER3_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn BSTIMER0_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn BSTIMER1_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn LPTIMER0_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn SAC_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn DMA0_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn I2S_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_IRQHandler() {
    loop {}
}

/// This function handles PWR Handler.
#[unsafe(no_mangle)]
pub extern "C" fn PWR_IRQHandler() {}

#[unsafe(no_mangle)]
pub extern "C" fn LPTIMER1_IRQHandler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn IWDG_IRQHandler() {
    loop {}
}
