// ! Cortex-M4 Core Register definitions and hardware abstraction layer.
// !
// ! This module provides type definitions and constants for the Cortex-M4
// ! processor core registers, including NVIC, SCB, SysTick, ITM, DWT, TPI,
// ! MPU, FPU, and CoreDebug peripherals.

use core::ptr;

use crate::define_reg;

/// Assembly functions
pub mod asm;
/// Cortex functions
pub mod func;
/// System setup
pub mod system;

/// CMSIS HAL main version
pub const CM4_CMSIS_VERSION_MAIN: usize = 0x03;
/// CMSIS HAL sub version
pub const CM4_CMSIS_VERSION_SUB: usize = 0x20;
/// CMSIS HAL version number
pub const CM4_CMSIS_VERSION: usize = (CM4_CMSIS_VERSION_MAIN << 16) | CM4_CMSIS_VERSION_SUB;
/// Cortex-M Core identifier
pub const CORTEX_M: usize = 0x04;

// ---------------------------------------------------------------------------
// Configuration — adjust these to match your device header
// ---------------------------------------------------------------------------

/// Number of NVIC priority bits implemented by this device.
pub const NVIC_PRIO_BITS: usize = 3;

/// Whether an FPU is present (1) or not (0).
pub const FPU_PRESENT: usize = 1;

/// Whether an MPU is present (1) or not (0).
pub const MPU_PRESENT: usize = 0;

pub const LIBRARY_NORMAL_INTERRUPT_PRIORITY: usize = 6;

// ---------------------------------------------------------------------------
// Core Status and Control Register types
// ---------------------------------------------------------------------------

/// Application Program Status Register (APSR) bit-field layout.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApsrBits {
    _reserved0: u16,
    /// Greater than or Equal flags (bits 16..19, Cortex-M4 only)
    pub ge: u8, // only lower 4 bits used
    _reserved1: u8, // bits 20..26 (7 bits) + Q/V/C/Z/N packed below
    /// Packed flags: Q(27), V(28), C(29), Z(30), N(31)
    pub flags: u8,
}

/// Union type to access the Application Program Status Register (APSR).
#[repr(C)]
pub union ApsrType {
    /// Word access
    pub w: usize,
}

/// Union type to access the Interrupt Program Status Register (IPSR).
#[repr(C)]
pub union IpsrType {
    /// Word access
    pub w: usize,
}

impl IpsrType {
    /// Get the exception number (bits 0..8).
    #[inline]
    pub fn isr(&self) -> u16 {
        (unsafe { self.w } & 0x1FF) as u16
    }
}

/// Union type to access the Special-Purpose Program Status Registers (xPSR).
#[repr(C)]
pub union XpsrType {
    /// Word access
    pub w: usize,
}

/// Union type to access the Control Register (CONTROL).
#[repr(C)]
pub union ControlType {
    /// Word access
    pub w: usize,
}

impl ControlType {
    /// Execution privilege in Thread mode (bit 0).
    #[inline]
    pub fn npriv(&self) -> bool {
        (unsafe { self.w } & (1 << 0)) != 0
    }

    /// Stack to be used (bit 1).
    #[inline]
    pub fn spsel(&self) -> bool {
        (unsafe { self.w } & (1 << 1)) != 0
    }

    /// FP extension active flag (bit 2).
    #[inline]
    pub fn fpca(&self) -> bool {
        (unsafe { self.w } & (1 << 2)) != 0
    }
}

// ---------------------------------------------------------------------------
// NVIC — Nested Vectored Interrupt Controller
// ---------------------------------------------------------------------------

define_reg! {
    /// Nested Vectored Interrupt Controller (NVIC) register block.
    Nvic
    __Nvic {
        /// Interrupt Set Enable Registers (offset 0x000)
        iser: [VolatileRW<usize>; 8],
        _reserved0: [usize; 24],
        /// Interrupt Clear Enable Registers (offset 0x080)
        icer: [VolatileRW<usize>; 8],
        _reserved1: [usize; 24],
        /// Interrupt Set Pending Registers (offset 0x100)
        ispr: [VolatileRW<usize>; 8],
        _reserved2: [usize; 24],
        /// Interrupt Clear Pending Registers (offset 0x180)
        icpr: [VolatileRW<usize>; 8],
        _reserved3: [usize; 24],
        /// Interrupt Active Bit Registers (offset 0x200)
        iabr: [VolatileRW<usize>; 8],
        _reserved4: [usize; 56],
        /// Interrupt Priority Registers, 8-bit wide (offset 0x300)
        ip: [VolatileRW<u8>; 240],
        _reserved5: [usize; 644],
        /// Software Trigger Interrupt Register (offset 0xE00, write-only)
        stir: VolatileRW<usize>,
    }
}

/// STIR: INTID position
pub const NVIC_STIR_INTID_POS: usize = 0;
/// STIR: INTID mask
pub const NVIC_STIR_INTID_MSK: usize = 0x1FF << NVIC_STIR_INTID_POS;

// ---------------------------------------------------------------------------
// SCB — System Control Block
// ---------------------------------------------------------------------------

define_reg! {
    /// System Control Block (SCB) register block.
    Scb
    __Scb {
        /// CPUID Base Register (offset 0x000, read-only)
        cpuid: VolatileRO<usize>,
        /// Interrupt Control and State Register (offset 0x004)
        icsr: VolatileRW<usize>,
        /// Vector Table Offset Register (offset 0x008)
        vtor: VolatileRW<usize>,
        /// Application Interrupt and Reset Control Register (offset 0x00C)
        aircr: VolatileRW<usize>,
        /// System Control Register (offset 0x010)
        scr: VolatileRW<usize>,
        /// Configuration Control Register (offset 0x014)
        ccr: VolatileRW<usize>,
        /// System Handlers Priority Registers (offset 0x018)
        shp: [VolatileRW<u8>; 12],
        /// System Handler Control and State Register (offset 0x024)
        shcsr: VolatileRW<usize>,
        /// Configurable Fault Status Register (offset 0x028)
        cfsr: VolatileRW<usize>,
        /// HardFault Status Register (offset 0x02C)
        hfsr: VolatileRW<usize>,
        /// Debug Fault Status Register (offset 0x030)
        dfsr: VolatileRW<usize>,
        /// MemManage Fault Address Register (offset 0x034)
        mmfar: VolatileRW<usize>,
        /// BusFault Address Register (offset 0x038)
        bfar: VolatileRW<usize>,
        /// Auxiliary Fault Status Register (offset 0x03C)
        afsr: VolatileRW<usize>,
        /// Processor Feature Registers (offset 0x040, read-only)
        pfr: [VolatileRO<usize>; 2],
        /// Debug Feature Register (offset 0x048, read-only)
        dfr: VolatileRO<usize>,
        /// Auxiliary Feature Register (offset 0x04C, read-only)
        adr: VolatileRO<usize>,
        /// Memory Model Feature Registers (offset 0x050, read-only)
        mmfr: [VolatileRO<usize>; 4],
        /// Instruction Set Attributes Registers (offset 0x060, read-only)
        isar: [VolatileRO<usize>; 5],
        _reserved0: [usize; 5],
        /// Coprocessor Access Control Register (offset 0x088)
        cpacr: VolatileRW<usize>,
    }
}

// SCB CPUID Register
/// SCB CPUID: IMPLEMENTER position
pub const SCB_CPUID_IMPLEMENTER_POS: usize = 24;
/// SCB CPUID: IMPLEMENTER mask
pub const SCB_CPUID_IMPLEMENTER_MSK: usize = 0xFF << SCB_CPUID_IMPLEMENTER_POS;
/// SCB CPUID: VARIANT position
pub const SCB_CPUID_VARIANT_POS: usize = 20;
/// SCB CPUID: VARIANT mask
pub const SCB_CPUID_VARIANT_MSK: usize = 0xF << SCB_CPUID_VARIANT_POS;
/// SCB CPUID: ARCHITECTURE position
pub const SCB_CPUID_ARCHITECTURE_POS: usize = 16;
/// SCB CPUID: ARCHITECTURE mask
pub const SCB_CPUID_ARCHITECTURE_MSK: usize = 0xF << SCB_CPUID_ARCHITECTURE_POS;
/// SCB CPUID: PARTNO position
pub const SCB_CPUID_PARTNO_POS: usize = 4;
/// SCB CPUID: PARTNO mask
pub const SCB_CPUID_PARTNO_MSK: usize = 0xFFF << SCB_CPUID_PARTNO_POS;
/// SCB CPUID: REVISION position
pub const SCB_CPUID_REVISION_POS: usize = 0;
/// SCB CPUID: REVISION mask
pub const SCB_CPUID_REVISION_MSK: usize = 0xF << SCB_CPUID_REVISION_POS;

// SCB Interrupt Control State Register
/// SCB ICSR: NMIPENDSET position
pub const SCB_ICSR_NMIPENDSET_POS: usize = 31;
/// SCB ICSR: NMIPENDSET mask
pub const SCB_ICSR_NMIPENDSET_MSK: usize = 1 << SCB_ICSR_NMIPENDSET_POS;
/// SCB ICSR: PENDSVSET position
pub const SCB_ICSR_PENDSVSET_POS: usize = 28;
/// SCB ICSR: PENDSVSET mask
pub const SCB_ICSR_PENDSVSET_MSK: usize = 1 << SCB_ICSR_PENDSVSET_POS;
/// SCB ICSR: PENDSVCLR position
pub const SCB_ICSR_PENDSVCLR_POS: usize = 27;
/// SCB ICSR: PENDSVCLR mask
pub const SCB_ICSR_PENDSVCLR_MSK: usize = 1 << SCB_ICSR_PENDSVCLR_POS;
/// SCB ICSR: PENDSTSET position
pub const SCB_ICSR_PENDSTSET_POS: usize = 26;
/// SCB ICSR: PENDSTSET mask
pub const SCB_ICSR_PENDSTSET_MSK: usize = 1 << SCB_ICSR_PENDSTSET_POS;
/// SCB ICSR: PENDSTCLR position
pub const SCB_ICSR_PENDSTCLR_POS: usize = 25;
/// SCB ICSR: PENDSTCLR mask
pub const SCB_ICSR_PENDSTCLR_MSK: usize = 1 << SCB_ICSR_PENDSTCLR_POS;
/// SCB ICSR: ISRPREEMPT position
pub const SCB_ICSR_ISRPREEMPT_POS: usize = 23;
/// SCB ICSR: ISRPREEMPT mask
pub const SCB_ICSR_ISRPREEMPT_MSK: usize = 1 << SCB_ICSR_ISRPREEMPT_POS;
/// SCB ICSR: ISRPENDING position
pub const SCB_ICSR_ISRPENDING_POS: usize = 22;
/// SCB ICSR: ISRPENDING mask
pub const SCB_ICSR_ISRPENDING_MSK: usize = 1 << SCB_ICSR_ISRPENDING_POS;
/// SCB ICSR: VECTPENDING position
pub const SCB_ICSR_VECTPENDING_POS: usize = 12;
/// SCB ICSR: VECTPENDING mask
pub const SCB_ICSR_VECTPENDING_MSK: usize = 0x1FF << SCB_ICSR_VECTPENDING_POS;
/// SCB ICSR: RETTOBASE position
pub const SCB_ICSR_RETTOBASE_POS: usize = 11;
/// SCB ICSR: RETTOBASE mask
pub const SCB_ICSR_RETTOBASE_MSK: usize = 1 << SCB_ICSR_RETTOBASE_POS;
/// SCB ICSR: VECTACTIVE position
pub const SCB_ICSR_VECTACTIVE_POS: usize = 0;
/// SCB ICSR: VECTACTIVE mask
pub const SCB_ICSR_VECTACTIVE_MSK: usize = 0x1FF << SCB_ICSR_VECTACTIVE_POS;

// SCB Vector Table Offset Register
/// SCB VTOR: TBLOFF position
pub const SCB_VTOR_TBLOFF_POS: usize = 7;
/// SCB VTOR: TBLOFF mask
pub const SCB_VTOR_TBLOFF_MSK: usize = 0x1FFFFFF << SCB_VTOR_TBLOFF_POS;

// SCB Application Interrupt and Reset Control Register
/// SCB AIRCR: VECTKEY position
pub const SCB_AIRCR_VECTKEY_POS: usize = 16;
/// SCB AIRCR: VECTKEY mask
pub const SCB_AIRCR_VECTKEY_MSK: usize = 0xFFFF << SCB_AIRCR_VECTKEY_POS;
/// SCB AIRCR: VECTKEYSTAT position
pub const SCB_AIRCR_VECTKEYSTAT_POS: usize = 16;
/// SCB AIRCR: VECTKEYSTAT mask
pub const SCB_AIRCR_VECTKEYSTAT_MSK: usize = 0xFFFF << SCB_AIRCR_VECTKEYSTAT_POS;
/// SCB AIRCR: ENDIANESS position
pub const SCB_AIRCR_ENDIANESS_POS: usize = 15;
/// SCB AIRCR: ENDIANESS mask
pub const SCB_AIRCR_ENDIANESS_MSK: usize = 1 << SCB_AIRCR_ENDIANESS_POS;
/// SCB AIRCR: PRIGROUP position
pub const SCB_AIRCR_PRIGROUP_POS: usize = 8;
/// SCB AIRCR: PRIGROUP mask
pub const SCB_AIRCR_PRIGROUP_MSK: usize = 7 << SCB_AIRCR_PRIGROUP_POS;
/// SCB AIRCR: SYSRESETREQ position
pub const SCB_AIRCR_SYSRESETREQ_POS: usize = 2;
/// SCB AIRCR: SYSRESETREQ mask
pub const SCB_AIRCR_SYSRESETREQ_MSK: usize = 1 << SCB_AIRCR_SYSRESETREQ_POS;
/// SCB AIRCR: VECTCLRACTIVE position
pub const SCB_AIRCR_VECTCLRACTIVE_POS: usize = 1;
/// SCB AIRCR: VECTCLRACTIVE mask
pub const SCB_AIRCR_VECTCLRACTIVE_MSK: usize = 1 << SCB_AIRCR_VECTCLRACTIVE_POS;
/// SCB AIRCR: VECTRESET position
pub const SCB_AIRCR_VECTRESET_POS: usize = 0;
/// SCB AIRCR: VECTRESET mask
pub const SCB_AIRCR_VECTRESET_MSK: usize = 1 << SCB_AIRCR_VECTRESET_POS;

// SCB System Control Register
/// SCB SCR: SEVONPEND position
pub const SCB_SCR_SEVONPEND_POS: usize = 4;
/// SCB SCR: SEVONPEND mask
pub const SCB_SCR_SEVONPEND_MSK: usize = 1 << SCB_SCR_SEVONPEND_POS;
/// SCB SCR: SLEEPDEEP position
pub const SCB_SCR_SLEEPDEEP_POS: usize = 2;
/// SCB SCR: SLEEPDEEP mask
pub const SCB_SCR_SLEEPDEEP_MSK: usize = 1 << SCB_SCR_SLEEPDEEP_POS;
/// SCB SCR: SLEEPONEXIT position
pub const SCB_SCR_SLEEPONEXIT_POS: usize = 1;
/// SCB SCR: SLEEPONEXIT mask
pub const SCB_SCR_SLEEPONEXIT_MSK: usize = 1 << SCB_SCR_SLEEPONEXIT_POS;

// SCB Configuration Control Register
/// SCB CCR: STKALIGN position
pub const SCB_CCR_STKALIGN_POS: usize = 9;
/// SCB CCR: STKALIGN mask
pub const SCB_CCR_STKALIGN_MSK: usize = 1 << SCB_CCR_STKALIGN_POS;
/// SCB CCR: BFHFNMIGN position
pub const SCB_CCR_BFHFNMIGN_POS: usize = 8;
/// SCB CCR: BFHFNMIGN mask
pub const SCB_CCR_BFHFNMIGN_MSK: usize = 1 << SCB_CCR_BFHFNMIGN_POS;
/// SCB CCR: DIV_0_TRP position
pub const SCB_CCR_DIV_0_TRP_POS: usize = 4;
/// SCB CCR: DIV_0_TRP mask
pub const SCB_CCR_DIV_0_TRP_MSK: usize = 1 << SCB_CCR_DIV_0_TRP_POS;
/// SCB CCR: UNALIGN_TRP position
pub const SCB_CCR_UNALIGN_TRP_POS: usize = 3;
/// SCB CCR: UNALIGN_TRP mask
pub const SCB_CCR_UNALIGN_TRP_MSK: usize = 1 << SCB_CCR_UNALIGN_TRP_POS;
/// SCB CCR: USERSETMPEND position
pub const SCB_CCR_USERSETMPEND_POS: usize = 1;
/// SCB CCR: USERSETMPEND mask
pub const SCB_CCR_USERSETMPEND_MSK: usize = 1 << SCB_CCR_USERSETMPEND_POS;
/// SCB CCR: NONBASETHRDENA position
pub const SCB_CCR_NONBASETHRDENA_POS: usize = 0;
/// SCB CCR: NONBASETHRDENA mask
pub const SCB_CCR_NONBASETHRDENA_MSK: usize = 1 << SCB_CCR_NONBASETHRDENA_POS;

// SCB System Handler Control and State Register
/// SCB SHCSR: USGFAULTENA position
pub const SCB_SHCSR_USGFAULTENA_POS: usize = 18;
/// SCB SHCSR: USGFAULTENA mask
pub const SCB_SHCSR_USGFAULTENA_MSK: usize = 1 << SCB_SHCSR_USGFAULTENA_POS;
/// SCB SHCSR: BUSFAULTENA position
pub const SCB_SHCSR_BUSFAULTENA_POS: usize = 17;
/// SCB SHCSR: BUSFAULTENA mask
pub const SCB_SHCSR_BUSFAULTENA_MSK: usize = 1 << SCB_SHCSR_BUSFAULTENA_POS;
/// SCB SHCSR: MEMFAULTENA position
pub const SCB_SHCSR_MEMFAULTENA_POS: usize = 16;
/// SCB SHCSR: MEMFAULTENA mask
pub const SCB_SHCSR_MEMFAULTENA_MSK: usize = 1 << SCB_SHCSR_MEMFAULTENA_POS;
/// SCB SHCSR: SVCALLPENDED position
pub const SCB_SHCSR_SVCALLPENDED_POS: usize = 15;
/// SCB SHCSR: SVCALLPENDED mask
pub const SCB_SHCSR_SVCALLPENDED_MSK: usize = 1 << SCB_SHCSR_SVCALLPENDED_POS;
/// SCB SHCSR: BUSFAULTPENDED position
pub const SCB_SHCSR_BUSFAULTPENDED_POS: usize = 14;
/// SCB SHCSR: BUSFAULTPENDED mask
pub const SCB_SHCSR_BUSFAULTPENDED_MSK: usize = 1 << SCB_SHCSR_BUSFAULTPENDED_POS;
/// SCB SHCSR: MEMFAULTPENDED position
pub const SCB_SHCSR_MEMFAULTPENDED_POS: usize = 13;
/// SCB SHCSR: MEMFAULTPENDED mask
pub const SCB_SHCSR_MEMFAULTPENDED_MSK: usize = 1 << SCB_SHCSR_MEMFAULTPENDED_POS;
/// SCB SHCSR: USGFAULTPENDED position
pub const SCB_SHCSR_USGFAULTPENDED_POS: usize = 12;
/// SCB SHCSR: USGFAULTPENDED mask
pub const SCB_SHCSR_USGFAULTPENDED_MSK: usize = 1 << SCB_SHCSR_USGFAULTPENDED_POS;
/// SCB SHCSR: SYSTICKACT position
pub const SCB_SHCSR_SYSTICKACT_POS: usize = 11;
/// SCB SHCSR: SYSTICKACT mask
pub const SCB_SHCSR_SYSTICKACT_MSK: usize = 1 << SCB_SHCSR_SYSTICKACT_POS;
/// SCB SHCSR: PENDSVACT position
pub const SCB_SHCSR_PENDSVACT_POS: usize = 10;
/// SCB SHCSR: PENDSVACT mask
pub const SCB_SHCSR_PENDSVACT_MSK: usize = 1 << SCB_SHCSR_PENDSVACT_POS;
/// SCB SHCSR: MONITORACT position
pub const SCB_SHCSR_MONITORACT_POS: usize = 8;
/// SCB SHCSR: MONITORACT mask
pub const SCB_SHCSR_MONITORACT_MSK: usize = 1 << SCB_SHCSR_MONITORACT_POS;
/// SCB SHCSR: SVCALLACT position
pub const SCB_SHCSR_SVCALLACT_POS: usize = 7;
/// SCB SHCSR: SVCALLACT mask
pub const SCB_SHCSR_SVCALLACT_MSK: usize = 1 << SCB_SHCSR_SVCALLACT_POS;
/// SCB SHCSR: USGFAULTACT position
pub const SCB_SHCSR_USGFAULTACT_POS: usize = 3;
/// SCB SHCSR: USGFAULTACT mask
pub const SCB_SHCSR_USGFAULTACT_MSK: usize = 1 << SCB_SHCSR_USGFAULTACT_POS;
/// SCB SHCSR: BUSFAULTACT position
pub const SCB_SHCSR_BUSFAULTACT_POS: usize = 1;
/// SCB SHCSR: BUSFAULTACT mask
pub const SCB_SHCSR_BUSFAULTACT_MSK: usize = 1 << SCB_SHCSR_BUSFAULTACT_POS;
/// SCB SHCSR: MEMFAULTACT position
pub const SCB_SHCSR_MEMFAULTACT_POS: usize = 0;
/// SCB SHCSR: MEMFAULTACT mask
pub const SCB_SHCSR_MEMFAULTACT_MSK: usize = 1 << SCB_SHCSR_MEMFAULTACT_POS;

// SCB Configurable Fault Status Register
/// SCB CFSR: Usage Fault Status Register position
pub const SCB_CFSR_USGFAULTSR_POS: usize = 16;
/// SCB CFSR: Usage Fault Status Register mask
pub const SCB_CFSR_USGFAULTSR_MSK: usize = 0xFFFF << SCB_CFSR_USGFAULTSR_POS;
/// SCB CFSR: Bus Fault Status Register position
pub const SCB_CFSR_BUSFAULTSR_POS: usize = 8;
/// SCB CFSR: Bus Fault Status Register mask
pub const SCB_CFSR_BUSFAULTSR_MSK: usize = 0xFF << SCB_CFSR_BUSFAULTSR_POS;
/// SCB CFSR: Memory Manage Fault Status Register position
pub const SCB_CFSR_MEMFAULTSR_POS: usize = 0;
/// SCB CFSR: Memory Manage Fault Status Register mask
pub const SCB_CFSR_MEMFAULTSR_MSK: usize = 0xFF << SCB_CFSR_MEMFAULTSR_POS;

// SCB Hard Fault Status Register
/// SCB HFSR: DEBUGEVT position
pub const SCB_HFSR_DEBUGEVT_POS: usize = 31;
/// SCB HFSR: DEBUGEVT mask
pub const SCB_HFSR_DEBUGEVT_MSK: usize = 1 << SCB_HFSR_DEBUGEVT_POS;
/// SCB HFSR: FORCED position
pub const SCB_HFSR_FORCED_POS: usize = 30;
/// SCB HFSR: FORCED mask
pub const SCB_HFSR_FORCED_MSK: usize = 1 << SCB_HFSR_FORCED_POS;
/// SCB HFSR: VECTTBL position
pub const SCB_HFSR_VECTTBL_POS: usize = 1;
/// SCB HFSR: VECTTBL mask
pub const SCB_HFSR_VECTTBL_MSK: usize = 1 << SCB_HFSR_VECTTBL_POS;

// SCB Debug Fault Status Register
/// SCB DFSR: EXTERNAL position
pub const SCB_DFSR_EXTERNAL_POS: usize = 4;
/// SCB DFSR: EXTERNAL mask
pub const SCB_DFSR_EXTERNAL_MSK: usize = 1 << SCB_DFSR_EXTERNAL_POS;
/// SCB DFSR: VCATCH position
pub const SCB_DFSR_VCATCH_POS: usize = 3;
/// SCB DFSR: VCATCH mask
pub const SCB_DFSR_VCATCH_MSK: usize = 1 << SCB_DFSR_VCATCH_POS;
/// SCB DFSR: DWTTRAP position
pub const SCB_DFSR_DWTTRAP_POS: usize = 2;
/// SCB DFSR: DWTTRAP mask
pub const SCB_DFSR_DWTTRAP_MSK: usize = 1 << SCB_DFSR_DWTTRAP_POS;
/// SCB DFSR: BKPT position
pub const SCB_DFSR_BKPT_POS: usize = 1;
/// SCB DFSR: BKPT mask
pub const SCB_DFSR_BKPT_MSK: usize = 1 << SCB_DFSR_BKPT_POS;
/// SCB DFSR: HALTED position
pub const SCB_DFSR_HALTED_POS: usize = 0;
/// SCB DFSR: HALTED mask
pub const SCB_DFSR_HALTED_MSK: usize = 1 << SCB_DFSR_HALTED_POS;

// ---------------------------------------------------------------------------
// SCnSCB — System Controls not in SCB
// ---------------------------------------------------------------------------

define_reg! {
    /// System Control and ID Register not in the SCB.
    ScnScb
    __ScnScb {
        _reserved0: [usize; 1],
        /// Interrupt Controller Type Register (offset 0x004, read-only)
        ictr: VolatileRO<usize>,
        /// Auxiliary Control Register (offset 0x008)
        actlr: VolatileRW<usize>,
    }
}

/// ICTR: INTLINESNUM position
pub const SCNSCB_ICTR_INTLINESNUM_POS: usize = 0;
/// ICTR: INTLINESNUM mask
pub const SCNSCB_ICTR_INTLINESNUM_MSK: usize = 0xF << SCNSCB_ICTR_INTLINESNUM_POS;

/// ACTLR: DISOOFP position
pub const SCNSCB_ACTLR_DISOOFP_POS: usize = 9;
/// ACTLR: DISOOFP mask
pub const SCNSCB_ACTLR_DISOOFP_MSK: usize = 1 << SCNSCB_ACTLR_DISOOFP_POS;
/// ACTLR: DISFPCA position
pub const SCNSCB_ACTLR_DISFPCA_POS: usize = 8;
/// ACTLR: DISFPCA mask
pub const SCNSCB_ACTLR_DISFPCA_MSK: usize = 1 << SCNSCB_ACTLR_DISFPCA_POS;
/// ACTLR: DISFOLD position
pub const SCNSCB_ACTLR_DISFOLD_POS: usize = 2;
/// ACTLR: DISFOLD mask
pub const SCNSCB_ACTLR_DISFOLD_MSK: usize = 1 << SCNSCB_ACTLR_DISFOLD_POS;
/// ACTLR: DISDEFWBUF position
pub const SCNSCB_ACTLR_DISDEFWBUF_POS: usize = 1;
/// ACTLR: DISDEFWBUF mask
pub const SCNSCB_ACTLR_DISDEFWBUF_MSK: usize = 1 << SCNSCB_ACTLR_DISDEFWBUF_POS;
/// ACTLR: DISMCYCINT position
pub const SCNSCB_ACTLR_DISMCYCINT_POS: usize = 0;
/// ACTLR: DISMCYCINT mask
pub const SCNSCB_ACTLR_DISMCYCINT_MSK: usize = 1 << SCNSCB_ACTLR_DISMCYCINT_POS;

// ---------------------------------------------------------------------------
// SysTick — System Timer
// ---------------------------------------------------------------------------

define_reg! {
    /// System Timer (SysTick) register block.
    SysTick
    __SysTick {
        /// SysTick Control and Status Register (offset 0x000)
        ctrl: VolatileRW<usize>,
        /// SysTick Reload Value Register (offset 0x004)
        load: VolatileRW<usize>,
        /// SysTick Current Value Register (offset 0x008)
        val: VolatileRW<usize>,
        /// SysTick Calibration Register (offset 0x00C, read-only)
        calib: VolatileRO<usize>,
    }
}

/// SysTick CTRL: COUNTFLAG position
pub const SYSTICK_CTRL_COUNTFLAG_POS: usize = 16;
/// SysTick CTRL: COUNTFLAG mask
pub const SYSTICK_CTRL_COUNTFLAG_MSK: usize = 1 << SYSTICK_CTRL_COUNTFLAG_POS;
/// SysTick CTRL: CLKSOURCE position
pub const SYSTICK_CTRL_CLKSOURCE_POS: usize = 2;
/// SysTick CTRL: CLKSOURCE mask
pub const SYSTICK_CTRL_CLKSOURCE_MSK: usize = 1 << SYSTICK_CTRL_CLKSOURCE_POS;
/// SysTick CTRL: TICKINT position
pub const SYSTICK_CTRL_TICKINT_POS: usize = 1;
/// SysTick CTRL: TICKINT mask
pub const SYSTICK_CTRL_TICKINT_MSK: usize = 1 << SYSTICK_CTRL_TICKINT_POS;
/// SysTick CTRL: ENABLE position
pub const SYSTICK_CTRL_ENABLE_POS: usize = 0;
/// SysTick CTRL: ENABLE mask
pub const SYSTICK_CTRL_ENABLE_MSK: usize = 1 << SYSTICK_CTRL_ENABLE_POS;
/// SysTick LOAD: RELOAD position
pub const SYSTICK_LOAD_RELOAD_POS: usize = 0;
/// SysTick LOAD: RELOAD mask
pub const SYSTICK_LOAD_RELOAD_MSK: usize = 0xFFFFFF << SYSTICK_LOAD_RELOAD_POS;
/// SysTick VAL: CURRENT position
pub const SYSTICK_VAL_CURRENT_POS: usize = 0;
/// SysTick VAL: CURRENT mask
pub const SYSTICK_VAL_CURRENT_MSK: usize = 0xFFFFFF << SYSTICK_VAL_CURRENT_POS;
/// SysTick CALIB: NOREF position
pub const SYSTICK_CALIB_NOREF_POS: usize = 31;
/// SysTick CALIB: NOREF mask
pub const SYSTICK_CALIB_NOREF_MSK: usize = 1 << SYSTICK_CALIB_NOREF_POS;
/// SysTick CALIB: SKEW position
pub const SYSTICK_CALIB_SKEW_POS: usize = 30;
/// SysTick CALIB: SKEW mask
pub const SYSTICK_CALIB_SKEW_MSK: usize = 1 << SYSTICK_CALIB_SKEW_POS;
/// SysTick CALIB: TENMS position
pub const SYSTICK_CALIB_TENMS_POS: usize = 0;
/// SysTick CALIB: TENMS mask
pub const SYSTICK_CALIB_TENMS_MSK: usize = 0xFFFFFF << SYSTICK_CALIB_TENMS_POS;

// ---------------------------------------------------------------------------
// ITM — Instrumentation Trace Macrocell
// ---------------------------------------------------------------------------

/// ITM Stimulus Port union for 8/16/32-bit writes.
#[repr(C)]
pub union ItmStimPort {
    /// 8-bit write access
    pub u8: VolatileRW<u8>,
    /// 16-bit write access
    pub u16: VolatileRW<u16>,
    /// 32-bit write access
    pub usize: VolatileRW<usize>,
}

define_reg! {
    /// Instrumentation Trace Macrocell (ITM) register block.
    Itm
    __Itm {
        /// ITM Stimulus Port Registers (offset 0x000, write-only)
        port: [ItmStimPort; 32],
        _reserved0: [usize; 864],
        /// ITM Trace Enable Register (offset 0xE00)
        ter: VolatileRW<usize>,
        _reserved1: [usize; 15],
        /// ITM Trace Privilege Register (offset 0xE40)
        tpr: VolatileRW<usize>,
        _reserved2: [usize; 15],
        /// ITM Trace Control Register (offset 0xE80)
        tcr: VolatileRW<usize>,
        _reserved3: [usize; 29],
        /// ITM Integration Write Register (offset 0xEF8, write-only)
        iwr: VolatileRW<usize>,
        /// ITM Integration Read Register (offset 0xEFC, read-only)
        irr: VolatileRO<usize>,
        /// ITM Integration Mode Control Register (offset 0xF00)
        imcr: VolatileRW<usize>,
        _reserved4: [usize; 43],
        /// ITM Lock Access Register (offset 0xFB0, write-only)
        lar: VolatileRW<usize>,
        /// ITM Lock Status Register (offset 0xFB4, read-only)
        lsr: VolatileRO<usize>,
        _reserved5: [usize; 6],
        /// ITM Peripheral Identification Register #4 (offset 0xFD0, read-only)
        pid4: VolatileRO<usize>,
        /// ITM Peripheral Identification Register #5 (offset 0xFD4, read-only)
        pid5: VolatileRO<usize>,
        /// ITM Peripheral Identification Register #6 (offset 0xFD8, read-only)
        pid6: VolatileRO<usize>,
        /// ITM Peripheral Identification Register #7 (offset 0xFDC, read-only)
        pid7: VolatileRO<usize>,
        /// ITM Peripheral Identification Register #0 (offset 0xFE0, read-only)
        pid0: VolatileRO<usize>,
        /// ITM Peripheral Identification Register #1 (offset 0xFE4, read-only)
        pid1: VolatileRO<usize>,
        /// ITM Peripheral Identification Register #2 (offset 0xFE8, read-only)
        pid2: VolatileRO<usize>,
        /// ITM Peripheral Identification Register #3 (offset 0xFEC, read-only)
        pid3: VolatileRO<usize>,
        /// ITM Component Identification Register #0 (offset 0xFF0, read-only)
        cid0: VolatileRO<usize>,
        /// ITM Component Identification Register #1 (offset 0xFF4, read-only)
        cid1: VolatileRO<usize>,
        /// ITM Component Identification Register #2 (offset 0xFF8, read-only)
        cid2: VolatileRO<usize>,
        /// ITM Component Identification Register #3 (offset 0xFFC, read-only)
        cid3: VolatileRO<usize>,
    }
}

/// ITM TPR: PRIVMASK position
pub const ITM_TPR_PRIVMASK_POS: usize = 0;
/// ITM TPR: PRIVMASK mask
pub const ITM_TPR_PRIVMASK_MSK: usize = 0xF << ITM_TPR_PRIVMASK_POS;
/// ITM TCR: BUSY position
pub const ITM_TCR_BUSY_POS: usize = 23;
/// ITM TCR: BUSY mask
pub const ITM_TCR_BUSY_MSK: usize = 1 << ITM_TCR_BUSY_POS;
/// ITM TCR: ATBID position
pub const ITM_TCR_TRACE_BUS_ID_POS: usize = 16;
/// ITM TCR: ATBID mask
pub const ITM_TCR_TRACE_BUS_ID_MSK: usize = 0x7F << ITM_TCR_TRACE_BUS_ID_POS;
/// ITM TCR: Global timestamp frequency position
pub const ITM_TCR_GTSFREQ_POS: usize = 10;
/// ITM TCR: Global timestamp frequency mask
pub const ITM_TCR_GTSFREQ_MSK: usize = 3 << ITM_TCR_GTSFREQ_POS;
/// ITM TCR: TSPrescale position
pub const ITM_TCR_TSPRESCALE_POS: usize = 8;
/// ITM TCR: TSPrescale mask
pub const ITM_TCR_TSPRESCALE_MSK: usize = 3 << ITM_TCR_TSPRESCALE_POS;
/// ITM TCR: SWOENA position
pub const ITM_TCR_SWOENA_POS: usize = 4;
/// ITM TCR: SWOENA mask
pub const ITM_TCR_SWOENA_MSK: usize = 1 << ITM_TCR_SWOENA_POS;
/// ITM TCR: DWTENA position
pub const ITM_TCR_DWTENA_POS: usize = 3;
/// ITM TCR: DWTENA mask
pub const ITM_TCR_DWTENA_MSK: usize = 1 << ITM_TCR_DWTENA_POS;
/// ITM TCR: SYNCENA position
pub const ITM_TCR_SYNCENA_POS: usize = 2;
/// ITM TCR: SYNCENA mask
pub const ITM_TCR_SYNCENA_MSK: usize = 1 << ITM_TCR_SYNCENA_POS;
/// ITM TCR: TSENA position
pub const ITM_TCR_TSENA_POS: usize = 1;
/// ITM TCR: TSENA mask
pub const ITM_TCR_TSENA_MSK: usize = 1 << ITM_TCR_TSENA_POS;
/// ITM TCR: ITM Enable bit position
pub const ITM_TCR_ITMENA_POS: usize = 0;
/// ITM TCR: ITM Enable bit mask
pub const ITM_TCR_ITMENA_MSK: usize = 1 << ITM_TCR_ITMENA_POS;
/// ITM IWR: ATVALIDM position
pub const ITM_IWR_ATVALIDM_POS: usize = 0;
/// ITM IWR: ATVALIDM mask
pub const ITM_IWR_ATVALIDM_MSK: usize = 1 << ITM_IWR_ATVALIDM_POS;
/// ITM IRR: ATREADYM position
pub const ITM_IRR_ATREADYM_POS: usize = 0;
/// ITM IRR: ATREADYM mask
pub const ITM_IRR_ATREADYM_MSK: usize = 1 << ITM_IRR_ATREADYM_POS;
/// ITM IMCR: INTEGRATION position
pub const ITM_IMCR_INTEGRATION_POS: usize = 0;
/// ITM IMCR: INTEGRATION mask
pub const ITM_IMCR_INTEGRATION_MSK: usize = 1 << ITM_IMCR_INTEGRATION_POS;
/// ITM LSR: ByteAcc position
pub const ITM_LSR_BYTE_ACC_POS: usize = 2;
/// ITM LSR: ByteAcc mask
pub const ITM_LSR_BYTE_ACC_MSK: usize = 1 << ITM_LSR_BYTE_ACC_POS;
/// ITM LSR: Access position
pub const ITM_LSR_ACCESS_POS: usize = 1;
/// ITM LSR: Access mask
pub const ITM_LSR_ACCESS_MSK: usize = 1 << ITM_LSR_ACCESS_POS;
/// ITM LSR: Present position
pub const ITM_LSR_PRESENT_POS: usize = 0;
/// ITM LSR: Present mask
pub const ITM_LSR_PRESENT_MSK: usize = 1 << ITM_LSR_PRESENT_POS;

// ---------------------------------------------------------------------------
// DWT — Data Watchpoint and Trace
// ---------------------------------------------------------------------------

define_reg! {
    /// Data Watchpoint and Trace (DWT) register block.
    Dwt
    __Dwt {
        /// Control Register (offset 0x000)
        ctrl: VolatileRW<usize>,
        /// Cycle Count Register (offset 0x004)
        cyccnt: VolatileRW<usize>,
        /// CPI Count Register (offset 0x008)
        cpicnt: VolatileRW<usize>,
        /// Exception Overhead Count Register (offset 0x00C)
        exccnt: VolatileRW<usize>,
        /// Sleep Count Register (offset 0x010)
        sleepcnt: VolatileRW<usize>,
        /// LSU Count Register (offset 0x014)
        lsucnt: VolatileRW<usize>,
        /// Folded-instruction Count Register (offset 0x018)
        foldcnt: VolatileRW<usize>,
        /// Program Counter Sample Register (offset 0x01C, read-only)
        pcsr: VolatileRO<usize>,
        /// Comparator Register 0 (offset 0x020)
        comp0: VolatileRW<usize>,
        /// Mask Register 0 (offset 0x024)
        mask0: VolatileRW<usize>,
        /// Function Register 0 (offset 0x028)
        function0: VolatileRW<usize>,
        _reserved0: [usize; 1],
        /// Comparator Register 1 (offset 0x030)
        comp1: VolatileRW<usize>,
        /// Mask Register 1 (offset 0x034)
        mask1: VolatileRW<usize>,
        /// Function Register 1 (offset 0x038)
        function1: VolatileRW<usize>,
        _reserved1: [usize; 1],
        /// Comparator Register 2 (offset 0x040)
        comp2: VolatileRW<usize>,
        /// Mask Register 2 (offset 0x044)
        mask2: VolatileRW<usize>,
        /// Function Register 2 (offset 0x048)
        function2: VolatileRW<usize>,
        _reserved2: [usize; 1],
        /// Comparator Register 3 (offset 0x050)
        comp3: VolatileRW<usize>,
        /// Mask Register 3 (offset 0x054)
        mask3: VolatileRW<usize>,
        /// Function Register 3 (offset 0x058)
        function3: VolatileRW<usize>,
    }
}

/// DWT CTRL: NUMCOMP position
pub const DWT_CTRL_NUMCOMP_POS: usize = 28;
/// DWT CTRL: NUMCOMP mask
pub const DWT_CTRL_NUMCOMP_MSK: usize = 0xF << DWT_CTRL_NUMCOMP_POS;
/// DWT CTRL: NOTRCPKT position
pub const DWT_CTRL_NOTRCPKT_POS: usize = 27;
/// DWT CTRL: NOTRCPKT mask
pub const DWT_CTRL_NOTRCPKT_MSK: usize = 0x1 << DWT_CTRL_NOTRCPKT_POS;
/// DWT CTRL: NOEXTTRIG position
pub const DWT_CTRL_NOEXTTRIG_POS: usize = 26;
/// DWT CTRL: NOEXTTRIG mask
pub const DWT_CTRL_NOEXTTRIG_MSK: usize = 0x1 << DWT_CTRL_NOEXTTRIG_POS;
/// DWT CTRL: NOCYCCNT position
pub const DWT_CTRL_NOCYCCNT_POS: usize = 25;
/// DWT CTRL: NOCYCCNT mask
pub const DWT_CTRL_NOCYCCNT_MSK: usize = 0x1 << DWT_CTRL_NOCYCCNT_POS;
/// DWT CTRL: NOPRFCNT position
pub const DWT_CTRL_NOPRFCNT_POS: usize = 24;
/// DWT CTRL: NOPRFCNT mask
pub const DWT_CTRL_NOPRFCNT_MSK: usize = 0x1 << DWT_CTRL_NOPRFCNT_POS;
/// DWT CTRL: CYCEVTENA position
pub const DWT_CTRL_CYCEVTENA_POS: usize = 22;
/// DWT CTRL: CYCEVTENA mask
pub const DWT_CTRL_CYCEVTENA_MSK: usize = 0x1 << DWT_CTRL_CYCEVTENA_POS;
/// DWT CTRL: FOLDEVTENA position
pub const DWT_CTRL_FOLDEVTENA_POS: usize = 21;
/// DWT CTRL: FOLDEVTENA mask
pub const DWT_CTRL_FOLDEVTENA_MSK: usize = 0x1 << DWT_CTRL_FOLDEVTENA_POS;
/// DWT CTRL: LSUEVTENA position
pub const DWT_CTRL_LSUEVTENA_POS: usize = 20;
/// DWT CTRL: LSUEVTENA mask
pub const DWT_CTRL_LSUEVTENA_MSK: usize = 0x1 << DWT_CTRL_LSUEVTENA_POS;
/// DWT CTRL: SLEEPEVTENA position
pub const DWT_CTRL_SLEEPEVTENA_POS: usize = 19;
/// DWT CTRL: SLEEPEVTENA mask
pub const DWT_CTRL_SLEEPEVTENA_MSK: usize = 0x1 << DWT_CTRL_SLEEPEVTENA_POS;
/// DWT CTRL: EXCEVTENA position
pub const DWT_CTRL_EXCEVTENA_POS: usize = 18;
/// DWT CTRL: EXCEVTENA mask
pub const DWT_CTRL_EXCEVTENA_MSK: usize = 0x1 << DWT_CTRL_EXCEVTENA_POS;
/// DWT CTRL: CPIEVTENA position
pub const DWT_CTRL_CPIEVTENA_POS: usize = 17;
/// DWT CTRL: CPIEVTENA mask
pub const DWT_CTRL_CPIEVTENA_MSK: usize = 0x1 << DWT_CTRL_CPIEVTENA_POS;
/// DWT CTRL: EXCTRCENA position
pub const DWT_CTRL_EXCTRCENA_POS: usize = 16;
/// DWT CTRL: EXCTRCENA mask
pub const DWT_CTRL_EXCTRCENA_MSK: usize = 0x1 << DWT_CTRL_EXCTRCENA_POS;
/// DWT CTRL: PCSAMPLENA position
pub const DWT_CTRL_PCSAMPLENA_POS: usize = 12;
/// DWT CTRL: PCSAMPLENA mask
pub const DWT_CTRL_PCSAMPLENA_MSK: usize = 0x1 << DWT_CTRL_PCSAMPLENA_POS;
/// DWT CTRL: SYNCTAP position
pub const DWT_CTRL_SYNCTAP_POS: usize = 10;
/// DWT CTRL: SYNCTAP mask
pub const DWT_CTRL_SYNCTAP_MSK: usize = 0x3 << DWT_CTRL_SYNCTAP_POS;
/// DWT CTRL: CYCTAP position
pub const DWT_CTRL_CYCTAP_POS: usize = 9;
/// DWT CTRL: CYCTAP mask
pub const DWT_CTRL_CYCTAP_MSK: usize = 0x1 << DWT_CTRL_CYCTAP_POS;
/// DWT CTRL: POSTINIT position
pub const DWT_CTRL_POSTINIT_POS: usize = 5;
/// DWT CTRL: POSTINIT mask
pub const DWT_CTRL_POSTINIT_MSK: usize = 0xF << DWT_CTRL_POSTINIT_POS;
/// DWT CTRL: POSTPRESET position
pub const DWT_CTRL_POSTPRESET_POS: usize = 1;
/// DWT CTRL: POSTPRESET mask
pub const DWT_CTRL_POSTPRESET_MSK: usize = 0xF << DWT_CTRL_POSTPRESET_POS;
/// DWT CTRL: CYCCNTENA position
pub const DWT_CTRL_CYCCNTENA_POS: usize = 0;
/// DWT CTRL: CYCCNTENA mask
pub const DWT_CTRL_CYCCNTENA_MSK: usize = 0x1 << DWT_CTRL_CYCCNTENA_POS;

/// DWT CPICNT: CPICNT position
pub const DWT_CPICNT_CPICNT_POS: usize = 0;
/// DWT CPICNT: CPICNT mask
pub const DWT_CPICNT_CPICNT_MSK: usize = 0xFF << DWT_CPICNT_CPICNT_POS;
/// DWT EXCCNT: EXCCNT position
pub const DWT_EXCCNT_EXCCNT_POS: usize = 0;
/// DWT EXCCNT: EXCCNT mask
pub const DWT_EXCCNT_EXCCNT_MSK: usize = 0xFF << DWT_EXCCNT_EXCCNT_POS;
/// DWT SLEEPCNT: SLEEPCNT position
pub const DWT_SLEEPCNT_SLEEPCNT_POS: usize = 0;
/// DWT SLEEPCNT: SLEEPCNT mask
pub const DWT_SLEEPCNT_SLEEPCNT_MSK: usize = 0xFF << DWT_SLEEPCNT_SLEEPCNT_POS;
/// DWT LSUCNT: LSUCNT position
pub const DWT_LSUCNT_LSUCNT_POS: usize = 0;
/// DWT LSUCNT: LSUCNT mask
pub const DWT_LSUCNT_LSUCNT_MSK: usize = 0xFF << DWT_LSUCNT_LSUCNT_POS;
/// DWT FOLDCNT: FOLDCNT position
pub const DWT_FOLDCNT_FOLDCNT_POS: usize = 0;
/// DWT FOLDCNT: FOLDCNT mask
pub const DWT_FOLDCNT_FOLDCNT_MSK: usize = 0xFF << DWT_FOLDCNT_FOLDCNT_POS;
/// DWT MASK: MASK position
pub const DWT_MASK_MASK_POS: usize = 0;
/// DWT MASK: MASK mask
pub const DWT_MASK_MASK_MSK: usize = 0x1F << DWT_MASK_MASK_POS;

/// DWT FUNCTION: MATCHED position
pub const DWT_FUNCTION_MATCHED_POS: usize = 24;
/// DWT FUNCTION: MATCHED mask
pub const DWT_FUNCTION_MATCHED_MSK: usize = 0x1 << DWT_FUNCTION_MATCHED_POS;
/// DWT FUNCTION: DATAVADDR1 position
pub const DWT_FUNCTION_DATAVADDR1_POS: usize = 16;
/// DWT FUNCTION: DATAVADDR1 mask
pub const DWT_FUNCTION_DATAVADDR1_MSK: usize = 0xF << DWT_FUNCTION_DATAVADDR1_POS;
/// DWT FUNCTION: DATAVADDR0 position
pub const DWT_FUNCTION_DATAVADDR0_POS: usize = 12;
/// DWT FUNCTION: DATAVADDR0 mask
pub const DWT_FUNCTION_DATAVADDR0_MSK: usize = 0xF << DWT_FUNCTION_DATAVADDR0_POS;
/// DWT FUNCTION: DATAVSIZE position
pub const DWT_FUNCTION_DATAVSIZE_POS: usize = 10;
/// DWT FUNCTION: DATAVSIZE mask
pub const DWT_FUNCTION_DATAVSIZE_MSK: usize = 0x3 << DWT_FUNCTION_DATAVSIZE_POS;
/// DWT FUNCTION: LNK1ENA position
pub const DWT_FUNCTION_LNK1ENA_POS: usize = 9;
/// DWT FUNCTION: LNK1ENA mask
pub const DWT_FUNCTION_LNK1ENA_MSK: usize = 0x1 << DWT_FUNCTION_LNK1ENA_POS;
/// DWT FUNCTION: DATAVMATCH position
pub const DWT_FUNCTION_DATAVMATCH_POS: usize = 8;
/// DWT FUNCTION: DATAVMATCH mask
pub const DWT_FUNCTION_DATAVMATCH_MSK: usize = 0x1 << DWT_FUNCTION_DATAVMATCH_POS;
/// DWT FUNCTION: CYCMATCH position
pub const DWT_FUNCTION_CYCMATCH_POS: usize = 7;
/// DWT FUNCTION: CYCMATCH mask
pub const DWT_FUNCTION_CYCMATCH_MSK: usize = 0x1 << DWT_FUNCTION_CYCMATCH_POS;
/// DWT FUNCTION: EMITRANGE position
pub const DWT_FUNCTION_EMITRANGE_POS: usize = 5;
/// DWT FUNCTION: EMITRANGE mask
pub const DWT_FUNCTION_EMITRANGE_MSK: usize = 0x1 << DWT_FUNCTION_EMITRANGE_POS;
/// DWT FUNCTION: FUNCTION position
pub const DWT_FUNCTION_FUNCTION_POS: usize = 0;
/// DWT FUNCTION: FUNCTION mask
pub const DWT_FUNCTION_FUNCTION_MSK: usize = 0xF << DWT_FUNCTION_FUNCTION_POS;

// ---------------------------------------------------------------------------
// TPI — Trace Port Interface
// ---------------------------------------------------------------------------

define_reg! {
    /// Trace Port Interface (TPI) register block.
    Tpi
    __Tpi {
        /// Supported Parallel Port Size Register (offset 0x000, read-only)
        sspsr: VolatileRO<usize>,
        /// Current Parallel Port Size Register (offset 0x004)
        cspsr: VolatileRW<usize>,
        _reserved0: [usize; 2],
        /// Asynchronous Clock Prescaler Register (offset 0x010)
        acpr: VolatileRW<usize>,
        _reserved1: [usize; 55],
        /// Selected Pin Protocol Register (offset 0x0F0)
        sppr: VolatileRW<usize>,
        _reserved2: [usize; 131],
        /// Formatter and Flush Status Register (offset 0x300, read-only)
        ffsr: VolatileRO<usize>,
        /// Formatter and Flush Control Register (offset 0x304)
        ffcr: VolatileRW<usize>,
        /// Formatter Synchronization Counter Register (offset 0x308, read-only)
        fscr: VolatileRO<usize>,
        _reserved3: [usize; 759],
        /// TRIGGER Register (offset 0xEE8, read-only)
        trigger: VolatileRO<usize>,
        /// Integration ETM Data (offset 0xEEC, read-only)
        fifo0: VolatileRO<usize>,
        /// ITATBCTR2 (offset 0xEF0, read-only)
        itatbctr2: VolatileRO<usize>,
        _reserved4: [usize; 1],
        /// ITATBCTR0 (offset 0xEF8, read-only)
        itatbctr0: VolatileRO<usize>,
        /// Integration ITM Data (offset 0xEFC, read-only)
        fifo1: VolatileRO<usize>,
        /// Integration Mode Control (offset 0xF00)
        itctrl: VolatileRW<usize>,
        _reserved5: [usize; 39],
        /// Claim tag set (offset 0xFA0)
        claimset: VolatileRW<usize>,
        /// Claim tag clear (offset 0xFA4)
        claimclr: VolatileRW<usize>,
        _reserved7: [usize; 8],
        /// TPIU_DEVID (offset 0xFC8, read-only)
        devid: VolatileRO<usize>,
        /// TPIU_DEVTYPE (offset 0xFCC, read-only)
        devtype: VolatileRO<usize>,
    }
}

/// TPI ACPR: PRESCALER position
pub const TPI_ACPR_PRESCALER_POS: usize = 0;
/// TPI ACPR: PRESCALER mask
pub const TPI_ACPR_PRESCALER_MSK: usize = 0x1FFF << TPI_ACPR_PRESCALER_POS;
/// TPI SPPR: TXMODE position
pub const TPI_SPPR_TXMODE_POS: usize = 0;
/// TPI SPPR: TXMODE mask
pub const TPI_SPPR_TXMODE_MSK: usize = 0x3 << TPI_SPPR_TXMODE_POS;
/// TPI FFSR: FtNonStop position
pub const TPI_FFSR_FT_NON_STOP_POS: usize = 3;
/// TPI FFSR: FtNonStop mask
pub const TPI_FFSR_FT_NON_STOP_MSK: usize = 0x1 << TPI_FFSR_FT_NON_STOP_POS;
/// TPI FFSR: TCPresent position
pub const TPI_FFSR_TC_PRESENT_POS: usize = 2;
/// TPI FFSR: TCPresent mask
pub const TPI_FFSR_TC_PRESENT_MSK: usize = 0x1 << TPI_FFSR_TC_PRESENT_POS;
/// TPI FFSR: FtStopped position
pub const TPI_FFSR_FT_STOPPED_POS: usize = 1;
/// TPI FFSR: FtStopped mask
pub const TPI_FFSR_FT_STOPPED_MSK: usize = 0x1 << TPI_FFSR_FT_STOPPED_POS;
/// TPI FFSR: FlInProg position
pub const TPI_FFSR_FL_IN_PROG_POS: usize = 0;
/// TPI FFSR: FlInProg mask
pub const TPI_FFSR_FL_IN_PROG_MSK: usize = 0x1 << TPI_FFSR_FL_IN_PROG_POS;
/// TPI FFCR: TrigIn position
pub const TPI_FFCR_TRIG_IN_POS: usize = 8;
/// TPI FFCR: TrigIn mask
pub const TPI_FFCR_TRIG_IN_MSK: usize = 0x1 << TPI_FFCR_TRIG_IN_POS;
/// TPI FFCR: EnFCont position
pub const TPI_FFCR_EN_FCONT_POS: usize = 1;
/// TPI FFCR: EnFCont mask
pub const TPI_FFCR_EN_FCONT_MSK: usize = 0x1 << TPI_FFCR_EN_FCONT_POS;
/// TPI TRIGGER: TRIGGER position
pub const TPI_TRIGGER_TRIGGER_POS: usize = 0;
/// TPI TRIGGER: TRIGGER mask
pub const TPI_TRIGGER_TRIGGER_MSK: usize = 0x1 << TPI_TRIGGER_TRIGGER_POS;
/// TPI ITCTRL: Mode position
pub const TPI_ITCTRL_MODE_POS: usize = 0;
/// TPI ITCTRL: Mode mask
pub const TPI_ITCTRL_MODE_MSK: usize = 0x1 << TPI_ITCTRL_MODE_POS;
/// TPI DEVID: NRZVALID position
pub const TPI_DEVID_NRZVALID_POS: usize = 11;
/// TPI DEVID: NRZVALID mask
pub const TPI_DEVID_NRZVALID_MSK: usize = 0x1 << TPI_DEVID_NRZVALID_POS;
/// TPI DEVID: MANCVALID position
pub const TPI_DEVID_MANCVALID_POS: usize = 10;
/// TPI DEVID: MANCVALID mask
pub const TPI_DEVID_MANCVALID_MSK: usize = 0x1 << TPI_DEVID_MANCVALID_POS;
/// TPI DEVID: PTINVALID position
pub const TPI_DEVID_PTINVALID_POS: usize = 9;
/// TPI DEVID: PTINVALID mask
pub const TPI_DEVID_PTINVALID_MSK: usize = 0x1 << TPI_DEVID_PTINVALID_POS;
/// TPI DEVID: MinBufSz position
pub const TPI_DEVID_MIN_BUF_SZ_POS: usize = 6;
/// TPI DEVID: MinBufSz mask
pub const TPI_DEVID_MIN_BUF_SZ_MSK: usize = 0x7 << TPI_DEVID_MIN_BUF_SZ_POS;
/// TPI DEVID: AsynClkIn position
pub const TPI_DEVID_ASYN_CLK_IN_POS: usize = 5;
/// TPI DEVID: AsynClkIn mask
pub const TPI_DEVID_ASYN_CLK_IN_MSK: usize = 0x1 << TPI_DEVID_ASYN_CLK_IN_POS;
/// TPI DEVID: NrTraceInput position
pub const TPI_DEVID_NR_TRACE_INPUT_POS: usize = 0;
/// TPI DEVID: NrTraceInput mask
pub const TPI_DEVID_NR_TRACE_INPUT_MSK: usize = 0x1F << TPI_DEVID_NR_TRACE_INPUT_POS;
/// TPI DEVTYPE: SubType position
pub const TPI_DEVTYPE_SUB_TYPE_POS: usize = 0;
/// TPI DEVTYPE: SubType mask
pub const TPI_DEVTYPE_SUB_TYPE_MSK: usize = 0xF << TPI_DEVTYPE_SUB_TYPE_POS;
/// TPI DEVTYPE: MajorType position
pub const TPI_DEVTYPE_MAJOR_TYPE_POS: usize = 4;
/// TPI DEVTYPE: MajorType mask
pub const TPI_DEVTYPE_MAJOR_TYPE_MSK: usize = 0xF << TPI_DEVTYPE_MAJOR_TYPE_POS;

// ---------------------------------------------------------------------------
// MPU — Memory Protection Unit
// ---------------------------------------------------------------------------

define_reg! {
    /// Memory Protection Unit (MPU) register block.
    Mpu
    __Mpu {
        /// MPU Type Register (offset 0x000, read-only)
        type_: VolatileRO<usize>,
        /// MPU Control Register (offset 0x004)
        ctrl: VolatileRW<usize>,
        /// MPU Region Number Register (offset 0x008)
        rnr: VolatileRW<usize>,
        /// MPU Region Base Address Register (offset 0x00C)
        rbar: VolatileRW<usize>,
        /// MPU Region Attribute and Size Register (offset 0x010)
        rasr: VolatileRW<usize>,
        /// MPU Alias 1 Region Base Address Register (offset 0x014)
        rbar_a1: VolatileRW<usize>,
        /// MPU Alias 1 Region Attribute and Size Register (offset 0x018)
        rasr_a1: VolatileRW<usize>,
        /// MPU Alias 2 Region Base Address Register (offset 0x01C)
        rbar_a2: VolatileRW<usize>,
        /// MPU Alias 2 Region Attribute and Size Register (offset 0x020)
        rasr_a2: VolatileRW<usize>,
        /// MPU Alias 3 Region Base Address Register (offset 0x024)
        rbar_a3: VolatileRW<usize>,
        /// MPU Alias 3 Region Attribute and Size Register (offset 0x028)
        rasr_a3: VolatileRW<usize>,
    }
}

/// MPU TYPE: IREGION position
pub const MPU_TYPE_IREGION_POS: usize = 16;
/// MPU TYPE: IREGION mask
pub const MPU_TYPE_IREGION_MSK: usize = 0xFF << MPU_TYPE_IREGION_POS;
/// MPU TYPE: DREGION position
pub const MPU_TYPE_DREGION_POS: usize = 8;
/// MPU TYPE: DREGION mask
pub const MPU_TYPE_DREGION_MSK: usize = 0xFF << MPU_TYPE_DREGION_POS;
/// MPU TYPE: SEPARATE position
pub const MPU_TYPE_SEPARATE_POS: usize = 0;
/// MPU TYPE: SEPARATE mask
pub const MPU_TYPE_SEPARATE_MSK: usize = 1 << MPU_TYPE_SEPARATE_POS;
/// MPU CTRL: PRIVDEFENA position
pub const MPU_CTRL_PRIVDEFENA_POS: usize = 2;
/// MPU CTRL: PRIVDEFENA mask
pub const MPU_CTRL_PRIVDEFENA_MSK: usize = 1 << MPU_CTRL_PRIVDEFENA_POS;
/// MPU CTRL: HFNMIENA position
pub const MPU_CTRL_HFNMIENA_POS: usize = 1;
/// MPU CTRL: HFNMIENA mask
pub const MPU_CTRL_HFNMIENA_MSK: usize = 1 << MPU_CTRL_HFNMIENA_POS;
/// MPU CTRL: ENABLE position
pub const MPU_CTRL_ENABLE_POS: usize = 0;
/// MPU CTRL: ENABLE mask
pub const MPU_CTRL_ENABLE_MSK: usize = 1 << MPU_CTRL_ENABLE_POS;
/// MPU RNR: REGION position
pub const MPU_RNR_REGION_POS: usize = 0;
/// MPU RNR: REGION mask
pub const MPU_RNR_REGION_MSK: usize = 0xFF << MPU_RNR_REGION_POS;
/// MPU RBAR: ADDR position
pub const MPU_RBAR_ADDR_POS: usize = 5;
/// MPU RBAR: ADDR mask
pub const MPU_RBAR_ADDR_MSK: usize = 0x7FFFFFF << MPU_RBAR_ADDR_POS;
/// MPU RBAR: VALID position
pub const MPU_RBAR_VALID_POS: usize = 4;
/// MPU RBAR: VALID mask
pub const MPU_RBAR_VALID_MSK: usize = 1 << MPU_RBAR_VALID_POS;
/// MPU RBAR: REGION position
pub const MPU_RBAR_REGION_POS: usize = 0;
/// MPU RBAR: REGION mask
pub const MPU_RBAR_REGION_MSK: usize = 0xF << MPU_RBAR_REGION_POS;
/// MPU RASR: ATTRS position
pub const MPU_RASR_ATTRS_POS: usize = 16;
/// MPU RASR: ATTRS mask
pub const MPU_RASR_ATTRS_MSK: usize = 0xFFFF << MPU_RASR_ATTRS_POS;
/// MPU RASR: XN position
pub const MPU_RASR_XN_POS: usize = 28;
/// MPU RASR: XN mask
pub const MPU_RASR_XN_MSK: usize = 1 << MPU_RASR_XN_POS;
/// MPU RASR: AP position
pub const MPU_RASR_AP_POS: usize = 24;
/// MPU RASR: AP mask
pub const MPU_RASR_AP_MSK: usize = 0x7 << MPU_RASR_AP_POS;
/// MPU RASR: TEX position
pub const MPU_RASR_TEX_POS: usize = 19;
/// MPU RASR: TEX mask
pub const MPU_RASR_TEX_MSK: usize = 0x7 << MPU_RASR_TEX_POS;
/// MPU RASR: S position
pub const MPU_RASR_S_POS: usize = 18;
/// MPU RASR: S mask
pub const MPU_RASR_S_MSK: usize = 1 << MPU_RASR_S_POS;
/// MPU RASR: C position
pub const MPU_RASR_C_POS: usize = 17;
/// MPU RASR: C mask
pub const MPU_RASR_C_MSK: usize = 1 << MPU_RASR_C_POS;
/// MPU RASR: B position
pub const MPU_RASR_B_POS: usize = 16;
/// MPU RASR: B mask
pub const MPU_RASR_B_MSK: usize = 1 << MPU_RASR_B_POS;
/// MPU RASR: Sub-Region Disable position
pub const MPU_RASR_SRD_POS: usize = 8;
/// MPU RASR: Sub-Region Disable mask
pub const MPU_RASR_SRD_MSK: usize = 0xFF << MPU_RASR_SRD_POS;
/// MPU RASR: Region Size Field position
pub const MPU_RASR_SIZE_POS: usize = 1;
/// MPU RASR: Region Size Field mask
pub const MPU_RASR_SIZE_MSK: usize = 0x1F << MPU_RASR_SIZE_POS;
/// MPU RASR: Region enable bit position
pub const MPU_RASR_ENABLE_POS: usize = 0;
/// MPU RASR: Region enable bit mask
pub const MPU_RASR_ENABLE_MSK: usize = 1 << MPU_RASR_ENABLE_POS;

// ---------------------------------------------------------------------------
// FPU — Floating Point Unit
// ---------------------------------------------------------------------------

define_reg! {
    /// Floating Point Unit (FPU) register block.
    Fpu
    __Fpu {
        _reserved0: [usize; 1],
        /// Floating-Point Context Control Register (offset 0x004)
        fpccr: VolatileRW<usize>,
        /// Floating-Point Context Address Register (offset 0x008)
        fpcar: VolatileRW<usize>,
        /// Floating-Point Default Status Control Register (offset 0x00C)
        fpdscr: VolatileRW<usize>,
        /// Media and FP Feature Register 0 (offset 0x010, read-only)
        mvfr0: VolatileRO<usize>,
        /// Media and FP Feature Register 1 (offset 0x014, read-only)
        mvfr1: VolatileRO<usize>,
    }
}

/// FPCCR: ASPEN position
pub const FPU_FPCCR_ASPEN_POS: usize = 31;
/// FPCCR: ASPEN mask
pub const FPU_FPCCR_ASPEN_MSK: usize = 1 << FPU_FPCCR_ASPEN_POS;
/// FPCCR: LSPEN position
pub const FPU_FPCCR_LSPEN_POS: usize = 30;
/// FPCCR: LSPEN mask
pub const FPU_FPCCR_LSPEN_MSK: usize = 1 << FPU_FPCCR_LSPEN_POS;
/// FPCCR: MONRDY position
pub const FPU_FPCCR_MONRDY_POS: usize = 8;
/// FPCCR: MONRDY mask
pub const FPU_FPCCR_MONRDY_MSK: usize = 1 << FPU_FPCCR_MONRDY_POS;
/// FPCCR: BFRDY position
pub const FPU_FPCCR_BFRDY_POS: usize = 6;
/// FPCCR: BFRDY mask
pub const FPU_FPCCR_BFRDY_MSK: usize = 1 << FPU_FPCCR_BFRDY_POS;
/// FPCCR: MMRDY position
pub const FPU_FPCCR_MMRDY_POS: usize = 5;
/// FPCCR: MMRDY mask
pub const FPU_FPCCR_MMRDY_MSK: usize = 1 << FPU_FPCCR_MMRDY_POS;
/// FPCCR: HFRDY position
pub const FPU_FPCCR_HFRDY_POS: usize = 4;
/// FPCCR: HFRDY mask
pub const FPU_FPCCR_HFRDY_MSK: usize = 1 << FPU_FPCCR_HFRDY_POS;
/// FPCCR: processor mode bit position
pub const FPU_FPCCR_THREAD_POS: usize = 3;
/// FPCCR: processor mode active bit mask
pub const FPU_FPCCR_THREAD_MSK: usize = 1 << FPU_FPCCR_THREAD_POS;
/// FPCCR: privilege level bit position
pub const FPU_FPCCR_USER_POS: usize = 1;
/// FPCCR: privilege level bit mask
pub const FPU_FPCCR_USER_MSK: usize = 1 << FPU_FPCCR_USER_POS;
/// FPCCR: Lazy state preservation active bit position
pub const FPU_FPCCR_LSPACT_POS: usize = 0;
/// FPCCR: Lazy state preservation active bit mask
pub const FPU_FPCCR_LSPACT_MSK: usize = 1 << FPU_FPCCR_LSPACT_POS;
/// FPCAR: ADDRESS position
pub const FPU_FPCAR_ADDRESS_POS: usize = 3;
/// FPCAR: ADDRESS mask
pub const FPU_FPCAR_ADDRESS_MSK: usize = 0x1FFFFFFF << FPU_FPCAR_ADDRESS_POS;
/// FPDSCR: AHP position
pub const FPU_FPDSCR_AHP_POS: usize = 26;
/// FPDSCR: AHP mask
pub const FPU_FPDSCR_AHP_MSK: usize = 1 << FPU_FPDSCR_AHP_POS;
/// FPDSCR: DN position
pub const FPU_FPDSCR_DN_POS: usize = 25;
/// FPDSCR: DN mask
pub const FPU_FPDSCR_DN_MSK: usize = 1 << FPU_FPDSCR_DN_POS;
/// FPDSCR: FZ position
pub const FPU_FPDSCR_FZ_POS: usize = 24;
/// FPDSCR: FZ mask
pub const FPU_FPDSCR_FZ_MSK: usize = 1 << FPU_FPDSCR_FZ_POS;
/// FPDSCR: RMode position
pub const FPU_FPDSCR_RMODE_POS: usize = 22;
/// FPDSCR: RMode mask
pub const FPU_FPDSCR_RMODE_MSK: usize = 3 << FPU_FPDSCR_RMODE_POS;
/// MVFR0: FP rounding modes position
pub const FPU_MVFR0_FP_ROUNDING_MODES_POS: usize = 28;
/// MVFR0: FP rounding modes mask
pub const FPU_MVFR0_FP_ROUNDING_MODES_MSK: usize = 0xF << FPU_MVFR0_FP_ROUNDING_MODES_POS;
/// MVFR0: Short vectors position
pub const FPU_MVFR0_SHORT_VECTORS_POS: usize = 24;
/// MVFR0: Short vectors mask
pub const FPU_MVFR0_SHORT_VECTORS_MSK: usize = 0xF << FPU_MVFR0_SHORT_VECTORS_POS;
/// MVFR0: Square root position
pub const FPU_MVFR0_SQUARE_ROOT_POS: usize = 20;
/// MVFR0: Square root mask
pub const FPU_MVFR0_SQUARE_ROOT_MSK: usize = 0xF << FPU_MVFR0_SQUARE_ROOT_POS;
/// MVFR0: Divide position
pub const FPU_MVFR0_DIVIDE_POS: usize = 16;
/// MVFR0: Divide mask
pub const FPU_MVFR0_DIVIDE_MSK: usize = 0xF << FPU_MVFR0_DIVIDE_POS;
/// MVFR0: FP exception trapping position
pub const FPU_MVFR0_FP_EXCEP_TRAPPING_POS: usize = 12;
/// MVFR0: FP exception trapping mask
pub const FPU_MVFR0_FP_EXCEP_TRAPPING_MSK: usize = 0xF << FPU_MVFR0_FP_EXCEP_TRAPPING_POS;
/// MVFR0: Double-precision position
pub const FPU_MVFR0_DOUBLE_PRECISION_POS: usize = 8;
/// MVFR0: Double-precision mask
pub const FPU_MVFR0_DOUBLE_PRECISION_MSK: usize = 0xF << FPU_MVFR0_DOUBLE_PRECISION_POS;
/// MVFR0: Single-precision position
pub const FPU_MVFR0_SINGLE_PRECISION_POS: usize = 4;
/// MVFR0: Single-precision mask
pub const FPU_MVFR0_SINGLE_PRECISION_MSK: usize = 0xF << FPU_MVFR0_SINGLE_PRECISION_POS;
/// MVFR0: A_SIMD registers position
pub const FPU_MVFR0_A_SIMD_REGISTERS_POS: usize = 0;
/// MVFR0: A_SIMD registers mask
pub const FPU_MVFR0_A_SIMD_REGISTERS_MSK: usize = 0xF << FPU_MVFR0_A_SIMD_REGISTERS_POS;
/// MVFR1: FP fused MAC position
pub const FPU_MVFR1_FP_FUSED_MAC_POS: usize = 28;
/// MVFR1: FP fused MAC mask
pub const FPU_MVFR1_FP_FUSED_MAC_MSK: usize = 0xF << FPU_MVFR1_FP_FUSED_MAC_POS;
/// MVFR1: FP HPFP position
pub const FPU_MVFR1_FP_HPFP_POS: usize = 24;
/// MVFR1: FP HPFP mask
pub const FPU_MVFR1_FP_HPFP_MSK: usize = 0xF << FPU_MVFR1_FP_HPFP_POS;
/// MVFR1: D_NaN mode position
pub const FPU_MVFR1_D_NAN_MODE_POS: usize = 4;
/// MVFR1: D_NaN mode mask
pub const FPU_MVFR1_D_NAN_MODE_MSK: usize = 0xF << FPU_MVFR1_D_NAN_MODE_POS;
/// MVFR1: FtZ mode position
pub const FPU_MVFR1_FTZ_MODE_POS: usize = 0;
/// MVFR1: FtZ mode mask
pub const FPU_MVFR1_FTZ_MODE_MSK: usize = 0xF << FPU_MVFR1_FTZ_MODE_POS;

// ---------------------------------------------------------------------------
// CoreDebug — Core Debug Registers
// ---------------------------------------------------------------------------

define_reg! {
    /// Core Debug Register (CoreDebug) block.
    CoreDebug
    __CoreDebug {
        /// Debug Halting Control and Status Register (offset 0x000)
        dhcsr: VolatileRW<usize>,
        /// Debug Core Register Selector Register (offset 0x004, write-only)
        dcrsr: VolatileRW<usize>,
        /// Debug Core Register Data Register (offset 0x008)
        dcrdr: VolatileRW<usize>,
        /// Debug Exception and Monitor Control Register (offset 0x00C)
        demcr: VolatileRW<usize>,
    }
}

/// CoreDebug DHCSR: DBGKEY position
pub const CORE_DEBUG_DHCSR_DBGKEY_POS: usize = 16;
/// CoreDebug DHCSR: DBGKEY mask
pub const CORE_DEBUG_DHCSR_DBGKEY_MSK: usize = 0xFFFF << CORE_DEBUG_DHCSR_DBGKEY_POS;
/// CoreDebug DHCSR: S_RESET_ST position
pub const CORE_DEBUG_DHCSR_S_RESET_ST_POS: usize = 25;
/// CoreDebug DHCSR: S_RESET_ST mask
pub const CORE_DEBUG_DHCSR_S_RESET_ST_MSK: usize = 1 << CORE_DEBUG_DHCSR_S_RESET_ST_POS;
/// CoreDebug DHCSR: S_RETIRE_ST position
pub const CORE_DEBUG_DHCSR_S_RETIRE_ST_POS: usize = 24;
/// CoreDebug DHCSR: S_RETIRE_ST mask
pub const CORE_DEBUG_DHCSR_S_RETIRE_ST_MSK: usize = 1 << CORE_DEBUG_DHCSR_S_RETIRE_ST_POS;
/// CoreDebug DHCSR: S_LOCKUP position
pub const CORE_DEBUG_DHCSR_S_LOCKUP_POS: usize = 19;
/// CoreDebug DHCSR: S_LOCKUP mask
pub const CORE_DEBUG_DHCSR_S_LOCKUP_MSK: usize = 1 << CORE_DEBUG_DHCSR_S_LOCKUP_POS;
/// CoreDebug DHCSR: S_SLEEP position
pub const CORE_DEBUG_DHCSR_S_SLEEP_POS: usize = 18;
/// CoreDebug DHCSR: S_SLEEP mask
pub const CORE_DEBUG_DHCSR_S_SLEEP_MSK: usize = 1 << CORE_DEBUG_DHCSR_S_SLEEP_POS;
/// CoreDebug DHCSR: S_HALT position
pub const CORE_DEBUG_DHCSR_S_HALT_POS: usize = 17;
/// CoreDebug DHCSR: S_HALT mask
pub const CORE_DEBUG_DHCSR_S_HALT_MSK: usize = 1 << CORE_DEBUG_DHCSR_S_HALT_POS;
/// CoreDebug DHCSR: S_REGRDY position
pub const CORE_DEBUG_DHCSR_S_REGRDY_POS: usize = 16;
/// CoreDebug DHCSR: S_REGRDY mask
pub const CORE_DEBUG_DHCSR_S_REGRDY_MSK: usize = 1 << CORE_DEBUG_DHCSR_S_REGRDY_POS;
/// CoreDebug DHCSR: C_SNAPSTALL position
pub const CORE_DEBUG_DHCSR_C_SNAPSTALL_POS: usize = 5;
/// CoreDebug DHCSR: C_SNAPSTALL mask
pub const CORE_DEBUG_DHCSR_C_SNAPSTALL_MSK: usize = 1 << CORE_DEBUG_DHCSR_C_SNAPSTALL_POS;
/// CoreDebug DHCSR: C_MASKINTS position
pub const CORE_DEBUG_DHCSR_C_MASKINTS_POS: usize = 3;
/// CoreDebug DHCSR: C_MASKINTS mask
pub const CORE_DEBUG_DHCSR_C_MASKINTS_MSK: usize = 1 << CORE_DEBUG_DHCSR_C_MASKINTS_POS;
/// CoreDebug DHCSR: C_STEP position
pub const CORE_DEBUG_DHCSR_C_STEP_POS: usize = 2;
/// CoreDebug DHCSR: C_STEP mask
pub const CORE_DEBUG_DHCSR_C_STEP_MSK: usize = 1 << CORE_DEBUG_DHCSR_C_STEP_POS;
/// CoreDebug DHCSR: C_HALT position
pub const CORE_DEBUG_DHCSR_C_HALT_POS: usize = 1;
/// CoreDebug DHCSR: C_HALT mask
pub const CORE_DEBUG_DHCSR_C_HALT_MSK: usize = 1 << CORE_DEBUG_DHCSR_C_HALT_POS;
/// CoreDebug DHCSR: C_DEBUGEN position
pub const CORE_DEBUG_DHCSR_C_DEBUGEN_POS: usize = 0;
/// CoreDebug DHCSR: C_DEBUGEN mask
pub const CORE_DEBUG_DHCSR_C_DEBUGEN_MSK: usize = 1 << CORE_DEBUG_DHCSR_C_DEBUGEN_POS;
/// CoreDebug DCRSR: REGWnR position
pub const CORE_DEBUG_DCRSR_REGWNR_POS: usize = 16;
/// CoreDebug DCRSR: REGWnR mask
pub const CORE_DEBUG_DCRSR_REGWNR_MSK: usize = 1 << CORE_DEBUG_DCRSR_REGWNR_POS;
/// CoreDebug DCRSR: REGSEL position
pub const CORE_DEBUG_DCRSR_REGSEL_POS: usize = 0;
/// CoreDebug DCRSR: REGSEL mask
pub const CORE_DEBUG_DCRSR_REGSEL_MSK: usize = 0x1F << CORE_DEBUG_DCRSR_REGSEL_POS;
/// CoreDebug DEMCR: TRCENA position
pub const CORE_DEBUG_DEMCR_TRCENA_POS: usize = 24;
/// CoreDebug DEMCR: TRCENA mask
pub const CORE_DEBUG_DEMCR_TRCENA_MSK: usize = 1 << CORE_DEBUG_DEMCR_TRCENA_POS;
/// CoreDebug DEMCR: MON_REQ position
pub const CORE_DEBUG_DEMCR_MON_REQ_POS: usize = 19;
/// CoreDebug DEMCR: MON_REQ mask
pub const CORE_DEBUG_DEMCR_MON_REQ_MSK: usize = 1 << CORE_DEBUG_DEMCR_MON_REQ_POS;
/// CoreDebug DEMCR: MON_STEP position
pub const CORE_DEBUG_DEMCR_MON_STEP_POS: usize = 18;
/// CoreDebug DEMCR: MON_STEP mask
pub const CORE_DEBUG_DEMCR_MON_STEP_MSK: usize = 1 << CORE_DEBUG_DEMCR_MON_STEP_POS;
/// CoreDebug DEMCR: MON_PEND position
pub const CORE_DEBUG_DEMCR_MON_PEND_POS: usize = 17;
/// CoreDebug DEMCR: MON_PEND mask
pub const CORE_DEBUG_DEMCR_MON_PEND_MSK: usize = 1 << CORE_DEBUG_DEMCR_MON_PEND_POS;
/// CoreDebug DEMCR: MON_EN position
pub const CORE_DEBUG_DEMCR_MON_EN_POS: usize = 16;
/// CoreDebug DEMCR: MON_EN mask
pub const CORE_DEBUG_DEMCR_MON_EN_MSK: usize = 1 << CORE_DEBUG_DEMCR_MON_EN_POS;
/// CoreDebug DEMCR: VC_HARDERR position
pub const CORE_DEBUG_DEMCR_VC_HARDERR_POS: usize = 10;
/// CoreDebug DEMCR: VC_HARDERR mask
pub const CORE_DEBUG_DEMCR_VC_HARDERR_MSK: usize = 1 << CORE_DEBUG_DEMCR_VC_HARDERR_POS;
/// CoreDebug DEMCR: VC_INTERR position
pub const CORE_DEBUG_DEMCR_VC_INTERR_POS: usize = 9;
/// CoreDebug DEMCR: VC_INTERR mask
pub const CORE_DEBUG_DEMCR_VC_INTERR_MSK: usize = 1 << CORE_DEBUG_DEMCR_VC_INTERR_POS;
/// CoreDebug DEMCR: VC_BUSERR position
pub const CORE_DEBUG_DEMCR_VC_BUSERR_POS: usize = 8;
/// CoreDebug DEMCR: VC_BUSERR mask
pub const CORE_DEBUG_DEMCR_VC_BUSERR_MSK: usize = 1 << CORE_DEBUG_DEMCR_VC_BUSERR_POS;
/// CoreDebug DEMCR: VC_STATERR position
pub const CORE_DEBUG_DEMCR_VC_STATERR_POS: usize = 7;
/// CoreDebug DEMCR: VC_STATERR mask
pub const CORE_DEBUG_DEMCR_VC_STATERR_MSK: usize = 1 << CORE_DEBUG_DEMCR_VC_STATERR_POS;
/// CoreDebug DEMCR: VC_CHKERR position
pub const CORE_DEBUG_DEMCR_VC_CHKERR_POS: usize = 6;
/// CoreDebug DEMCR: VC_CHKERR mask
pub const CORE_DEBUG_DEMCR_VC_CHKERR_MSK: usize = 1 << CORE_DEBUG_DEMCR_VC_CHKERR_POS;
/// CoreDebug DEMCR: VC_NOCPERR position
pub const CORE_DEBUG_DEMCR_VC_NOCPERR_POS: usize = 5;
/// CoreDebug DEMCR: VC_NOCPERR mask
pub const CORE_DEBUG_DEMCR_VC_NOCPERR_MSK: usize = 1 << CORE_DEBUG_DEMCR_VC_NOCPERR_POS;
/// CoreDebug DEMCR: VC_MMERR position
pub const CORE_DEBUG_DEMCR_VC_MMERR_POS: usize = 4;
/// CoreDebug DEMCR: VC_MMERR mask
pub const CORE_DEBUG_DEMCR_VC_MMERR_MSK: usize = 1 << CORE_DEBUG_DEMCR_VC_MMERR_POS;
/// CoreDebug DEMCR: VC_CORERESET position
pub const CORE_DEBUG_DEMCR_VC_CORERESET_POS: usize = 0;
/// CoreDebug DEMCR: VC_CORERESET mask
pub const CORE_DEBUG_DEMCR_VC_CORERESET_MSK: usize = 1 << CORE_DEBUG_DEMCR_VC_CORERESET_POS;

// ---------------------------------------------------------------------------
// Volatile register access wrappers
// ---------------------------------------------------------------------------

/// A read-write volatile register.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VolatileRW<T: Copy> {
    value: T,
}

impl<T: Copy> VolatileRW<T> {
    /// Read the register value.
    #[inline]
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.value) }
    }

    /// Write a value to the register.
    #[inline]
    #[allow(invalid_reference_casting)]
    pub fn write(&self, val: T) {
        unsafe { ptr::write_volatile(&self.value as *const T as *mut T, val) }
    }
}

/// A read-only volatile register.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VolatileRO<T: Copy> {
    value: T,
}

impl<T: Copy> VolatileRO<T> {
    /// Read the register value.
    #[inline]
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.value) }
    }
}

// ---------------------------------------------------------------------------
// Memory-mapped peripheral base addresses
// ---------------------------------------------------------------------------

/// System Control Space base address.
pub const SCS_BASE: usize = 0xE000_E000;
/// ITM base address.
pub const ITM_BASE: usize = 0xE000_0000;
/// DWT base address.
pub const DWT_BASE: usize = 0xE000_1000;
/// TPI base address.
pub const TPI_BASE: usize = 0xE004_0000;
/// Core Debug base address.
pub const CORE_DEBUG_BASE: usize = 0xE000_EDF0;
/// SysTick base address.
pub const SYSTICK_BASE: usize = SCS_BASE + 0x0010;
/// NVIC base address.
pub const NVIC_BASE: usize = SCS_BASE + 0x0100;
/// SCB base address.
pub const SCB_BASE: usize = SCS_BASE + 0x0D00;
/// MPU base address.
pub const MPU_BASE: usize = SCS_BASE + 0x0D90;
/// FPU base address.
pub const FPU_BASE: usize = SCS_BASE + 0x0F30;

/// Get a reference to the SCnSCB register block.
pub static SCNSCB: ScnScb = ScnScb::new(SCS_BASE);

/// Get a reference to the SCB register block.
pub static SCB: Scb = Scb::new(SCB_BASE);

/// Get a reference to the SysTick register block.
pub static SYSTICK: SysTick = SysTick::new(SYSTICK_BASE);

/// Get a reference to the NVIC register block.
pub static NVIC: Nvic = Nvic::new(NVIC_BASE);

/// Get a reference to the ITM register block.
pub static ITM: Itm = Itm::new(ITM_BASE);

/// Get a reference to the DWT register block.
pub static DWT: Dwt = Dwt::new(DWT_BASE);

/// Get a reference to the TPI register block.
pub static TPI: Tpi = Tpi::new(TPI_BASE);

/// Get a reference to the CoreDebug register block.
pub static CORE_DEBUG: CoreDebug = CoreDebug::new(CORE_DEBUG_BASE);

/// Get a reference to the MPU register block.
pub static MPU: Mpu = Mpu::new(MPU_BASE);

/// Get a reference to the FPU register block.
pub static FPU: Fpu = Fpu::new(FPU_BASE);

// ---------------------------------------------------------------------------
// IRQ number type (device-specific values should be provided elsewhere)
// ---------------------------------------------------------------------------

/// Interrupt number type. Negative values represent system exceptions.
#[repr(i32)]
#[derive(Clone, Copy)]
pub enum IRQType {
    /// 2 Non Maskable Interrupt
    NonMaskableInterrupt = -14,
    /// 4 Cortex-M3 Memory Management Interrupt
    MemoryManagement = -12,
    /// 5 Cortex-M3 Bus Fault Interrupt
    BusFault = -11,
    /// 6 Cortex-M3 Usage Fault Interrupt
    UsageFault = -10,
    /// 11 Cortex-M3 SV Call Interrupt
    SVCall = -5,
    /// 12 Cortex-M3 Debug Monitor Interrupt
    DebugMonitor = -4,
    /// 14 Cortex-M3 Pend SV Interrupt
    PendSV = -2,
    /// 15 Cortex-M3 System Tick Interrupt
    SysTick = -1,
    /// SEC Interrupt
    Sec = 0,
    /// RTC Interrupt
    Rtc = 1,
    /// WDG Interrupt
    Wdg = 2,
    /// EFC Interrupt
    Efc = 3,
    /// UART3 Interrupt
    Uart3 = 4,
    /// I2C2 Interrupt
    I2c2 = 5,
    /// UART0 Interrupt
    Uart0 = 6,
    /// UART1 Interrupt
    Uart1 = 7,
    /// UART2 Interrupt
    Uart2 = 8,
    /// LPUART Interrupt
    Lpuart = 9,
    /// Ssp0 Interrupt       
    Ssp0 = 10,
    /// Ssp1 Interrupt       
    Ssp1 = 11,
    /// QSPI Interrupt       
    Qspi = 12,
    /// I2C0 Interrupt       
    I2c0 = 13,
    /// I2C1 Interrupt       
    I2c1 = 14,
    /// SCC Interrupt        
    Scc = 15,
    /// ADC Interrupt        
    Adc = 16,
    /// AFEC Interrupt       
    Afec = 17,
    /// Ssp2 Interrupt       
    Ssp2 = 18,
    /// DMA1 Interrupt       
    Dma1 = 19,
    /// DAC Interrupt        
    Dac = 20,
    /// LORA Interrupt       
    Lora = 21,
    /// GPIO Interrupt       
    Gpio = 22,
    /// TIMER0 Interrupt     
    Timer0 = 23,
    /// TIMER1 Interrupt     
    Timer1 = 24,
    /// TIMER2 Interrupt     
    Timer2 = 25,
    /// TIMER3 Interrupt     
    Timer3 = 26,
    /// BSTIMER0 Interrupt   
    Bstimer0 = 27,
    /// BSTIMER1 Interrupt   
    Bstimer1 = 28,
    /// LPTIMER0 Interrupt   
    Lptimer0 = 29,
    /// SAC Interrupt        
    Sac = 30,
    /// DMA0 Interrupt       
    Dma0 = 31,
    /// I2S Interrupt        
    I2s = 32,
    /// LCD Interrupt        
    Lcd = 33,
    /// PWR Interrupt        
    Pwr = 34,
    /// LPTIMER1 Interrupt   
    Lptimer1 = 35,
    /// IWDG Interrupt       
    Iwdg = 36,
}

impl IRQType {
    #[inline]
    pub fn from_i32(n: i32) -> Self {
        match n {
            -14 => IRQType::NonMaskableInterrupt,
            -12 => IRQType::MemoryManagement,
            -11 => IRQType::BusFault,
            -10 => IRQType::UsageFault,
            -5 => IRQType::SVCall,
            -4 => IRQType::DebugMonitor,
            -2 => IRQType::PendSV,
            -1 => IRQType::SysTick,
            0 => IRQType::Sec,
            1 => IRQType::Rtc,
            2 => IRQType::Wdg,
            3 => IRQType::Efc,
            4 => IRQType::Uart3,
            5 => IRQType::I2c2,
            6 => IRQType::Uart0,
            7 => IRQType::Uart1,
            8 => IRQType::Uart2,
            9 => IRQType::Lpuart,
            10 => IRQType::Ssp0,
            11 => IRQType::Ssp1,
            12 => IRQType::Qspi,
            13 => IRQType::I2c0,
            14 => IRQType::I2c1,
            15 => IRQType::Scc,
            16 => IRQType::Adc,
            17 => IRQType::Afec,
            18 => IRQType::Ssp2,
            19 => IRQType::Dma1,
            20 => IRQType::Dac,
            21 => IRQType::Lora,
            22 => IRQType::Gpio,
            23 => IRQType::Timer0,
            24 => IRQType::Timer1,
            25 => IRQType::Timer2,
            26 => IRQType::Timer3,
            27 => IRQType::Bstimer0,
            28 => IRQType::Bstimer1,
            29 => IRQType::Lptimer0,
            30 => IRQType::Sac,
            31 => IRQType::Dma0,
            32 => IRQType::I2s,
            33 => IRQType::Lcd,
            34 => IRQType::Pwr,
            35 => IRQType::Lptimer1,
            36 => IRQType::Iwdg,
            _ => panic!("Invalid interrupt number: {}", n),
        }
    }
}

// ---------------------------------------------------------------------------
// NVIC Functions
// ---------------------------------------------------------------------------

/// Set the priority grouping field in `SCB->AIRCR`.
///
/// Only values 0..7 are used. In case of a conflict between priority grouping
/// and available priority bits ([`NVIC_PRIO_BITS`]), the smallest possible
/// priority group is set.
#[inline]
pub fn nvic_set_priority_grouping(priority_group: usize) {
    let priority_group_tmp = priority_group & 0x07;

    let mut reg_value = SCB.aircr.read();
    reg_value &= !(SCB_AIRCR_VECTKEY_MSK | SCB_AIRCR_PRIGROUP_MSK);
    reg_value |= (0x5FA << SCB_AIRCR_VECTKEY_POS) | (priority_group_tmp << 8);
    SCB.aircr.write(reg_value);
}

/// Get the priority grouping field from `SCB->AIRCR`.
///
/// Returns the priority grouping field (bits \[10:8\]).
#[inline]
pub fn nvic_get_priority_grouping() -> usize {
    (SCB.aircr.read() & SCB_AIRCR_PRIGROUP_MSK) >> SCB_AIRCR_PRIGROUP_POS
}

/// Enable an external interrupt in the NVIC.
///
/// `irqn` must be non-negative (device-specific interrupt).
#[inline]
pub fn nvic_enable_irq(irqn: IRQType) {
    let n = irqn as usize;
    NVIC.iser[n >> 5].write(1 << (n & 0x1F));
}

/// Disable an external interrupt in the NVIC.
///
/// `irqn` must be non-negative (device-specific interrupt).
#[inline]
pub fn nvic_disable_irq(irqn: IRQType) {
    let n = irqn as usize;
    NVIC.icer[n >> 5].write(1 << (n & 0x1F));
}

/// Get the pending status of an interrupt.
///
/// Returns `true` if the interrupt is pending.
#[inline]
pub fn nvic_get_pending_irq(irqn: IRQType) -> bool {
    let n = irqn as usize;
    (NVIC.ispr[n >> 5].read() & (1 << (n & 0x1F))) != 0
}

/// Set an interrupt to pending.
#[inline]
pub fn nvic_set_pending_irq(irqn: IRQType) {
    let n = irqn as usize;
    NVIC.ispr[n >> 5].write(1 << (n & 0x1F));
}

/// Clear the pending status of an interrupt.
#[inline]
pub fn nvic_clear_pending_irq(irqn: IRQType) {
    let n = irqn as usize;
    NVIC.icpr[n >> 5].write(1 << (n & 0x1F));
}

/// Get the active status of an interrupt.
///
/// Returns `true` if the interrupt is active.
#[inline]
pub fn nvic_get_active(irqn: IRQType) -> bool {
    let n = irqn as usize;
    (NVIC.iabr[n >> 5].read() & (1 << (n & 0x1F))) != 0
}

/// Set the priority of an interrupt.
///
/// Negative `irqn` values address system handler priorities in `SCB->SHP`.
/// Non-negative values address device-specific interrupts via `NVIC->IP`.
#[inline]
pub fn nvic_set_priority(irqn: IRQType, priority: usize) {
    let encoded = ((priority << (8 - NVIC_PRIO_BITS)) & 0xFF) as u8;
    if (irqn as i8) < 0 {
        let idx = ((irqn as usize) & 0xF).wrapping_sub(4);
        SCB.shp[idx].write(encoded);
    } else {
        NVIC.ip[irqn as usize].write(encoded);
    }
}

/// Get the priority of an interrupt.
///
/// Returns the priority value, right-shifted to the implemented bits.
#[inline]
pub fn nvic_get_priority(irqn: IRQType) -> usize {
    if (irqn as i8) < 0 {
        let idx = ((irqn as usize) & 0xF).wrapping_sub(4);
        (SCB.shp[idx].read() as usize) >> (8 - NVIC_PRIO_BITS)
    } else {
        (NVIC.ip[irqn as usize].read() as usize) >> (8 - NVIC_PRIO_BITS)
    }
}

/// Encode a priority value from group, preempt, and sub-priority.
///
/// Returns the encoded priority suitable for [`nvic_set_priority`].
#[inline]
pub fn nvic_encode_priority(
    priority_group: usize,
    preempt_priority: usize,
    sub_priority: usize,
) -> usize {
    let pg = priority_group & 0x07;
    let preempt_bits = if (7 - pg) > NVIC_PRIO_BITS {
        NVIC_PRIO_BITS
    } else {
        7 - pg
    };
    let sub_bits = (pg + NVIC_PRIO_BITS).saturating_sub(7);

    ((preempt_priority & ((1 << preempt_bits) - 1)) << sub_bits)
        | (sub_priority & ((1 << sub_bits) - 1))
}

/// Decode a priority value into preempt and sub-priority.
///
/// Returns `(preempt_priority, sub_priority)`.
#[inline]
pub fn nvic_decode_priority(priority: usize, priority_group: usize) -> (usize, usize) {
    let pg = priority_group & 0x07;
    let preempt_bits = if (7 - pg) > NVIC_PRIO_BITS {
        NVIC_PRIO_BITS
    } else {
        7 - pg
    };
    let sub_bits = (pg + NVIC_PRIO_BITS).saturating_sub(7);

    let preempt = (priority >> sub_bits) & ((1 << preempt_bits) - 1);
    let sub = priority & ((1 << sub_bits) - 1);
    (preempt, sub)
}

/// Initiate a system reset request.
///
/// This function does not return. It issues a DSB, writes the reset
/// request to `SCB->AIRCR`, then waits forever.
///
/// # Safety
/// Accesses memory-mapped hardware registers and resets the system.
#[inline]
pub fn nvic_system_reset() -> ! {
    asm::_dsb();
    let val = (0x5FA << SCB_AIRCR_VECTKEY_POS)
        | (SCB.aircr.read() & SCB_AIRCR_PRIGROUP_MSK)
        | SCB_AIRCR_SYSRESETREQ_MSK;
    SCB.aircr.write(val);
    asm::_dsb();
    loop {
        asm::_nop();
    }
}

/// Reload value too large [`systick_config`].
#[derive(Debug)]
pub struct ReloadValueTooLarge;

// ---------------------------------------------------------------------------
// SysTick Functions
// ---------------------------------------------------------------------------

/// Configure and start the SysTick timer.
///
/// Sets up the SysTick timer with the given number of `ticks` between
/// interrupts. Returns `Ok(())` on success or `Err(())` if the reload
/// value is too large.
#[inline]
pub fn systick_config(ticks: usize) -> Result<(), ReloadValueTooLarge> {
    if (ticks - 1) > SYSTICK_LOAD_RELOAD_MSK {
        return Err(ReloadValueTooLarge);
    }

    SYSTICK.load.write(ticks - 1);
    nvic_set_priority(IRQType::SysTick, (1 << NVIC_PRIO_BITS) - 1);
    SYSTICK.val.write(0);
    SYSTICK
        .ctrl
        .write(SYSTICK_CTRL_CLKSOURCE_MSK | SYSTICK_CTRL_TICKINT_MSK | SYSTICK_CTRL_ENABLE_MSK);
    Ok(())
}

// ---------------------------------------------------------------------------
// ITM Debug Functions
// ---------------------------------------------------------------------------

/// Value indicating the ITM receive buffer is empty and ready for the next character.
pub const ITM_RXBUFFER_EMPTY: i32 = 0x5AA5_5AA5usize as i32;

/// Transmit a character via ITM stimulus port 0.
///
/// Returns the character. If no debugger is connected the function returns
/// immediately. If a debugger is connected it blocks until the port is ready.
#[inline]
pub fn itm_send_char(ch: usize) -> usize {
    if (ITM.tcr.read() & ITM_TCR_ITMENA_MSK) != 0 && (ITM.ter.read() & 1) != 0 {
        while unsafe { ITM.port[0].usize.read() } == 0 {}
        unsafe { ITM.port[0].u8.write(ch as u8) };
    }
    ch
}
