use crate::{
    peripherals::{
        rcc::{
            RCC_PERIPHERAL_TIMER0, RCC_PERIPHERAL_TIMER1, RCC_PERIPHERAL_TIMER2,
            RCC_PERIPHERAL_TIMER3,
        },
        regs::{RCC, TIMER0_SFR_BASE, TIMER1_SFR_BASE, TIMER2_SFR_BASE, TIMER3_SFR_BASE, TimerGp},
    },
    toggle_reg_bits,
};

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerChannel {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
}

pub const TIMER_DCR_DBL_POSITION: u32 = 8;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerCountMode {
    Up = 0xFFFF,
    Down = 0x0,
    CenterAligned1 = 0x20,
    CenterAligned2 = 0x40,
    CenterAligned3 = 0x60,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerClockDivision {
    Div1 = 0x0,
    Div2 = 0x100,
    Div4 = 0x200,
    Reserved = 0x300,
}

#[derive(Clone, Copy)]
pub struct TimerConfig {
    pub prescaler: u32,
    pub counter_mode: TimerCountMode,
    pub period: u32,
    pub clock_division: TimerClockDivision,
    pub autoreload_preload: bool,
    pub resv: u8,
}

#[repr(u32)]
pub enum TimerCr1 {
    Cen = 0x1,
    Udis = 0x2,
    Urs = 0x4,
    Opm = 0x8,
    Dir = 0x10,
    AutoreloadPreload = 0x80,
}

#[repr(u32)]
pub enum TimerMasterMode {
    Reset = 0x0,
    Enable = 0x10,
    Update = 0x20,
    Oc1 = 0x30,
    Oc0Ref = 0x40,
    Oc1Ref = 0x50,
    Oc2Ref = 0x60,
    Oc3Ref = 0x70,
}

#[repr(u32)]
pub enum TimerCr2 {
    CcdsUpd = 0x8,
    TiosXor = 0x80,
}

/// TIMER ETR Prescaler
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerEtrPrescaler {
    /// No prescaler is used
    Div1 = 0x0,
    /// ETR input source is divided by 2
    Div2 = 0x1000,
    /// ETR input source is divided by 4
    Div4 = 0x2000,
    /// ETR input source is divided by 8
    Div8 = 0x3000,
}

/// TIMER External trigger filter
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerEtf {
    /// No filter
    Etf0 = 0x0,
    /// Sample frequence is fpclk, filter length is 2
    Etf1 = 0x100,
    /// Sample frequence is fpclk, filter length is 4
    Etf2 = 0x200,
    /// Sample frequence is fpclk, filter length is 8
    Etf3 = 0x300,
    /// Sample frequence is fDTS/2, filter length is 6
    Etf4 = 0x400,
    /// Sample frequence is fDTS/2, filter length is 8
    Etf5 = 0x500,
    /// Sample frequence is fDTS/4, filter length is 6
    Etf6 = 0x600,
    /// Sample frequence is fDTS/4, filter length is 8
    Etf7 = 0x700,
    /// Sample frequence is fDTS/8, filter length is 6
    Etf8 = 0x800,
    /// Sample frequence is fDTS/8, filter length is 8
    Etf9 = 0x900,
    /// Sample frequence is fDTS/16, filter length is 5
    Etf10 = 0xa00,
    /// Sample frequence is fDTS/16, filter length is 6
    Etf11 = 0xb00,
    /// Sample frequence is fDTS/16, filter length is 8
    Etf12 = 0xc00,
    /// Sample frequence is fDTS/32, filter length is 5
    Etf13 = 0xd00,
    /// Sample frequence is fDTS/32, filter length is 6
    Etf14 = 0xe00,
    /// Sample frequence is fDTS/32, filter length is 8
    Etf15 = 0xf00,
}

/// TIMER Trigger source selection
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTs {
    /// Internal Trigger 0 (ITR0)
    Itr0 = 0x0,
    /// Internal Trigger 1 (ITR1)
    Itr1 = 0x10,
    /// Internal Trigger 2 (ITR2)
    Itr2 = 0x20,
    /// TI0 Edge Detector (TI0F_ED)
    Ti0fEd = 0x40,
    /// Filtered Timer Input 0 (TI0FP0)
    Ti0fp0 = 0x50,
    /// Filtered Timer Input 1 (TI1FP1)
    Ti1fp1 = 0x60,
    /// Filtered External Trigger input (ETRF)
    Etrf = 0x70,
}

/// TIMER Slave mode selection
#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TimerSms {
    /// Slave mode disabled
    Disable = 0x0,
    /// Encoder mode 1, counts up/down on TI0FP0 edge depending on TI1FP1 level
    Encoder1 = 0x1,
    /// Encoder mode 2, counts up/down on TI1FP1 edge depending on TI0FP0 level
    Encoder2 = 0x2,
    /// Encoder mode 3, counts up/down on both TI0FP0 and TI1FP1 edges depending on the level of the other input
    Encoder3 = 0x3,
    /// Reset Mode
    Reset = 0x4,
    /// Gated Mode
    Gated = 0x5,
    /// Trigger Mode
    Trigger = 0x6,
    /// External Clock Mode 1
    External1 = 0x7,
}

/// TIMER smcr configuration
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerSmcr {
    /// Master/slave mode is synchronized
    MsmSync = 0x80,
    /// External Clock Mode 2 enable
    EceEnable = 0x4000,
    /// Polarity for ETR source
    EtpInverted = 0x8000,
}

/// TIMER channel 1 and 3 input filter
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerIc1f {
    /// No filter
    Ic1f0 = 0x0,
    /// Sample frequence is fpclk, filter length is 2
    Ic1f1 = 0x1000,
    /// Sample frequence is fpclk, filter length is 4
    Ic1f2 = 0x2000,
    /// Sample frequence is fpclk, filter length is 8
    Ic1f3 = 0x3000,
    /// Sample frequence is fDTS/2, filter length is 6
    Ic1f4 = 0x4000,
    /// Sample frequence is fDTS/2, filter length is 8
    Ic1f5 = 0x5000,
    /// Sample frequence is fDTS/4, filter length is 6
    Ic1f6 = 0x6000,
    /// Sample frequence is fDTS/4, filter length is 8
    Ic1f7 = 0x7000,
    /// Sample frequence is fDTS/8, filter length is 6
    Ic1f8 = 0x8000,
    /// Sample frequence is fDTS/8, filter length is 8
    Ic1f9 = 0x9000,
    /// Sample frequence is fDTS/16, filter length is 5
    Ic1f10 = 0xa000,
    /// Sample frequence is fDTS/16, filter length is 6
    Ic1f11 = 0xb000,
    /// Sample frequence is fDTS/16, filter length is 8
    Ic1f12 = 0xc000,
    /// Sample frequence is fDTS/32, filter length is 5
    Ic1f13 = 0xd000,
    /// Sample frequence is fDTS/32, filter length is 6
    Ic1f14 = 0xe000,
    /// Sample frequence is fDTS/32, filter length is 8
    Ic1f15 = 0xf000,
}

/// TIMER channel 0 and 2 input filter
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerIc0f {
    /// No filter
    Ic0f0 = 0x0,
    /// Sample frequence is fpclk, filter length is 2
    Ic0f1 = 0x10,
    /// Sample frequence is fpclk, filter length is 4
    Ic0f2 = 0x20,
    /// Sample frequence is fpclk, filter length is 8
    Ic0f3 = 0x30,
    /// Sample frequence is fDTS/2, filter length is 6
    Ic0f4 = 0x40,
    /// Sample frequence is fDTS/2, filter length is 8
    Ic0f5 = 0x50,
    /// Sample frequence is fDTS/4, filter length is 6
    Ic0f6 = 0x60,
    /// Sample frequence is fDTS/4, filter length is 8
    Ic0f7 = 0x70,
    /// Sample frequence is fDTS/8, filter length is 6
    Ic0f8 = 0x80,
    /// Sample frequence is fDTS/8, filter length is 8
    Ic0f9 = 0x90,
    /// Sample frequence is fDTS/16, filter length is 5
    Ic0f10 = 0xa0,
    /// Sample frequence is fDTS/16, filter length is 6
    Ic0f11 = 0xb0,
    /// Sample frequence is fDTS/16, filter length is 8
    Ic0f12 = 0xc0,
    /// Sample frequence is fDTS/32, filter length is 5
    Ic0f13 = 0xd0,
    /// Sample frequence is fDTS/32, filter length is 6
    Ic0f14 = 0xe0,
    /// Sample frequence is fDTS/32, filter length is 8
    Ic0f15 = 0xf0,
}

/// TIMER channel 0 input capture polarity
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerCc0pInput {
    /// Capture triggered by rising edge on timer input
    Rising = 0x0,
    /// Capture triggered by falling edge on timer input
    Falling = 0x2,
    /// Capture triggered by both rising and falling edges on timer input
    BothEdge = 0xa,
}

/// TIMER channel 1 input capture polarity
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerCc1pInput {
    /// Capture triggered by rising edge on timer input
    Rising = 0x0,
    /// Capture triggered by falling edge on timer input
    Falling = 0x20,
    /// Capture triggered by both rising and falling edges on timer input
    BothEdge = 0xa0,
}

/// TIMER channel 2 input capture polarity
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerCc2pInput {
    /// Capture triggered by rising edge on timer input
    Rising = 0x0,
    /// Capture triggered by falling edge on timer input
    Falling = 0x200,
    /// Capture triggered by both rising and falling edges on timer input
    BothEdge = 0xa00,
}

/// TIMER channel 3 input capture polarity
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerCc3pInput {
    /// Capture triggered by rising edge on timer input
    Rising = 0x0,
    /// Capture triggered by falling edge on timer input
    Falling = 0x2000,
    /// Capture triggered by both rising and falling edges on timer input
    BothEdge = 0xa000,
}

/// TIMER channel polarity
pub union TimerPolarity {
    pub cc0p_polarity: TimerCc0pInput,
    pub cc1p_polarity: TimerCc1pInput,
    pub cc2p_polarity: TimerCc2pInput,
    pub cc3p_polarity: TimerCc3pInput,
}

/// TIMER channel filter
pub union TimerFilter {
    pub ic0f_filter: TimerIc0f,
    pub ic1f_filter: TimerIc1f,
}

/// TIMER Slave configuration Structure definition
pub struct TimerSlaveConfig {
    pub slave_mode: TimerSms,
    pub input_trigger: TimerTs,
    pub trigger_polarity: bool,
    pub trigger_prescaler: TimerEtrPrescaler,
    pub trigger_filter: TimerEtf,
    pub ic_polarity: TimerPolarity,
    pub ic_filter: TimerFilter,
}

/// TIMER dma definition
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerDma {
    /// DMA request is triggered by the update event
    Ude = 0x100,
    /// DMA request is triggered by the capture/compare match 0 event
    Cc0de = 0x200,
    /// DMA request is triggered by the capture/compare match 1 event
    Cc1de = 0x400,
    /// DMA request is triggered by the capture/compare match 2 event
    Cc2de = 0x800,
    /// DMA request is triggered by the capture/compare match 3 event
    Cc3de = 0x1000,
    /// DMA request is triggered by the trigger event
    Tde = 0x4000,
}

/// TIMER interrupt definition
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerInterrupt {
    /// Update interrupt
    Uie = 0x1,
    /// Capture/Compare 0 interrupt
    Cc0ie = 0x2,
    /// Capture/Compare 1 interrupt
    Cc1ie = 0x4,
    /// Capture/Compare 2 interrupt
    Cc2ie = 0x8,
    /// Capture/Compare 3 interrupt
    Cc3ie = 0x10,
    /// Trigger interrupt
    Tie = 0x40,
}

/// TIMER Flag definition
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerSr {
    /// Update interrupt flag
    Uif = 0x1,
    /// Capture/Compare 0 interrupt flag
    Cc0if = 0x2,
    /// Capture/Compare 1 interrupt flag
    Cc1if = 0x4,
    /// Capture/Compare 2 interrupt flag
    Cc2if = 0x8,
    /// Capture/Compare 3 interrupt flag
    Cc3if = 0x10,
    /// Trigger interrupt flag
    Tif = 0x40,
    /// Capture 0 overcapture flag
    Cc0of = 0x200,
    /// Capture 1 overcapture flag
    Cc1of = 0x400,
    /// Capture 2 overcapture flag
    Cc2of = 0x800,
    /// Capture 3 overcapture flag
    Cc3of = 0x1000,
}

/// TIMER Event register
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerEgr {
    /// Reinitialize the counter and generates an update
    Ug = 0x1,
    /// A capture/compare event is generated on channel 0
    Cc0g = 0x2,
    /// A capture/compare event is generated on channel 1
    Cc1g = 0x4,
    /// A capture/compare event is generated on channel 2
    Cc2g = 0x8,
    /// A capture/compare event is generated on channel 3
    Cc3g = 0x10,
    /// A trigger event is generated
    Tg = 0x40,
}

/// TIMER Channel 1 and 3 output compare and PWM modes
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerOc1m {
    /// Frozen
    Timing = 0x0,
    /// Set channel to active level on match
    Active = 0x1000,
    /// Set channel to inactive level on match
    Inactive = 0x2000,
    /// Toggle
    Toggle = 0x3000,
    /// Force active level
    ForcedInactive = 0x4000,
    /// Force inactive level
    ForcedActive = 0x5000,
    /// PWM mode 1
    Pwm1 = 0x6000,
    /// PWM mode 2
    Pwm2 = 0x7000,
}

/// TIMER Channel 0 and 2 output compare and PWM modes
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerOc0m {
    /// Frozen
    Timing = 0x0,
    /// Set channel to active level on match
    Active = 0x10,
    /// Set channel to inactive level on match
    Inactive = 0x20,
    /// Toggle
    Toggle = 0x30,
    /// Force active level
    ForcedInactive = 0x40,
    /// Force inactive level
    ForcedActive = 0x50,
    /// PWM mode 1
    Pwm1 = 0x60,
    /// PWM mode 2
    Pwm2 = 0x70,
}

/// TIMER Channel 1 and 3 capture and compare selection
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerCc1s {
    /// Output mode
    Output = 0x0,
    /// Input to the same channel
    InputSame = 0x100,
    /// Input to the near channel
    InputNear = 0x200,
    /// Input to TRC
    InputTrc = 0x300,
}

/// TIMER Channel 0 and 2 capture and compare selection
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerCc0s {
    /// Output mode
    Output = 0x0,
    /// Input to the same channel
    InputSame = 0x1,
    /// Input to the near channel
    InputNear = 0x2,
    /// Input to TRC
    InputTrc = 0x3,
}

/// TIMER ccmr output mode configuration
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerCcmrOutput {
    /// Output fast state enable
    Oc0fe = 0x4,
    /// Output compare preload enable
    Oc0pe = 0x8,
    /// Output compare clear enable
    Oc0ce = 0x80,
    /// Output fast state enable
    Oc1fe = 0x400,
    /// Output compare preload enable
    Oc1pe = 0x800,
    /// Output compare clear enable
    Oc1ce = 0x8000,
}

/// TIMER channel 1 and 3 input capture prescaler
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerIc1psc {
    /// Clock division: 1
    Div1 = 0x0,
    /// Clock division: 2
    Div2 = 0x400,
    /// Clock division: 4
    Div4 = 0x800,
    /// Clock division: 8
    Div8 = 0xc00,
}

/// TIMER channel 0 and 2 input capture prescaler
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerIc0psc {
    /// Clock division: 1
    Div1 = 0x0,
    /// Clock division: 2
    Div2 = 0x4,
    /// Clock division: 4
    Div4 = 0x8,
    /// Clock division: 8
    Div8 = 0xc,
}

/// TIMER CCER register config
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerCcer {
    /// Capture compare 0 enabled
    Cc0e = 0x1,
    /// Capture/Compare output polarity
    Cc0p = 0x2,
    /// Input enable
    Cc0np = 0x8,
    /// Capture compare 1 enabled
    Cc1e = 0x10,
    /// Capture/Compare output polarity
    Cc1p = 0x20,
    /// Input enable
    Cc1np = 0x80,
    /// Capture compare 2 enabled
    Cc2e = 0x100,
    /// Capture/Compare output polarity
    Cc2p = 0x200,
    /// Input enable
    Cc2np = 0x800,
    /// Capture compare 3 enabled
    Cc3e = 0x1000,
    /// Capture/Compare output polarity
    Cc3p = 0x2000,
    /// Input enable
    Cc3np = 0x8000,
}

/// TIMER channel selection
pub union TimerSelection {
    pub cc0s_selection: TimerCc0s,
    pub cc1s_selection: TimerCc1s,
}

/// TIMER channel prescaler
pub union TimerPrescaler {
    pub ic0psc_prescaler: TimerIc0psc,
    pub ic1psc_prescaler: TimerIc1psc,
}

/// TIMER input capture configuration structure definition
pub struct TimerIcInit {
    pub ic_polarity: TimerPolarity,
    pub ic_selection: TimerSelection,
    pub ic_prescaler: TimerPrescaler,
    pub ic_filter: TimerFilter,
}

/// TIMER channel output mode
pub union TimerOcMode {
    pub oc0m_mode: TimerOc0m,
    pub oc1m_mode: TimerOc1m,
}

/// TIMER Output compare configuration structure definition
pub struct TimerOcInit {
    pub oc_mode: TimerOcMode,
    pub pulse: u32,
    pub high_level: bool,
    pub oc_fast: bool,
}

/// TIMER dma read/write length
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerDbl {
    /// 1 transmission
    Dbl1 = 0x0,
    /// 2 transmissions
    Dbl2 = 0x100,
    /// 3 transmissions
    Dbl3 = 0x200,
    /// 4 transmissions
    Dbl4 = 0x300,
    /// 5 transmissions
    Dbl5 = 0x400,
    /// 6 transmissions
    Dbl6 = 0x500,
    /// 7 transmissions
    Dbl7 = 0x600,
    /// 8 transmissions
    Dbl8 = 0x700,
    /// 9 transmissions
    Dbl9 = 0x800,
    /// 10 transmissions
    Dbl10 = 0x900,
    /// 11 transmissions
    Dbl11 = 0xa00,
    /// 12 transmissions
    Dbl12 = 0xb00,
    /// 13 transmissions
    Dbl13 = 0xc00,
    /// 14 transmissions
    Dbl14 = 0xd00,
    /// 15 transmissions
    Dbl15 = 0xe00,
    /// 16 transmissions
    Dbl16 = 0xf00,
    /// 17 transmissions
    Dbl17 = 0x1000,
    /// 18 transmissions
    Dbl18 = 0x1100,
    /// Reserved
    DblResv = 0x1f00,
}

/// TIMER dma read/write base address
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerDba {
    /// TIM_CR1
    Cr1 = 0x0,
    /// TIM_CR2
    Cr2 = 0x1,
    /// TIM_SMCR
    Smcr = 0x2,
    /// TIM_DIER
    Dier = 0x3,
    /// TIM_SR
    Sr = 0x4,
    /// TIM_EGR
    Egr = 0x5,
    /// TIM_CCMR1
    Ccmr1 = 0x6,
    /// TIM_CCMR2
    Ccmr2 = 0x7,
    /// TIM_CCER
    Ccer = 0x8,
    /// TIM_CNT
    Cnt = 0x9,
    /// TIM_PSC
    Psc = 0xa,
    /// TIM_ARR
    Arr = 0xb,
    /// RESERVED
    Resv1 = 0xc,
    /// TIM_CCR0
    Ccr0 = 0xd,
    /// TIM_CCR1
    Ccr1 = 0xe,
    /// TIM_CCR2
    Ccr2 = 0xf,
    /// TIM_CCR3
    Ccr3 = 0x10,
    /// TIM_DMAR
    Dmar = 0x11,
    /// TIM_OR
    Or = 0x12,
    /// RESERVED
    Res2 = 0x1f,
}

/// TIMER clear OCxREF structure definition
pub struct TimerClearOcxref {
    pub oc_init: TimerOcInit,
    pub trigger_polarity: bool,
    pub trigger_filter: TimerEtf,
}

/// TIMER timer0 TI0 OR remap
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTim0Ti0Or {
    /// TIM0 TI0 input connected to channel 0
    Iom = 0x0,
    /// TIM0 TI0 input connected to uart0 rx
    Uart0 = 0x1,
    /// TIM0 TI0 input connected to uart1 rx
    Uart1 = 0x2,
    /// TIM0 TI0 input connected to uart2 rx
    Uart2 = 0x3,
    /// TIM0 TI0 input connected to uart3 rx
    Uart3 = 0x4,
    /// TIM0 TI0 input connected to uart4 rx
    Uart4 = 0x5,
    /// RESERVED
    Resv = 0xf,
}

/// TIMER timer0 TI3 OR remap
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTim0Ti3Or {
    /// TIM0 TI3 input connected to channel 3
    Iom = 0x0,
    /// TIM0 TI3 input connected to comparator0
    Comp0 = 0x10,
    /// TIM0 TI3 input connected to comparator1
    Comp1 = 0x20,
    /// RESERVED
    Resv = 0x70,
}

/// TIMER timer0 etr OR remap
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTim0EtrOr {
    /// TIM0 etr input connected to etr
    Iom = 0x0,
    /// TIM0 etr input connected to comparator0
    Comp0 = 0x80,
    /// TIM0 etr input connected to comparator1
    Comp1 = 0x100,
    /// TIM0 etr input connected to XO32K
    Xo32k = 0x180,
    /// TIM0 etr input connected to RCO48M
    Rco48m = 0x200,
    /// TIM0 etr input connected to adcctrl awd0
    Adc0 = 0x280,
    /// TIM0 etr input connected to adcctrl awd1
    Adc1 = 0x300,
    /// TIM0 etr input connected to adcctrl awd2
    Adc2 = 0x380,
    /// TIM0 etr input connected to uart0 rx
    Uart0 = 0x400,
    /// TIM0 etr input connected to uart1 rx
    Uart1 = 0x480,
    /// TIM0 etr input connected to uart2 rx
    Uart2 = 0x500,
    /// TIM0 etr input connected to uart3 rx
    Uart3 = 0x580,
    /// TIM0 etr input connected to uart4 rx
    Uart4 = 0x600,
    /// RESERVED
    Resv = 0x780,
}

/// TIMER timer1 TI2 OR remap
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTim1Ti2Or {
    /// TIM1 TI2 input connected to channel 2
    Iom = 0x0,
    /// TIM1 TI2 input connected to TIM3 channel 1
    Tim3Ch1 = 0x1,
    /// RESERVED
    Resv = 0x3,
}

/// TIMER timer2 TI0 OR remap
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTim2Ti0Or {
    /// TIM2 TI0 input connected to channel 0
    Iom = 0x0,
    /// TIM2 TI0 input connected to XO24M
    Xo24m = 0x1,
    /// TIM2 TI0 input connected to XO32M
    Xo32m = 0x2,
    /// TIM2 TI0 input connected to RCO48M
    Rco48m = 0x3,
    /// TIM2 TI0 input connected to XO32K
    Xo32k = 0x4,
    /// TIM2 TI0 input connected to RCO32K
    Rco32k = 0x5,
    /// TIM2 TI0 input connected to mco
    Mco = 0x6,
    /// TIM2 TI0 input connected to comparator0
    Comp0 = 0x7,
    /// TIM2 TI0 input connected to RCO4M
    Rco4m = 0x8,
    /// TIM2 TI0 input connected to rtc wakeup0
    Wakeup0 = 0x9,
    /// TIM2 TI0 input connected to rtc wakeup1
    Wakeup1 = 0xa,
    /// TIM2 TI0 input connected to rtc wakeup2
    Wakeup2 = 0xb,
    /// RESERVED
    Resv = 0x1f,
}

/// TIMER timer2 TI1 OR remap
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTim2Ti1Or {
    /// TIM2 TI1 input connected to channel 1
    Iom = 0x0,
    /// TIM2 TI1 input connected to comparator1
    Comp1 = 0x20,
    /// RESERVED
    Resv = 0x60,
}

/// TIMER timer2 etr OR remap
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTim2EtrOr {
    /// TIM2 etr input connected to etr
    Iom = 0x0,
    /// TIM2 etr input connected to comparator0
    Comp0 = 0x80,
    /// TIM2 etr input connected to comparator1
    Comp1 = 0x100,
    /// TIM2 etr input connected to XO32K
    Xo32k = 0x180,
    /// RESERVED
    Resv = 0x380,
}

/// TIMER timer3 TI0 OR remap
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTim3Ti0Or {
    /// TIM3 TI0 input connected to channel 0
    Iom = 0x0,
    /// TIM3 TI0 input connected to comparator0
    Comp0 = 0x1,
    /// TIM3 TI0 input connected to comparator1
    Comp1 = 0x2,
    /// TIM3 TI0 input connected to uart0 rx
    Uart0 = 0x3,
    /// TIM3 TI0 input connected to uart1 rx
    Uart1 = 0x4,
    /// TIM3 TI0 input connected to uart2 rx
    Uart2 = 0x5,
    /// TIM3 TI0 input connected to uart3 rx
    Uart3 = 0x6,
    /// TIM3 TI0 input connected to uart4 rx
    Uart4 = 0x7,
}

/// TIMER timer3 etr OR remap
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TimerTim3EtrOr {
    /// TIM3 etr input connected to channel 0
    Iom = 0x0,
    /// TIM3 etr input connected to comparator0
    Comp0 = 0x8,
    /// TIM3 etr input connected to comparator1
    Comp1 = 0x10,
    /// TIM3 etr input connected to XO32K
    Xo32k = 0x18,
    /// TIM3 etr input connected to uart0 rx
    Uart0 = 0x20,
    /// TIM3 etr input connected to uart1 rx
    Uart1 = 0x28,
    /// TIM3 etr input connected to uart2 rx
    Uart2 = 0x30,
    /// TIM3 etr input connected to uart3 rx
    Uart3 = 0x38,
    /// TIM3 etr input connected to uart4 rx
    Uart4 = 0x40,
    /// RESERVED
    Resv = 0x78,
}

pub const TIMER_OK: i32 = 0;
pub const TIMER_ERROR: i32 = -1;

impl TimerGp {
    pub fn init(&self, config: TimerConfig) {
        let mut tmpcr1 = self.cr1.read();

        tmpcr1 &= !(TimerCountMode::CenterAligned3 as u32);
        if matches!(
            config.counter_mode,
            TimerCountMode::Up | TimerCountMode::Down
        ) {
            tmpcr1 &= !(TimerCr1::Dir as u32);
            if matches!(config.counter_mode, TimerCountMode::Down) {
                tmpcr1 |= TimerCr1::Dir as u32;
            }
        } else {
            tmpcr1 |= config.counter_mode as u32;
        }

        tmpcr1 &= !(TimerClockDivision::Reserved as u32);
        tmpcr1 |= config.clock_division as u32;

        tmpcr1 &= !(TimerCr1::AutoreloadPreload as u32);
        if config.autoreload_preload {
            tmpcr1 |= TimerCr1::AutoreloadPreload as u32;
        }

        self.cr1.write(tmpcr1);
        self.arr.write(config.period);
        self.psc.write(config.prescaler);
    }

    pub fn timer_deinit(&self) {
        let periph = match self.ptr() as u32 {
            TIMER0_SFR_BASE => RCC_PERIPHERAL_TIMER0,
            TIMER1_SFR_BASE => RCC_PERIPHERAL_TIMER1,
            TIMER2_SFR_BASE => RCC_PERIPHERAL_TIMER2,
            TIMER3_SFR_BASE => RCC_PERIPHERAL_TIMER3,
            _ => unreachable!(),
        };

        RCC.enable_peripheral_clk(periph, false);
        RCC.rst_peripheral(periph, true);
        RCC.rst_peripheral(periph, false);
    }

    pub fn timer_cmd(&self, state: bool) {
        toggle_reg_bits!(self.cr1, TimerCr1::Cen as u32, state);
    }

    pub fn timer_disable_update(&self, state: bool) {
        toggle_reg_bits!(self.cr1, TimerCr1::Udis as u32, state);
    }
    pub fn timer_config_update_source(&self, state: bool) {
        toggle_reg_bits!(self.cr1, TimerCr1::Urs as u32, state);
    }
    pub fn timer_config_master_mode(&self, master_mode: TimerMasterMode) {
        toggle_reg_bits!(self.cr2, TimerMasterMode::Oc3Ref as u32, false);
        toggle_reg_bits!(self.cr2, master_mode as u32, true);
    }
    pub fn timer_enable_xor(&self, state: bool) {
        toggle_reg_bits!(self.cr2, TimerCr2::TiosXor as u32, state);
    }
    pub fn timer_enable_ccds_upd(&self, state: bool) {
        toggle_reg_bits!(self.cr2, TimerCr2::CcdsUpd as u32, state);
    }
    pub fn timer_config_etr(
        &self,
        ext_trg_prescaler: TimerEtrPrescaler,
        ext_trg_polarity: bool,
        ext_trg_filter: TimerEtf,
    ) {
        let mut tmp_smcr = self.smcr.read();

        tmp_smcr &= !(TimerEtf::Etf15 as u32
            | TimerEtrPrescaler::Div8 as u32
            | TimerSmcr::EceEnable as u32
            | TimerSmcr::EtpInverted as u32);

        tmp_smcr |= ext_trg_prescaler as u32 | ext_trg_filter as u32;

        if ext_trg_polarity {
            tmp_smcr |= TimerSmcr::EtpInverted as u32;
        }

        self.smcr.write(tmp_smcr);
    }
    pub fn timer_config_external_clock2(&self, state: bool) {
        toggle_reg_bits!(self.smcr, TimerSmcr::EceEnable as u32, state);
    }
    pub fn timer_config_itrx(&self, input_trigger_source: TimerTs) {
        let mut tmp_smcr = self.smcr.read();
        tmp_smcr &= !(TimerTs::Etrf as u32);
        tmp_smcr |= input_trigger_source as u32;
        self.smcr.write(tmp_smcr);
    }
    pub fn timer_config_slave_mode(&self, slave_mode: TimerSms) {
        toggle_reg_bits!(self.smcr, TimerSms::External1 as u32, false);
        toggle_reg_bits!(self.smcr, slave_mode as u32, true);
    }
    pub fn timer_config_ti(
        &self,
        ic_polarity: u32,
        ic_selection: u32,
        ic_filter: u32,
        channel: TimerChannel,
    ) {
        // Determine which CCMR register and which bit positions to use
        let (cc_e, cc_s_mask, ic_f_mask, cc_p_mask, use_ccmr1) = match channel {
            TimerChannel::Channel0 => (
                TimerCcer::Cc0e as u32,
                TimerCc0s::InputTrc as u32,
                TimerIc0f::Ic0f15 as u32,
                TimerCc0pInput::BothEdge as u32,
                true,
            ),
            TimerChannel::Channel1 => (
                TimerCcer::Cc1e as u32,
                TimerCc1s::InputTrc as u32,
                TimerIc1f::Ic1f15 as u32,
                TimerCc1pInput::BothEdge as u32,
                true,
            ),
            TimerChannel::Channel2 => (
                TimerCcer::Cc2e as u32,
                TimerCc0s::InputTrc as u32,
                TimerIc0f::Ic0f15 as u32,
                TimerCc2pInput::BothEdge as u32,
                false,
            ),
            TimerChannel::Channel3 => (
                TimerCcer::Cc3e as u32,
                TimerCc1s::InputTrc as u32,
                TimerIc1f::Ic1f15 as u32,
                TimerCc3pInput::BothEdge as u32,
                false,
            ),
        };

        // Disable the channel
        let mut tmp_ccer = self.ccer.read();
        tmp_ccer &= !cc_e;

        let ccmr_reg = if use_ccmr1 { &self.ccmr1 } else { &self.ccmr2 };
        let mut tmp_ccmr = ccmr_reg.read();

        // Set selection
        tmp_ccmr &= !cc_s_mask;
        tmp_ccmr |= ic_selection;

        // Set the filter
        tmp_ccmr &= !ic_f_mask;
        tmp_ccmr |= ic_filter;

        // Select the polarity and enable the channel
        tmp_ccer &= !cc_p_mask;
        tmp_ccer |= ic_polarity | cc_e;

        // Write back
        ccmr_reg.write(tmp_ccmr);
        self.ccer.write(tmp_ccer);
    }
    /// Configure input polarity and filter for one TI channel.
    pub fn timer_config_polarity_filter_ti(&self, ic_polarity: u32, ic_filter: u32, channel: u8) {
        let mut tmp_ccer = self.ccer.read();

        // Disable the selected channel before updating configuration bits.
        let (cc_enable, cc_polarity_mask, ic_filter_mask, ccmr_is_1) = match channel {
            0 => (
                TimerCcer::Cc0e as u32,
                TimerCc0pInput::BothEdge as u32,
                TimerIc0f::Ic0f15 as u32,
                true,
            ),
            1 => (
                TimerCcer::Cc1e as u32,
                TimerCc1pInput::BothEdge as u32,
                TimerIc1f::Ic1f15 as u32,
                true,
            ),
            2 => (
                TimerCcer::Cc2e as u32,
                TimerCc2pInput::BothEdge as u32,
                TimerIc0f::Ic0f15 as u32,
                false,
            ),
            3 => (
                TimerCcer::Cc3e as u32,
                TimerCc3pInput::BothEdge as u32,
                TimerIc1f::Ic1f15 as u32,
                false,
            ),
            _ => return,
        };

        tmp_ccer &= !cc_enable;
        let ccmr_reg = if ccmr_is_1 { &self.ccmr1 } else { &self.ccmr2 };
        let mut tmp_ccmr = ccmr_reg.read();

        // Update filter bits in CCMR.
        tmp_ccmr &= !ic_filter_mask;
        tmp_ccmr |= ic_filter;

        // Update polarity bits in CCER.
        tmp_ccer &= !cc_polarity_mask;
        tmp_ccer |= ic_polarity;

        ccmr_reg.write(tmp_ccmr);
        self.ccer.write(tmp_ccer);
    }

    /// Configure timer slave mode.
    pub fn timer_config_slave(&self, slave_config: &TimerSlaveConfig) -> i32 {
        match slave_config.input_trigger {
            TimerTs::Etrf => {
                self.timer_config_etr(
                    slave_config.trigger_prescaler,
                    slave_config.trigger_polarity,
                    slave_config.trigger_filter,
                );
            }
            TimerTs::Ti0fEd => {
                if slave_config.slave_mode == TimerSms::Gated {
                    return TIMER_ERROR;
                }

                // Disable channel 0, then apply TI0 filter and re-enable it.
                let mut tmp_ccer = self.ccer.read();
                tmp_ccer &= !(TimerCcer::Cc0e as u32);
                let mut tmp_ccmr1 = self.ccmr1.read();
                tmp_ccmr1 &= !(TimerIc0f::Ic0f15 as u32);
                tmp_ccmr1 |= unsafe { slave_config.ic_filter.ic0f_filter as u32 };
                tmp_ccer |= TimerCcer::Cc0e as u32;
                self.ccmr1.write(tmp_ccmr1);
                self.ccer.write(tmp_ccer);
            }
            TimerTs::Ti0fp0 => {
                self.timer_config_polarity_filter_ti(
                    unsafe { slave_config.ic_polarity.cc0p_polarity as u32 },
                    unsafe { slave_config.ic_filter.ic0f_filter as u32 },
                    0,
                );
            }
            TimerTs::Ti1fp1 => {
                self.timer_config_polarity_filter_ti(
                    unsafe { slave_config.ic_polarity.cc1p_polarity as u32 },
                    unsafe { slave_config.ic_filter.ic1f_filter as u32 },
                    1,
                );
            }
            TimerTs::Itr0 | TimerTs::Itr1 | TimerTs::Itr2 => {
                // Internal trigger source selected; no TI/ETR preprocessing required.
            }
        }

        // Program trigger selection bits.
        let mut tmp_smcr = self.smcr.read();
        tmp_smcr &= !(TimerTs::Etrf as u32);
        tmp_smcr |= slave_config.input_trigger as u32;
        self.smcr.write(tmp_smcr);

        // Program slave mode bits.
        tmp_smcr = self.smcr.read();
        tmp_smcr &= !(TimerSms::External1 as u32);
        tmp_smcr |= slave_config.slave_mode as u32;
        self.smcr.write(tmp_smcr);

        TIMER_OK
    }

    /// Configure one input capture channel.
    pub fn timer_config_channel_ic(&self, ic_init: &TimerIcInit, channel: u8) {
        match channel {
            0 => {
                self.timer_config_ti(
                    unsafe { ic_init.ic_polarity.cc0p_polarity as u32 },
                    unsafe { ic_init.ic_selection.cc0s_selection as u32 },
                    unsafe { ic_init.ic_filter.ic0f_filter as u32 },
                    TimerChannel::Channel0,
                );

                let mut ccmr1 = self.ccmr1.read();
                ccmr1 &= !(TimerIc0psc::Div8 as u32);
                ccmr1 |= unsafe { ic_init.ic_prescaler.ic0psc_prescaler as u32 };
                self.ccmr1.write(ccmr1);
            }
            1 => {
                self.timer_config_ti(
                    unsafe { ic_init.ic_polarity.cc1p_polarity as u32 },
                    unsafe { ic_init.ic_selection.cc1s_selection as u32 },
                    unsafe { ic_init.ic_filter.ic1f_filter as u32 },
                    TimerChannel::Channel1,
                );

                let mut ccmr1 = self.ccmr1.read();
                ccmr1 &= !(TimerIc1psc::Div8 as u32);
                ccmr1 |= unsafe { ic_init.ic_prescaler.ic0psc_prescaler as u32 };
                self.ccmr1.write(ccmr1);
            }
            2 => {
                self.timer_config_ti(
                    unsafe { ic_init.ic_polarity.cc2p_polarity as u32 },
                    unsafe { ic_init.ic_selection.cc0s_selection as u32 },
                    unsafe { ic_init.ic_filter.ic0f_filter as u32 },
                    TimerChannel::Channel2,
                );

                let mut ccmr2 = self.ccmr2.read();
                ccmr2 &= !(TimerIc0psc::Div8 as u32);
                ccmr2 |= unsafe { ic_init.ic_prescaler.ic0psc_prescaler as u32 };
                self.ccmr2.write(ccmr2);
            }
            3 => {
                self.timer_config_ti(
                    unsafe { ic_init.ic_polarity.cc3p_polarity as u32 },
                    unsafe { ic_init.ic_selection.cc1s_selection as u32 },
                    unsafe { ic_init.ic_filter.ic1f_filter as u32 },
                    TimerChannel::Channel3,
                );

                let mut ccmr2 = self.ccmr2.read();
                ccmr2 &= !(TimerIc1psc::Div8 as u32);
                ccmr2 |= unsafe { ic_init.ic_prescaler.ic0psc_prescaler as u32 };
                self.ccmr2.write(ccmr2);
            }
            _ => {}
        }
    }

    /// Configure output compare mode for one channel.
    pub fn timer_config_oc(&self, oc_init: &TimerOcInit, channel: u8) {
        let tmp_cr2 = self.cr2.read();
        let mut tmp_ccer = self.ccer.read();

        match channel {
            0 => {
                tmp_ccer &= !(TimerCcer::Cc0e as u32);

                let mut tmp_ccmr1 = self.ccmr1.read();
                tmp_ccmr1 &= !(TimerOc0m::Pwm2 as u32);
                tmp_ccmr1 &= !(TimerCc0s::InputTrc as u32);
                tmp_ccmr1 |= unsafe { oc_init.oc_mode.oc0m_mode as u32 };

                tmp_ccer |= TimerCcer::Cc0e as u32;
                tmp_ccer &= !(TimerCcer::Cc0np as u32);
                tmp_ccer &= !(TimerCcer::Cc0p as u32);
                if !oc_init.high_level {
                    tmp_ccer |= TimerCcer::Cc0p as u32;
                }

                self.ccr0.write(oc_init.pulse);
                self.ccmr1.write(tmp_ccmr1);
            }
            1 => {
                tmp_ccer &= !(TimerCcer::Cc1e as u32);

                let mut tmp_ccmr1 = self.ccmr1.read();
                tmp_ccmr1 &= !(TimerOc1m::Pwm2 as u32);
                tmp_ccmr1 &= !(TimerCc1s::InputTrc as u32);
                tmp_ccmr1 |= unsafe { oc_init.oc_mode.oc1m_mode as u32 };

                tmp_ccer |= TimerCcer::Cc1e as u32;
                tmp_ccer &= !(TimerCcer::Cc1np as u32);
                tmp_ccer &= !(TimerCcer::Cc1p as u32);
                if !oc_init.high_level {
                    tmp_ccer |= TimerCcer::Cc1p as u32;
                }

                self.ccr1.write(oc_init.pulse);
                self.ccmr1.write(tmp_ccmr1);
            }
            2 => {
                tmp_ccer &= !(TimerCcer::Cc2e as u32);

                let mut tmp_ccmr2 = self.ccmr2.read();
                tmp_ccmr2 &= !(TimerOc0m::Pwm2 as u32);
                tmp_ccmr2 &= !(TimerCc0s::InputTrc as u32);
                tmp_ccmr2 |= unsafe { oc_init.oc_mode.oc0m_mode as u32 };

                tmp_ccer |= TimerCcer::Cc2e as u32;
                tmp_ccer &= !(TimerCcer::Cc2np as u32);
                tmp_ccer &= !(TimerCcer::Cc2p as u32);
                if !oc_init.high_level {
                    tmp_ccer |= TimerCcer::Cc2p as u32;
                }

                self.ccr2.write(oc_init.pulse);
                self.ccmr2.write(tmp_ccmr2);
            }
            3 => {
                tmp_ccer &= !(TimerCcer::Cc3e as u32);

                let mut tmp_ccmr2 = self.ccmr2.read();
                tmp_ccmr2 &= !(TimerOc1m::Pwm2 as u32);
                tmp_ccmr2 &= !(TimerCc1s::InputTrc as u32);
                tmp_ccmr2 |= unsafe { oc_init.oc_mode.oc1m_mode as u32 };

                tmp_ccer |= TimerCcer::Cc3e as u32;
                tmp_ccer &= !(TimerCcer::Cc3np as u32);
                tmp_ccer &= !(TimerCcer::Cc3p as u32);
                if !oc_init.high_level {
                    tmp_ccer |= TimerCcer::Cc3p as u32;
                }

                self.ccr3.write(oc_init.pulse);
                self.ccmr2.write(tmp_ccmr2);
            }
            _ => return,
        }

        self.cr2.write(tmp_cr2);
        self.ccer.write(tmp_ccer);
    }

    /// Enable or disable one pulse mode.
    pub fn timer_config_one_pulse(&self, state: bool) {
        toggle_reg_bits!(self.cr1, TimerCr1::Opm as u32, state);
    }

    /// Configure trigger source selection in SMCR.
    pub fn timer_config_ts(&self, ts: TimerTs) {
        let mut smcr = self.smcr.read();
        smcr &= !(TimerTs::Etrf as u32);
        smcr |= ts as u32;
        self.smcr.write(smcr);
    }

    /// Configure clear OCxREF behavior using ETR input.
    pub fn timer_clear_ocxref(&self, clear_ocxref: &TimerClearOcxref, channel: u8) -> i32 {
        if channel == 0 || channel == 2 {
            let oc_mode = unsafe { clear_ocxref.oc_init.oc_mode.oc0m_mode as u32 };
            if oc_mode != TimerOc0m::Pwm1 as u32 && oc_mode != TimerOc0m::Pwm2 as u32 {
                return TIMER_ERROR;
            }

            if channel == 0 {
                self.ccmr1.write(
                    self.ccmr1.read()
                        | TimerCcmrOutput::Oc0pe as u32
                        | TimerCcmrOutput::Oc0ce as u32,
                );
            } else {
                self.ccmr2.write(
                    self.ccmr2.read()
                        | TimerCcmrOutput::Oc0pe as u32
                        | TimerCcmrOutput::Oc0ce as u32,
                );
            }
        } else {
            let oc_mode = unsafe { clear_ocxref.oc_init.oc_mode.oc1m_mode as u32 };
            if oc_mode != TimerOc1m::Pwm1 as u32 && oc_mode != TimerOc1m::Pwm2 as u32 {
                return TIMER_ERROR;
            }

            if channel == 1 {
                self.ccmr1.write(
                    self.ccmr1.read()
                        | TimerCcmrOutput::Oc1pe as u32
                        | TimerCcmrOutput::Oc1ce as u32,
                );
            } else {
                self.ccmr2.write(
                    self.ccmr2.read()
                        | TimerCcmrOutput::Oc1pe as u32
                        | TimerCcmrOutput::Oc1ce as u32,
                );
            }
        }

        self.timer_config_oc(&clear_ocxref.oc_init, channel);
        self.timer_config_etr(
            TimerEtrPrescaler::Div1,
            clear_ocxref.trigger_polarity,
            clear_ocxref.trigger_filter,
        );

        TIMER_OK
    }

    /// Configure PWM output and base timer parameters.
    pub fn timer_config_pwm(&self, oc_init: &TimerOcInit, timerx_init: &TimerConfig, channel: u8) {
        if channel == 0 || channel == 2 {
            let mode = unsafe { oc_init.oc_mode.oc0m_mode as u32 };
            if mode != TimerOc0m::Pwm1 as u32 && mode != TimerOc0m::Pwm2 as u32 {
                return;
            }
        } else {
            let mode = unsafe { oc_init.oc_mode.oc1m_mode as u32 };
            if mode != TimerOc1m::Pwm1 as u32 && mode != TimerOc1m::Pwm2 as u32 {
                return;
            }
        }

        self.timer_config_oc(oc_init, channel);
        self.init(*timerx_init);
    }

    /// Enable or disable event generation bits in EGR.
    pub fn timer_generate_event(&self, egr_event: TimerEgr, state: bool) {
        toggle_reg_bits!(self.egr, egr_event as u32, state);
    }

    /// Enable or disable timer interrupts.
    pub fn timer_config_interrupt(&self, interrupt: TimerInterrupt, state: bool) {
        toggle_reg_bits!(self.dier, interrupt as u32, state);
    }

    /// Read one timer status flag.
    pub fn timer_get_status(&self, status: TimerSr) -> bool {
        (self.sr.read() & status as u32) == status as u32
    }

    /// Clear one timer status flag.
    pub fn timer_clear_status(&self, status: TimerSr) {
        self.sr.write(self.sr.read() & !(status as u32));
    }

    /// Enable or disable timer DMA requests.
    pub fn timer_config_dma(&self, dma: TimerDma, state: bool) {
        toggle_reg_bits!(self.dier, dma as u32, state);
    }

    /// Set DMA burst read/write length.
    pub fn timer_set_dma_rw_len(&self, dbl_len: TimerDbl) {
        let mut dcr = self.dcr.read();
        dcr &= !(TimerDbl::DblResv as u32);
        dcr |= dbl_len as u32;
        self.dcr.write(dcr);
    }

    /// Set DMA burst base address.
    pub fn timer_set_dma_base_addr(&self, dba_addr: TimerDba) {
        let mut dcr = self.dcr.read();
        dcr &= !(TimerDba::Res2 as u32);
        dcr |= dba_addr as u32;
        self.dcr.write(dcr);
    }

    /// Configure TIM0 TI0 OR remap.
    pub fn timer_config_or_tim0_ti0(&self, tim0_ti0_or: TimerTim0Ti0Or) {
        let mut or = self.or.read();
        or &= !(TimerTim0Ti0Or::Resv as u32);
        or |= tim0_ti0_or as u32;
        self.or.write(or);
    }

    /// Configure TIM0 TI3 OR remap.
    pub fn timer_config_or_tim0_ti3(&self, tim0_ti3_or: TimerTim0Ti3Or) {
        let mut or = self.or.read();
        or &= !(TimerTim0Ti3Or::Resv as u32);
        or |= tim0_ti3_or as u32;
        self.or.write(or);
    }

    /// Configure TIM0 ETR OR remap.
    pub fn timer_config_or_tim0_etr(&self, tim0_etr_or: TimerTim0EtrOr) {
        let mut or = self.or.read();
        or &= !(TimerTim0EtrOr::Resv as u32);
        or |= tim0_etr_or as u32;
        self.or.write(or);
    }

    /// Configure TIM1 TI2 OR remap.
    pub fn timer_config_or_tim1_ti2(&self, tim1_ti2_or: TimerTim1Ti2Or) {
        let mut or = self.or.read();
        or &= !(TimerTim1Ti2Or::Resv as u32);
        or |= tim1_ti2_or as u32;
        self.or.write(or);
    }

    /// Configure TIM2 TI0 OR remap.
    pub fn timer_config_or_tim2_ti0(&self, tim2_ti0_or: TimerTim2Ti0Or) {
        let mut or = self.or.read();
        or &= !(TimerTim2Ti0Or::Resv as u32);
        or |= tim2_ti0_or as u32;
        self.or.write(or);
    }

    /// Configure TIM2 TI1 OR remap.
    pub fn timer_config_or_tim2_ti1(&self, tim2_ti1_or: TimerTim2Ti1Or) {
        let mut or = self.or.read();
        or &= !(TimerTim2Ti1Or::Resv as u32);
        or |= tim2_ti1_or as u32;
        self.or.write(or);
    }

    /// Configure TIM2 ETR OR remap.
    pub fn timer_config_or_tim2_etr(&self, tim2_etr_or: TimerTim2EtrOr) {
        let mut or = self.or.read();
        or &= !(TimerTim2EtrOr::Resv as u32);
        or |= tim2_etr_or as u32;
        self.or.write(or);
    }

    /// Configure TIM3 TI0 OR remap.
    pub fn timer_config_or_tim3_ti0(&self, tim3_ti0_or: TimerTim3Ti0Or) {
        let mut or = self.or.read();
        or &= !(TimerTim3Ti0Or::Uart4 as u32);
        or |= tim3_ti0_or as u32;
        self.or.write(or);
    }

    /// Configure TIM3 ETR OR remap.
    pub fn timer_config_or_tim3_etr(&self, tim3_etr_or: TimerTim3EtrOr) {
        let mut or = self.or.read();
        or &= !(TimerTim3EtrOr::Resv as u32);
        or |= tim3_etr_or as u32;
        self.or.write(or);
    }
}
