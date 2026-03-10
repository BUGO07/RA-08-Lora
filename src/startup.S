  .syntax unified
  .cpu cortex-m4
  .thumb

.global  g_pfnVectors
.global  Default_Handler

/* start address for the initialization values of the .data section.
defined in linker script */
.word  _sidata
/* start address for the .data section. defined in linker script */
.word  _sdata
/* end address for the .data section. defined in linker script */
.word  _edata
/* start address for the .bss section. defined in linker script */
.word  _sbss
/* end address for the .bss section. defined in linker script */
.word  _ebss
/* stack used for SystemInit_ExtMemCtl; always internal RAM used */

/**
 * @brief  This is the code that gets called when the processor first
 *          starts execution following a reset event. Only the absolutely
 *          necessary set is performed, after which the application
 *          supplied main() routine is called.
 * @param  None
 * @retval : None
*/

  .section  .text.Reset_Handler
  .weak  Reset_Handler
  .type  Reset_Handler, %function
Reset_Handler:
/* Copy the data segment initializers from flash to SRAM */
  movs  r1, #0
  b  LoopCopyDataInit

CopyDataInit:
  ldr  r3, =_sidata
  ldr  r3, [r3, r1]
  str  r3, [r0, r1]
  adds  r1, r1, #4

LoopCopyDataInit:
  ldr  r0, =_sdata
  ldr  r3, =_edata
  adds  r2, r0, r1
  cmp  r2, r3
  bcc  CopyDataInit
  ldr  r2, =_sbss
  b  LoopFillZerobss
/* Zero fill the bss segment. */
FillZerobss:
  movs  r3, #0
  str  r3, [r2], #4

LoopFillZerobss:
  ldr  r3, = _ebss
  cmp  r2, r3
  bcc  FillZerobss

/* Call the clock system intitialization function.*/
  bl  system_init

/* Call the application's entry point.*/
  bl  main
  
LoopForever:
  b LoopForever
	
.size  Reset_Handler, .-Reset_Handler

/**
 * @brief  This is the code that gets called when the processor receives an
 *         unexpected interrupt.  This simply enters an infinite loop, preserving
 *         the system state for examination by a debugger.
 * @param  None
 * @retval None
*/
    .section  .text.Default_Handler,"ax",%progbits
Default_Handler:
Infinite_Loop:
  b  Infinite_Loop
  .size  Default_Handler, .-Default_Handler
/******************************************************************************
*
* The minimal vector table for a Cortex M3. Note that the proper constructs
* must be placed on this to ensure that it ends up at physical address
* 0x0000.0000.
*
*******************************************************************************/
  .section  .isr_vector,"a",%progbits
  .type  g_pfnVectors, %object
  .size  g_pfnVectors, .-g_pfnVectors


g_pfnVectors:
  .word  _estack
  .word  Reset_Handler
  .word  NMI_Handler
  .word  HardFault_Handler
  .word  MemManage_Handler
  .word  BusFault_Handler
  .word  UsageFault_Handler
  .word  0
  .word  0
  .word  0
  .word  0
  .word  SVC_Handler
  .word  0
  .word  0
  .word  PendSV_Handler
  .word  SysTick_Handler

  /* External Interrupts */
  .word     SEC_IRQHandler
  .word     RTC_IRQHandler
  .word     WDG_IRQHandler
  .word     EFC_IRQHandler
  .word     UART3_IRQHandler
  .word     I2C2_IRQHandler
  .word     UART0_IRQHandler
  .word     UART1_IRQHandler
  .word     UART2_IRQHandler
  .word     LPUART_IRQHandler
  .word     SSP0_IRQHandler
  .word     SSP1_IRQHandler
  .word     QSPI_IRQHandler
  .word     I2C0_IRQHandler
  .word     I2C1_IRQHandler
  .word     SCC_IRQHandler
  .word     ADC_IRQHandler
  .word     AFEC_IRQHandler
  .word     SSP2_IRQHandler
  .word     DMA1_IRQHandler
  .word     DAC_IRQHandler
  .word     LORA_IRQHandler
  .word     GPIO_IRQHandler
  .word     TIMER0_IRQHandler
  .word     TIMER1_IRQHandler
  .word     TIMER2_IRQHandler
  .word     TIMER3_IRQHandler
  .word     BSTIMER0_IRQHandler
  .word     BSTIMER1_IRQHandler
  .word     LPTIMER0_IRQHandler
  .word     SAC_IRQHandler
  .word     DMA0_IRQHandler
  .word     I2S_IRQHandler
  .word     LCD_IRQHandler
  .word     PWR_IRQHandler
  .word     LPTIMER1_IRQHandler
  .word     IWDG_IRQHandler
