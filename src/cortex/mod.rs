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
pub const CM4_CMSIS_VERSION_MAIN: u32 = 0x03;
/// CMSIS HAL sub version
pub const CM4_CMSIS_VERSION_SUB: u32 = 0x20;
/// CMSIS HAL version number
pub const CM4_CMSIS_VERSION: u32 = (CM4_CMSIS_VERSION_MAIN << 16) | CM4_CMSIS_VERSION_SUB;
/// Cortex-M Core identifier
pub const CORTEX_M: u32 = 0x04;

// ---------------------------------------------------------------------------
// Configuration — adjust these to match your device header
// ---------------------------------------------------------------------------

/// Number of NVIC priority bits implemented by this device.
pub const NVIC_PRIO_BITS: u32 = 3;

/// Whether an FPU is present (1) or not (0).
pub const FPU_PRESENT: u32 = 1;

/// Whether an MPU is present (1) or not (0).
pub const MPU_PRESENT: u32 = 0;

pub const LIBRARY_NORMAL_INTERRUPT_PRIORITY: u32 = 6;

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
    pub w: u32,
}

/// Union type to access the Interrupt Program Status Register (IPSR).
#[repr(C)]
pub union IpsrType {
    /// Word access
    pub w: u32,
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
    pub w: u32,
}

/// Union type to access the Control Register (CONTROL).
#[repr(C)]
pub union ControlType {
    /// Word access
    pub w: u32,
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
        iser: [VolatileRW<u32>; 8],
        _reserved0: [u32; 24],
        /// Interrupt Clear Enable Registers (offset 0x080)
        icer: [VolatileRW<u32>; 8],
        _reserved1: [u32; 24],
        /// Interrupt Set Pending Registers (offset 0x100)
        ispr: [VolatileRW<u32>; 8],
        _reserved2: [u32; 24],
        /// Interrupt Clear Pending Registers (offset 0x180)
        icpr: [VolatileRW<u32>; 8],
        _reserved3: [u32; 24],
        /// Interrupt Active Bit Registers (offset 0x200)
        iabr: [VolatileRW<u32>; 8],
        _reserved4: [u32; 56],
        /// Interrupt Priority Registers, 8-bit wide (offset 0x300)
        ip: [VolatileRW<u8>; 240],
        _reserved5: [u32; 644],
        /// Software Trigger Interrupt Register (offset 0xE00, write-only)
        stir: VolatileWO<u32>,
    }
}

/// STIR: INTID position
pub const NVIC_STIR_INTID_POS: u32 = 0;
/// STIR: INTID mask
pub const NVIC_STIR_INTID_MSK: u32 = 0x1FF << NVIC_STIR_INTID_POS;

// ---------------------------------------------------------------------------
// SCB — System Control Block
// ---------------------------------------------------------------------------

define_reg! {
    /// System Control Block (SCB) register block.
    Scb
    __Scb {
        /// CPUID Base Register (offset 0x000, read-only)
        cpuid: VolatileRO<u32>,
        /// Interrupt Control and State Register (offset 0x004)
        icsr: VolatileRW<u32>,
        /// Vector Table Offset Register (offset 0x008)
        vtor: VolatileRW<u32>,
        /// Application Interrupt and Reset Control Register (offset 0x00C)
        aircr: VolatileRW<u32>,
        /// System Control Register (offset 0x010)
        scr: VolatileRW<u32>,
        /// Configuration Control Register (offset 0x014)
        ccr: VolatileRW<u32>,
        /// System Handlers Priority Registers (offset 0x018)
        shp: [VolatileRW<u8>; 12],
        /// System Handler Control and State Register (offset 0x024)
        shcsr: VolatileRW<u32>,
        /// Configurable Fault Status Register (offset 0x028)
        cfsr: VolatileRW<u32>,
        /// HardFault Status Register (offset 0x02C)
        hfsr: VolatileRW<u32>,
        /// Debug Fault Status Register (offset 0x030)
        dfsr: VolatileRW<u32>,
        /// MemManage Fault Address Register (offset 0x034)
        mmfar: VolatileRW<u32>,
        /// BusFault Address Register (offset 0x038)
        bfar: VolatileRW<u32>,
        /// Auxiliary Fault Status Register (offset 0x03C)
        afsr: VolatileRW<u32>,
        /// Processor Feature Registers (offset 0x040, read-only)
        pfr: [VolatileRO<u32>; 2],
        /// Debug Feature Register (offset 0x048, read-only)
        dfr: VolatileRO<u32>,
        /// Auxiliary Feature Register (offset 0x04C, read-only)
        adr: VolatileRO<u32>,
        /// Memory Model Feature Registers (offset 0x050, read-only)
        mmfr: [VolatileRO<u32>; 4],
        /// Instruction Set Attributes Registers (offset 0x060, read-only)
        isar: [VolatileRO<u32>; 5],
        _reserved0: [u32; 5],
        /// Coprocessor Access Control Register (offset 0x088)
        cpacr: VolatileRW<u32>,
    }
}

// SCB CPUID Register
/// SCB CPUID: IMPLEMENTER position
pub const SCB_CPUID_IMPLEMENTER_POS: u32 = 24;
/// SCB CPUID: IMPLEMENTER mask
pub const SCB_CPUID_IMPLEMENTER_MSK: u32 = 0xFF << SCB_CPUID_IMPLEMENTER_POS;
/// SCB CPUID: VARIANT position
pub const SCB_CPUID_VARIANT_POS: u32 = 20;
/// SCB CPUID: VARIANT mask
pub const SCB_CPUID_VARIANT_MSK: u32 = 0xF << SCB_CPUID_VARIANT_POS;
/// SCB CPUID: ARCHITECTURE position
pub const SCB_CPUID_ARCHITECTURE_POS: u32 = 16;
/// SCB CPUID: ARCHITECTURE mask
pub const SCB_CPUID_ARCHITECTURE_MSK: u32 = 0xF << SCB_CPUID_ARCHITECTURE_POS;
/// SCB CPUID: PARTNO position
pub const SCB_CPUID_PARTNO_POS: u32 = 4;
/// SCB CPUID: PARTNO mask
pub const SCB_CPUID_PARTNO_MSK: u32 = 0xFFF << SCB_CPUID_PARTNO_POS;
/// SCB CPUID: REVISION position
pub const SCB_CPUID_REVISION_POS: u32 = 0;
/// SCB CPUID: REVISION mask
pub const SCB_CPUID_REVISION_MSK: u32 = 0xF << SCB_CPUID_REVISION_POS;

// SCB Interrupt Control State Register
/// SCB ICSR: NMIPENDSET position
pub const SCB_ICSR_NMIPENDSET_POS: u32 = 31;
/// SCB ICSR: NMIPENDSET mask
pub const SCB_ICSR_NMIPENDSET_MSK: u32 = 1 << SCB_ICSR_NMIPENDSET_POS;
/// SCB ICSR: PENDSVSET position
pub const SCB_ICSR_PENDSVSET_POS: u32 = 28;
/// SCB ICSR: PENDSVSET mask
pub const SCB_ICSR_PENDSVSET_MSK: u32 = 1 << SCB_ICSR_PENDSVSET_POS;
/// SCB ICSR: PENDSVCLR position
pub const SCB_ICSR_PENDSVCLR_POS: u32 = 27;
/// SCB ICSR: PENDSVCLR mask
pub const SCB_ICSR_PENDSVCLR_MSK: u32 = 1 << SCB_ICSR_PENDSVCLR_POS;
/// SCB ICSR: PENDSTSET position
pub const SCB_ICSR_PENDSTSET_POS: u32 = 26;
/// SCB ICSR: PENDSTSET mask
pub const SCB_ICSR_PENDSTSET_MSK: u32 = 1 << SCB_ICSR_PENDSTSET_POS;
/// SCB ICSR: PENDSTCLR position
pub const SCB_ICSR_PENDSTCLR_POS: u32 = 25;
/// SCB ICSR: PENDSTCLR mask
pub const SCB_ICSR_PENDSTCLR_MSK: u32 = 1 << SCB_ICSR_PENDSTCLR_POS;
/// SCB ICSR: ISRPREEMPT position
pub const SCB_ICSR_ISRPREEMPT_POS: u32 = 23;
/// SCB ICSR: ISRPREEMPT mask
pub const SCB_ICSR_ISRPREEMPT_MSK: u32 = 1 << SCB_ICSR_ISRPREEMPT_POS;
/// SCB ICSR: ISRPENDING position
pub const SCB_ICSR_ISRPENDING_POS: u32 = 22;
/// SCB ICSR: ISRPENDING mask
pub const SCB_ICSR_ISRPENDING_MSK: u32 = 1 << SCB_ICSR_ISRPENDING_POS;
/// SCB ICSR: VECTPENDING position
pub const SCB_ICSR_VECTPENDING_POS: u32 = 12;
/// SCB ICSR: VECTPENDING mask
pub const SCB_ICSR_VECTPENDING_MSK: u32 = 0x1FF << SCB_ICSR_VECTPENDING_POS;
/// SCB ICSR: RETTOBASE position
pub const SCB_ICSR_RETTOBASE_POS: u32 = 11;
/// SCB ICSR: RETTOBASE mask
pub const SCB_ICSR_RETTOBASE_MSK: u32 = 1 << SCB_ICSR_RETTOBASE_POS;
/// SCB ICSR: VECTACTIVE position
pub const SCB_ICSR_VECTACTIVE_POS: u32 = 0;
/// SCB ICSR: VECTACTIVE mask
pub const SCB_ICSR_VECTACTIVE_MSK: u32 = 0x1FF << SCB_ICSR_VECTACTIVE_POS;

// SCB Vector Table Offset Register
/// SCB VTOR: TBLOFF position
pub const SCB_VTOR_TBLOFF_POS: u32 = 7;
/// SCB VTOR: TBLOFF mask
pub const SCB_VTOR_TBLOFF_MSK: u32 = 0x1FFFFFF << SCB_VTOR_TBLOFF_POS;

// SCB Application Interrupt and Reset Control Register
/// SCB AIRCR: VECTKEY position
pub const SCB_AIRCR_VECTKEY_POS: u32 = 16;
/// SCB AIRCR: VECTKEY mask
pub const SCB_AIRCR_VECTKEY_MSK: u32 = 0xFFFF << SCB_AIRCR_VECTKEY_POS;
/// SCB AIRCR: VECTKEYSTAT position
pub const SCB_AIRCR_VECTKEYSTAT_POS: u32 = 16;
/// SCB AIRCR: VECTKEYSTAT mask
pub const SCB_AIRCR_VECTKEYSTAT_MSK: u32 = 0xFFFF << SCB_AIRCR_VECTKEYSTAT_POS;
/// SCB AIRCR: ENDIANESS position
pub const SCB_AIRCR_ENDIANESS_POS: u32 = 15;
/// SCB AIRCR: ENDIANESS mask
pub const SCB_AIRCR_ENDIANESS_MSK: u32 = 1 << SCB_AIRCR_ENDIANESS_POS;
/// SCB AIRCR: PRIGROUP position
pub const SCB_AIRCR_PRIGROUP_POS: u32 = 8;
/// SCB AIRCR: PRIGROUP mask
pub const SCB_AIRCR_PRIGROUP_MSK: u32 = 7 << SCB_AIRCR_PRIGROUP_POS;
/// SCB AIRCR: SYSRESETREQ position
pub const SCB_AIRCR_SYSRESETREQ_POS: u32 = 2;
/// SCB AIRCR: SYSRESETREQ mask
pub const SCB_AIRCR_SYSRESETREQ_MSK: u32 = 1 << SCB_AIRCR_SYSRESETREQ_POS;
/// SCB AIRCR: VECTCLRACTIVE position
pub const SCB_AIRCR_VECTCLRACTIVE_POS: u32 = 1;
/// SCB AIRCR: VECTCLRACTIVE mask
pub const SCB_AIRCR_VECTCLRACTIVE_MSK: u32 = 1 << SCB_AIRCR_VECTCLRACTIVE_POS;
/// SCB AIRCR: VECTRESET position
pub const SCB_AIRCR_VECTRESET_POS: u32 = 0;
/// SCB AIRCR: VECTRESET mask
pub const SCB_AIRCR_VECTRESET_MSK: u32 = 1 << SCB_AIRCR_VECTRESET_POS;

// SCB System Control Register
/// SCB SCR: SEVONPEND position
pub const SCB_SCR_SEVONPEND_POS: u32 = 4;
/// SCB SCR: SEVONPEND mask
pub const SCB_SCR_SEVONPEND_MSK: u32 = 1 << SCB_SCR_SEVONPEND_POS;
/// SCB SCR: SLEEPDEEP position
pub const SCB_SCR_SLEEPDEEP_POS: u32 = 2;
/// SCB SCR: SLEEPDEEP mask
pub const SCB_SCR_SLEEPDEEP_MSK: u32 = 1 << SCB_SCR_SLEEPDEEP_POS;
/// SCB SCR: SLEEPONEXIT position
pub const SCB_SCR_SLEEPONEXIT_POS: u32 = 1;
/// SCB SCR: SLEEPONEXIT mask
pub const SCB_SCR_SLEEPONEXIT_MSK: u32 = 1 << SCB_SCR_SLEEPONEXIT_POS;

// SCB Configuration Control Register
/// SCB CCR: STKALIGN position
pub const SCB_CCR_STKALIGN_POS: u32 = 9;
/// SCB CCR: STKALIGN mask
pub const SCB_CCR_STKALIGN_MSK: u32 = 1 << SCB_CCR_STKALIGN_POS;
/// SCB CCR: BFHFNMIGN position
pub const SCB_CCR_BFHFNMIGN_POS: u32 = 8;
/// SCB CCR: BFHFNMIGN mask
pub const SCB_CCR_BFHFNMIGN_MSK: u32 = 1 << SCB_CCR_BFHFNMIGN_POS;
/// SCB CCR: DIV_0_TRP position
pub const SCB_CCR_DIV_0_TRP_POS: u32 = 4;
/// SCB CCR: DIV_0_TRP mask
pub const SCB_CCR_DIV_0_TRP_MSK: u32 = 1 << SCB_CCR_DIV_0_TRP_POS;
/// SCB CCR: UNALIGN_TRP position
pub const SCB_CCR_UNALIGN_TRP_POS: u32 = 3;
/// SCB CCR: UNALIGN_TRP mask
pub const SCB_CCR_UNALIGN_TRP_MSK: u32 = 1 << SCB_CCR_UNALIGN_TRP_POS;
/// SCB CCR: USERSETMPEND position
pub const SCB_CCR_USERSETMPEND_POS: u32 = 1;
/// SCB CCR: USERSETMPEND mask
pub const SCB_CCR_USERSETMPEND_MSK: u32 = 1 << SCB_CCR_USERSETMPEND_POS;
/// SCB CCR: NONBASETHRDENA position
pub const SCB_CCR_NONBASETHRDENA_POS: u32 = 0;
/// SCB CCR: NONBASETHRDENA mask
pub const SCB_CCR_NONBASETHRDENA_MSK: u32 = 1 << SCB_CCR_NONBASETHRDENA_POS;

// SCB System Handler Control and State Register
/// SCB SHCSR: USGFAULTENA position
pub const SCB_SHCSR_USGFAULTENA_POS: u32 = 18;
/// SCB SHCSR: USGFAULTENA mask
pub const SCB_SHCSR_USGFAULTENA_MSK: u32 = 1 << SCB_SHCSR_USGFAULTENA_POS;
/// SCB SHCSR: BUSFAULTENA position
pub const SCB_SHCSR_BUSFAULTENA_POS: u32 = 17;
/// SCB SHCSR: BUSFAULTENA mask
pub const SCB_SHCSR_BUSFAULTENA_MSK: u32 = 1 << SCB_SHCSR_BUSFAULTENA_POS;
/// SCB SHCSR: MEMFAULTENA position
pub const SCB_SHCSR_MEMFAULTENA_POS: u32 = 16;
/// SCB SHCSR: MEMFAULTENA mask
pub const SCB_SHCSR_MEMFAULTENA_MSK: u32 = 1 << SCB_SHCSR_MEMFAULTENA_POS;
/// SCB SHCSR: SVCALLPENDED position
pub const SCB_SHCSR_SVCALLPENDED_POS: u32 = 15;
/// SCB SHCSR: SVCALLPENDED mask
pub const SCB_SHCSR_SVCALLPENDED_MSK: u32 = 1 << SCB_SHCSR_SVCALLPENDED_POS;
/// SCB SHCSR: BUSFAULTPENDED position
pub const SCB_SHCSR_BUSFAULTPENDED_POS: u32 = 14;
/// SCB SHCSR: BUSFAULTPENDED mask
pub const SCB_SHCSR_BUSFAULTPENDED_MSK: u32 = 1 << SCB_SHCSR_BUSFAULTPENDED_POS;
/// SCB SHCSR: MEMFAULTPENDED position
pub const SCB_SHCSR_MEMFAULTPENDED_POS: u32 = 13;
/// SCB SHCSR: MEMFAULTPENDED mask
pub const SCB_SHCSR_MEMFAULTPENDED_MSK: u32 = 1 << SCB_SHCSR_MEMFAULTPENDED_POS;
/// SCB SHCSR: USGFAULTPENDED position
pub const SCB_SHCSR_USGFAULTPENDED_POS: u32 = 12;
/// SCB SHCSR: USGFAULTPENDED mask
pub const SCB_SHCSR_USGFAULTPENDED_MSK: u32 = 1 << SCB_SHCSR_USGFAULTPENDED_POS;
/// SCB SHCSR: SYSTICKACT position
pub const SCB_SHCSR_SYSTICKACT_POS: u32 = 11;
/// SCB SHCSR: SYSTICKACT mask
pub const SCB_SHCSR_SYSTICKACT_MSK: u32 = 1 << SCB_SHCSR_SYSTICKACT_POS;
/// SCB SHCSR: PENDSVACT position
pub const SCB_SHCSR_PENDSVACT_POS: u32 = 10;
/// SCB SHCSR: PENDSVACT mask
pub const SCB_SHCSR_PENDSVACT_MSK: u32 = 1 << SCB_SHCSR_PENDSVACT_POS;
/// SCB SHCSR: MONITORACT position
pub const SCB_SHCSR_MONITORACT_POS: u32 = 8;
/// SCB SHCSR: MONITORACT mask
pub const SCB_SHCSR_MONITORACT_MSK: u32 = 1 << SCB_SHCSR_MONITORACT_POS;
/// SCB SHCSR: SVCALLACT position
pub const SCB_SHCSR_SVCALLACT_POS: u32 = 7;
/// SCB SHCSR: SVCALLACT mask
pub const SCB_SHCSR_SVCALLACT_MSK: u32 = 1 << SCB_SHCSR_SVCALLACT_POS;
/// SCB SHCSR: USGFAULTACT position
pub const SCB_SHCSR_USGFAULTACT_POS: u32 = 3;
/// SCB SHCSR: USGFAULTACT mask
pub const SCB_SHCSR_USGFAULTACT_MSK: u32 = 1 << SCB_SHCSR_USGFAULTACT_POS;
/// SCB SHCSR: BUSFAULTACT position
pub const SCB_SHCSR_BUSFAULTACT_POS: u32 = 1;
/// SCB SHCSR: BUSFAULTACT mask
pub const SCB_SHCSR_BUSFAULTACT_MSK: u32 = 1 << SCB_SHCSR_BUSFAULTACT_POS;
/// SCB SHCSR: MEMFAULTACT position
pub const SCB_SHCSR_MEMFAULTACT_POS: u32 = 0;
/// SCB SHCSR: MEMFAULTACT mask
pub const SCB_SHCSR_MEMFAULTACT_MSK: u32 = 1 << SCB_SHCSR_MEMFAULTACT_POS;

// SCB Configurable Fault Status Register
/// SCB CFSR: Usage Fault Status Register position
pub const SCB_CFSR_USGFAULTSR_POS: u32 = 16;
/// SCB CFSR: Usage Fault Status Register mask
pub const SCB_CFSR_USGFAULTSR_MSK: u32 = 0xFFFF << SCB_CFSR_USGFAULTSR_POS;
/// SCB CFSR: Bus Fault Status Register position
pub const SCB_CFSR_BUSFAULTSR_POS: u32 = 8;
/// SCB CFSR: Bus Fault Status Register mask
pub const SCB_CFSR_BUSFAULTSR_MSK: u32 = 0xFF << SCB_CFSR_BUSFAULTSR_POS;
/// SCB CFSR: Memory Manage Fault Status Register position
pub const SCB_CFSR_MEMFAULTSR_POS: u32 = 0;
/// SCB CFSR: Memory Manage Fault Status Register mask
pub const SCB_CFSR_MEMFAULTSR_MSK: u32 = 0xFF << SCB_CFSR_MEMFAULTSR_POS;

// SCB Hard Fault Status Register
/// SCB HFSR: DEBUGEVT position
pub const SCB_HFSR_DEBUGEVT_POS: u32 = 31;
/// SCB HFSR: DEBUGEVT mask
pub const SCB_HFSR_DEBUGEVT_MSK: u32 = 1 << SCB_HFSR_DEBUGEVT_POS;
/// SCB HFSR: FORCED position
pub const SCB_HFSR_FORCED_POS: u32 = 30;
/// SCB HFSR: FORCED mask
pub const SCB_HFSR_FORCED_MSK: u32 = 1 << SCB_HFSR_FORCED_POS;
/// SCB HFSR: VECTTBL position
pub const SCB_HFSR_VECTTBL_POS: u32 = 1;
/// SCB HFSR: VECTTBL mask
pub const SCB_HFSR_VECTTBL_MSK: u32 = 1 << SCB_HFSR_VECTTBL_POS;

// SCB Debug Fault Status Register
/// SCB DFSR: EXTERNAL position
pub const SCB_DFSR_EXTERNAL_POS: u32 = 4;
/// SCB DFSR: EXTERNAL mask
pub const SCB_DFSR_EXTERNAL_MSK: u32 = 1 << SCB_DFSR_EXTERNAL_POS;
/// SCB DFSR: VCATCH position
pub const SCB_DFSR_VCATCH_POS: u32 = 3;
/// SCB DFSR: VCATCH mask
pub const SCB_DFSR_VCATCH_MSK: u32 = 1 << SCB_DFSR_VCATCH_POS;
/// SCB DFSR: DWTTRAP position
pub const SCB_DFSR_DWTTRAP_POS: u32 = 2;
/// SCB DFSR: DWTTRAP mask
pub const SCB_DFSR_DWTTRAP_MSK: u32 = 1 << SCB_DFSR_DWTTRAP_POS;
/// SCB DFSR: BKPT position
pub const SCB_DFSR_BKPT_POS: u32 = 1;
/// SCB DFSR: BKPT mask
pub const SCB_DFSR_BKPT_MSK: u32 = 1 << SCB_DFSR_BKPT_POS;
/// SCB DFSR: HALTED position
pub const SCB_DFSR_HALTED_POS: u32 = 0;
/// SCB DFSR: HALTED mask
pub const SCB_DFSR_HALTED_MSK: u32 = 1 << SCB_DFSR_HALTED_POS;

// ---------------------------------------------------------------------------
// SCnSCB — System Controls not in SCB
// ---------------------------------------------------------------------------

define_reg! {
    /// System Control and ID Register not in the SCB.
    ScnScb
    __ScnScb {
        _reserved0: [u32; 1],
        /// Interrupt Controller Type Register (offset 0x004, read-only)
        ictr: VolatileRO<u32>,
        /// Auxiliary Control Register (offset 0x008)
        actlr: VolatileRW<u32>,
    }
}

/// ICTR: INTLINESNUM position
pub const SCNSCB_ICTR_INTLINESNUM_POS: u32 = 0;
/// ICTR: INTLINESNUM mask
pub const SCNSCB_ICTR_INTLINESNUM_MSK: u32 = 0xF << SCNSCB_ICTR_INTLINESNUM_POS;

/// ACTLR: DISOOFP position
pub const SCNSCB_ACTLR_DISOOFP_POS: u32 = 9;
/// ACTLR: DISOOFP mask
pub const SCNSCB_ACTLR_DISOOFP_MSK: u32 = 1 << SCNSCB_ACTLR_DISOOFP_POS;
/// ACTLR: DISFPCA position
pub const SCNSCB_ACTLR_DISFPCA_POS: u32 = 8;
/// ACTLR: DISFPCA mask
pub const SCNSCB_ACTLR_DISFPCA_MSK: u32 = 1 << SCNSCB_ACTLR_DISFPCA_POS;
/// ACTLR: DISFOLD position
pub const SCNSCB_ACTLR_DISFOLD_POS: u32 = 2;
/// ACTLR: DISFOLD mask
pub const SCNSCB_ACTLR_DISFOLD_MSK: u32 = 1 << SCNSCB_ACTLR_DISFOLD_POS;
/// ACTLR: DISDEFWBUF position
pub const SCNSCB_ACTLR_DISDEFWBUF_POS: u32 = 1;
/// ACTLR: DISDEFWBUF mask
pub const SCNSCB_ACTLR_DISDEFWBUF_MSK: u32 = 1 << SCNSCB_ACTLR_DISDEFWBUF_POS;
/// ACTLR: DISMCYCINT position
pub const SCNSCB_ACTLR_DISMCYCINT_POS: u32 = 0;
/// ACTLR: DISMCYCINT mask
pub const SCNSCB_ACTLR_DISMCYCINT_MSK: u32 = 1 << SCNSCB_ACTLR_DISMCYCINT_POS;

// ---------------------------------------------------------------------------
// SysTick — System Timer
// ---------------------------------------------------------------------------

define_reg! {
    /// System Timer (SysTick) register block.
    SysTick
    __SysTick {
        /// SysTick Control and Status Register (offset 0x000)
        ctrl: VolatileRW<u32>,
        /// SysTick Reload Value Register (offset 0x004)
        load: VolatileRW<u32>,
        /// SysTick Current Value Register (offset 0x008)
        val: VolatileRW<u32>,
        /// SysTick Calibration Register (offset 0x00C, read-only)
        calib: VolatileRO<u32>,
    }
}

/// SysTick CTRL: COUNTFLAG position
pub const SYSTICK_CTRL_COUNTFLAG_POS: u32 = 16;
/// SysTick CTRL: COUNTFLAG mask
pub const SYSTICK_CTRL_COUNTFLAG_MSK: u32 = 1 << SYSTICK_CTRL_COUNTFLAG_POS;
/// SysTick CTRL: CLKSOURCE position
pub const SYSTICK_CTRL_CLKSOURCE_POS: u32 = 2;
/// SysTick CTRL: CLKSOURCE mask
pub const SYSTICK_CTRL_CLKSOURCE_MSK: u32 = 1 << SYSTICK_CTRL_CLKSOURCE_POS;
/// SysTick CTRL: TICKINT position
pub const SYSTICK_CTRL_TICKINT_POS: u32 = 1;
/// SysTick CTRL: TICKINT mask
pub const SYSTICK_CTRL_TICKINT_MSK: u32 = 1 << SYSTICK_CTRL_TICKINT_POS;
/// SysTick CTRL: ENABLE position
pub const SYSTICK_CTRL_ENABLE_POS: u32 = 0;
/// SysTick CTRL: ENABLE mask
pub const SYSTICK_CTRL_ENABLE_MSK: u32 = 1 << SYSTICK_CTRL_ENABLE_POS;
/// SysTick LOAD: RELOAD position
pub const SYSTICK_LOAD_RELOAD_POS: u32 = 0;
/// SysTick LOAD: RELOAD mask
pub const SYSTICK_LOAD_RELOAD_MSK: u32 = 0xFFFFFF << SYSTICK_LOAD_RELOAD_POS;
/// SysTick VAL: CURRENT position
pub const SYSTICK_VAL_CURRENT_POS: u32 = 0;
/// SysTick VAL: CURRENT mask
pub const SYSTICK_VAL_CURRENT_MSK: u32 = 0xFFFFFF << SYSTICK_VAL_CURRENT_POS;
/// SysTick CALIB: NOREF position
pub const SYSTICK_CALIB_NOREF_POS: u32 = 31;
/// SysTick CALIB: NOREF mask
pub const SYSTICK_CALIB_NOREF_MSK: u32 = 1 << SYSTICK_CALIB_NOREF_POS;
/// SysTick CALIB: SKEW position
pub const SYSTICK_CALIB_SKEW_POS: u32 = 30;
/// SysTick CALIB: SKEW mask
pub const SYSTICK_CALIB_SKEW_MSK: u32 = 1 << SYSTICK_CALIB_SKEW_POS;
/// SysTick CALIB: TENMS position
pub const SYSTICK_CALIB_TENMS_POS: u32 = 0;
/// SysTick CALIB: TENMS mask
pub const SYSTICK_CALIB_TENMS_MSK: u32 = 0xFFFFFF << SYSTICK_CALIB_TENMS_POS;

// ---------------------------------------------------------------------------
// ITM — Instrumentation Trace Macrocell
// ---------------------------------------------------------------------------

/// ITM Stimulus Port union for 8/16/32-bit writes.
#[repr(C)]
pub union ItmStimPort {
    /// 8-bit write access
    pub u8: VolatileWO<u8>,
    /// 16-bit write access
    pub u16: VolatileWO<u16>,
    /// 32-bit write access
    pub u32: VolatileWO<u32>,
}

define_reg! {
    /// Instrumentation Trace Macrocell (ITM) register block.
    Itm
    __Itm {
        /// ITM Stimulus Port Registers (offset 0x000, write-only)
        port: [ItmStimPort; 32],
        _reserved0: [u32; 864],
        /// ITM Trace Enable Register (offset 0xE00)
        ter: VolatileRW<u32>,
        _reserved1: [u32; 15],
        /// ITM Trace Privilege Register (offset 0xE40)
        tpr: VolatileRW<u32>,
        _reserved2: [u32; 15],
        /// ITM Trace Control Register (offset 0xE80)
        tcr: VolatileRW<u32>,
        _reserved3: [u32; 29],
        /// ITM Integration Write Register (offset 0xEF8, write-only)
        iwr: VolatileWO<u32>,
        /// ITM Integration Read Register (offset 0xEFC, read-only)
        irr: VolatileRO<u32>,
        /// ITM Integration Mode Control Register (offset 0xF00)
        imcr: VolatileRW<u32>,
        _reserved4: [u32; 43],
        /// ITM Lock Access Register (offset 0xFB0, write-only)
        lar: VolatileWO<u32>,
        /// ITM Lock Status Register (offset 0xFB4, read-only)
        lsr: VolatileRO<u32>,
        _reserved5: [u32; 6],
        /// ITM Peripheral Identification Register #4 (offset 0xFD0, read-only)
        pid4: VolatileRO<u32>,
        /// ITM Peripheral Identification Register #5 (offset 0xFD4, read-only)
        pid5: VolatileRO<u32>,
        /// ITM Peripheral Identification Register #6 (offset 0xFD8, read-only)
        pid6: VolatileRO<u32>,
        /// ITM Peripheral Identification Register #7 (offset 0xFDC, read-only)
        pid7: VolatileRO<u32>,
        /// ITM Peripheral Identification Register #0 (offset 0xFE0, read-only)
        pid0: VolatileRO<u32>,
        /// ITM Peripheral Identification Register #1 (offset 0xFE4, read-only)
        pid1: VolatileRO<u32>,
        /// ITM Peripheral Identification Register #2 (offset 0xFE8, read-only)
        pid2: VolatileRO<u32>,
        /// ITM Peripheral Identification Register #3 (offset 0xFEC, read-only)
        pid3: VolatileRO<u32>,
        /// ITM Component Identification Register #0 (offset 0xFF0, read-only)
        cid0: VolatileRO<u32>,
        /// ITM Component Identification Register #1 (offset 0xFF4, read-only)
        cid1: VolatileRO<u32>,
        /// ITM Component Identification Register #2 (offset 0xFF8, read-only)
        cid2: VolatileRO<u32>,
        /// ITM Component Identification Register #3 (offset 0xFFC, read-only)
        cid3: VolatileRO<u32>,
    }
}

/// ITM TPR: PRIVMASK position
pub const ITM_TPR_PRIVMASK_POS: u32 = 0;
/// ITM TPR: PRIVMASK mask
pub const ITM_TPR_PRIVMASK_MSK: u32 = 0xF << ITM_TPR_PRIVMASK_POS;
/// ITM TCR: BUSY position
pub const ITM_TCR_BUSY_POS: u32 = 23;
/// ITM TCR: BUSY mask
pub const ITM_TCR_BUSY_MSK: u32 = 1 << ITM_TCR_BUSY_POS;
/// ITM TCR: ATBID position
pub const ITM_TCR_TRACE_BUS_ID_POS: u32 = 16;
/// ITM TCR: ATBID mask
pub const ITM_TCR_TRACE_BUS_ID_MSK: u32 = 0x7F << ITM_TCR_TRACE_BUS_ID_POS;
/// ITM TCR: Global timestamp frequency position
pub const ITM_TCR_GTSFREQ_POS: u32 = 10;
/// ITM TCR: Global timestamp frequency mask
pub const ITM_TCR_GTSFREQ_MSK: u32 = 3 << ITM_TCR_GTSFREQ_POS;
/// ITM TCR: TSPrescale position
pub const ITM_TCR_TSPRESCALE_POS: u32 = 8;
/// ITM TCR: TSPrescale mask
pub const ITM_TCR_TSPRESCALE_MSK: u32 = 3 << ITM_TCR_TSPRESCALE_POS;
/// ITM TCR: SWOENA position
pub const ITM_TCR_SWOENA_POS: u32 = 4;
/// ITM TCR: SWOENA mask
pub const ITM_TCR_SWOENA_MSK: u32 = 1 << ITM_TCR_SWOENA_POS;
/// ITM TCR: DWTENA position
pub const ITM_TCR_DWTENA_POS: u32 = 3;
/// ITM TCR: DWTENA mask
pub const ITM_TCR_DWTENA_MSK: u32 = 1 << ITM_TCR_DWTENA_POS;
/// ITM TCR: SYNCENA position
pub const ITM_TCR_SYNCENA_POS: u32 = 2;
/// ITM TCR: SYNCENA mask
pub const ITM_TCR_SYNCENA_MSK: u32 = 1 << ITM_TCR_SYNCENA_POS;
/// ITM TCR: TSENA position
pub const ITM_TCR_TSENA_POS: u32 = 1;
/// ITM TCR: TSENA mask
pub const ITM_TCR_TSENA_MSK: u32 = 1 << ITM_TCR_TSENA_POS;
/// ITM TCR: ITM Enable bit position
pub const ITM_TCR_ITMENA_POS: u32 = 0;
/// ITM TCR: ITM Enable bit mask
pub const ITM_TCR_ITMENA_MSK: u32 = 1 << ITM_TCR_ITMENA_POS;
/// ITM IWR: ATVALIDM position
pub const ITM_IWR_ATVALIDM_POS: u32 = 0;
/// ITM IWR: ATVALIDM mask
pub const ITM_IWR_ATVALIDM_MSK: u32 = 1 << ITM_IWR_ATVALIDM_POS;
/// ITM IRR: ATREADYM position
pub const ITM_IRR_ATREADYM_POS: u32 = 0;
/// ITM IRR: ATREADYM mask
pub const ITM_IRR_ATREADYM_MSK: u32 = 1 << ITM_IRR_ATREADYM_POS;
/// ITM IMCR: INTEGRATION position
pub const ITM_IMCR_INTEGRATION_POS: u32 = 0;
/// ITM IMCR: INTEGRATION mask
pub const ITM_IMCR_INTEGRATION_MSK: u32 = 1 << ITM_IMCR_INTEGRATION_POS;
/// ITM LSR: ByteAcc position
pub const ITM_LSR_BYTE_ACC_POS: u32 = 2;
/// ITM LSR: ByteAcc mask
pub const ITM_LSR_BYTE_ACC_MSK: u32 = 1 << ITM_LSR_BYTE_ACC_POS;
/// ITM LSR: Access position
pub const ITM_LSR_ACCESS_POS: u32 = 1;
/// ITM LSR: Access mask
pub const ITM_LSR_ACCESS_MSK: u32 = 1 << ITM_LSR_ACCESS_POS;
/// ITM LSR: Present position
pub const ITM_LSR_PRESENT_POS: u32 = 0;
/// ITM LSR: Present mask
pub const ITM_LSR_PRESENT_MSK: u32 = 1 << ITM_LSR_PRESENT_POS;

// ---------------------------------------------------------------------------
// DWT — Data Watchpoint and Trace
// ---------------------------------------------------------------------------

define_reg! {
    /// Data Watchpoint and Trace (DWT) register block.
    Dwt
    __Dwt {
        /// Control Register (offset 0x000)
        ctrl: VolatileRW<u32>,
        /// Cycle Count Register (offset 0x004)
        cyccnt: VolatileRW<u32>,
        /// CPI Count Register (offset 0x008)
        cpicnt: VolatileRW<u32>,
        /// Exception Overhead Count Register (offset 0x00C)
        exccnt: VolatileRW<u32>,
        /// Sleep Count Register (offset 0x010)
        sleepcnt: VolatileRW<u32>,
        /// LSU Count Register (offset 0x014)
        lsucnt: VolatileRW<u32>,
        /// Folded-instruction Count Register (offset 0x018)
        foldcnt: VolatileRW<u32>,
        /// Program Counter Sample Register (offset 0x01C, read-only)
        pcsr: VolatileRO<u32>,
        /// Comparator Register 0 (offset 0x020)
        comp0: VolatileRW<u32>,
        /// Mask Register 0 (offset 0x024)
        mask0: VolatileRW<u32>,
        /// Function Register 0 (offset 0x028)
        function0: VolatileRW<u32>,
        _reserved0: [u32; 1],
        /// Comparator Register 1 (offset 0x030)
        comp1: VolatileRW<u32>,
        /// Mask Register 1 (offset 0x034)
        mask1: VolatileRW<u32>,
        /// Function Register 1 (offset 0x038)
        function1: VolatileRW<u32>,
        _reserved1: [u32; 1],
        /// Comparator Register 2 (offset 0x040)
        comp2: VolatileRW<u32>,
        /// Mask Register 2 (offset 0x044)
        mask2: VolatileRW<u32>,
        /// Function Register 2 (offset 0x048)
        function2: VolatileRW<u32>,
        _reserved2: [u32; 1],
        /// Comparator Register 3 (offset 0x050)
        comp3: VolatileRW<u32>,
        /// Mask Register 3 (offset 0x054)
        mask3: VolatileRW<u32>,
        /// Function Register 3 (offset 0x058)
        function3: VolatileRW<u32>,
    }
}

/// DWT CTRL: NUMCOMP position
pub const DWT_CTRL_NUMCOMP_POS: u32 = 28;
/// DWT CTRL: NUMCOMP mask
pub const DWT_CTRL_NUMCOMP_MSK: u32 = 0xF << DWT_CTRL_NUMCOMP_POS;
/// DWT CTRL: NOTRCPKT position
pub const DWT_CTRL_NOTRCPKT_POS: u32 = 27;
/// DWT CTRL: NOTRCPKT mask
pub const DWT_CTRL_NOTRCPKT_MSK: u32 = 0x1 << DWT_CTRL_NOTRCPKT_POS;
/// DWT CTRL: NOEXTTRIG position
pub const DWT_CTRL_NOEXTTRIG_POS: u32 = 26;
/// DWT CTRL: NOEXTTRIG mask
pub const DWT_CTRL_NOEXTTRIG_MSK: u32 = 0x1 << DWT_CTRL_NOEXTTRIG_POS;
/// DWT CTRL: NOCYCCNT position
pub const DWT_CTRL_NOCYCCNT_POS: u32 = 25;
/// DWT CTRL: NOCYCCNT mask
pub const DWT_CTRL_NOCYCCNT_MSK: u32 = 0x1 << DWT_CTRL_NOCYCCNT_POS;
/// DWT CTRL: NOPRFCNT position
pub const DWT_CTRL_NOPRFCNT_POS: u32 = 24;
/// DWT CTRL: NOPRFCNT mask
pub const DWT_CTRL_NOPRFCNT_MSK: u32 = 0x1 << DWT_CTRL_NOPRFCNT_POS;
/// DWT CTRL: CYCEVTENA position
pub const DWT_CTRL_CYCEVTENA_POS: u32 = 22;
/// DWT CTRL: CYCEVTENA mask
pub const DWT_CTRL_CYCEVTENA_MSK: u32 = 0x1 << DWT_CTRL_CYCEVTENA_POS;
/// DWT CTRL: FOLDEVTENA position
pub const DWT_CTRL_FOLDEVTENA_POS: u32 = 21;
/// DWT CTRL: FOLDEVTENA mask
pub const DWT_CTRL_FOLDEVTENA_MSK: u32 = 0x1 << DWT_CTRL_FOLDEVTENA_POS;
/// DWT CTRL: LSUEVTENA position
pub const DWT_CTRL_LSUEVTENA_POS: u32 = 20;
/// DWT CTRL: LSUEVTENA mask
pub const DWT_CTRL_LSUEVTENA_MSK: u32 = 0x1 << DWT_CTRL_LSUEVTENA_POS;
/// DWT CTRL: SLEEPEVTENA position
pub const DWT_CTRL_SLEEPEVTENA_POS: u32 = 19;
/// DWT CTRL: SLEEPEVTENA mask
pub const DWT_CTRL_SLEEPEVTENA_MSK: u32 = 0x1 << DWT_CTRL_SLEEPEVTENA_POS;
/// DWT CTRL: EXCEVTENA position
pub const DWT_CTRL_EXCEVTENA_POS: u32 = 18;
/// DWT CTRL: EXCEVTENA mask
pub const DWT_CTRL_EXCEVTENA_MSK: u32 = 0x1 << DWT_CTRL_EXCEVTENA_POS;
/// DWT CTRL: CPIEVTENA position
pub const DWT_CTRL_CPIEVTENA_POS: u32 = 17;
/// DWT CTRL: CPIEVTENA mask
pub const DWT_CTRL_CPIEVTENA_MSK: u32 = 0x1 << DWT_CTRL_CPIEVTENA_POS;
/// DWT CTRL: EXCTRCENA position
pub const DWT_CTRL_EXCTRCENA_POS: u32 = 16;
/// DWT CTRL: EXCTRCENA mask
pub const DWT_CTRL_EXCTRCENA_MSK: u32 = 0x1 << DWT_CTRL_EXCTRCENA_POS;
/// DWT CTRL: PCSAMPLENA position
pub const DWT_CTRL_PCSAMPLENA_POS: u32 = 12;
/// DWT CTRL: PCSAMPLENA mask
pub const DWT_CTRL_PCSAMPLENA_MSK: u32 = 0x1 << DWT_CTRL_PCSAMPLENA_POS;
/// DWT CTRL: SYNCTAP position
pub const DWT_CTRL_SYNCTAP_POS: u32 = 10;
/// DWT CTRL: SYNCTAP mask
pub const DWT_CTRL_SYNCTAP_MSK: u32 = 0x3 << DWT_CTRL_SYNCTAP_POS;
/// DWT CTRL: CYCTAP position
pub const DWT_CTRL_CYCTAP_POS: u32 = 9;
/// DWT CTRL: CYCTAP mask
pub const DWT_CTRL_CYCTAP_MSK: u32 = 0x1 << DWT_CTRL_CYCTAP_POS;
/// DWT CTRL: POSTINIT position
pub const DWT_CTRL_POSTINIT_POS: u32 = 5;
/// DWT CTRL: POSTINIT mask
pub const DWT_CTRL_POSTINIT_MSK: u32 = 0xF << DWT_CTRL_POSTINIT_POS;
/// DWT CTRL: POSTPRESET position
pub const DWT_CTRL_POSTPRESET_POS: u32 = 1;
/// DWT CTRL: POSTPRESET mask
pub const DWT_CTRL_POSTPRESET_MSK: u32 = 0xF << DWT_CTRL_POSTPRESET_POS;
/// DWT CTRL: CYCCNTENA position
pub const DWT_CTRL_CYCCNTENA_POS: u32 = 0;
/// DWT CTRL: CYCCNTENA mask
pub const DWT_CTRL_CYCCNTENA_MSK: u32 = 0x1 << DWT_CTRL_CYCCNTENA_POS;

/// DWT CPICNT: CPICNT position
pub const DWT_CPICNT_CPICNT_POS: u32 = 0;
/// DWT CPICNT: CPICNT mask
pub const DWT_CPICNT_CPICNT_MSK: u32 = 0xFF << DWT_CPICNT_CPICNT_POS;
/// DWT EXCCNT: EXCCNT position
pub const DWT_EXCCNT_EXCCNT_POS: u32 = 0;
/// DWT EXCCNT: EXCCNT mask
pub const DWT_EXCCNT_EXCCNT_MSK: u32 = 0xFF << DWT_EXCCNT_EXCCNT_POS;
/// DWT SLEEPCNT: SLEEPCNT position
pub const DWT_SLEEPCNT_SLEEPCNT_POS: u32 = 0;
/// DWT SLEEPCNT: SLEEPCNT mask
pub const DWT_SLEEPCNT_SLEEPCNT_MSK: u32 = 0xFF << DWT_SLEEPCNT_SLEEPCNT_POS;
/// DWT LSUCNT: LSUCNT position
pub const DWT_LSUCNT_LSUCNT_POS: u32 = 0;
/// DWT LSUCNT: LSUCNT mask
pub const DWT_LSUCNT_LSUCNT_MSK: u32 = 0xFF << DWT_LSUCNT_LSUCNT_POS;
/// DWT FOLDCNT: FOLDCNT position
pub const DWT_FOLDCNT_FOLDCNT_POS: u32 = 0;
/// DWT FOLDCNT: FOLDCNT mask
pub const DWT_FOLDCNT_FOLDCNT_MSK: u32 = 0xFF << DWT_FOLDCNT_FOLDCNT_POS;
/// DWT MASK: MASK position
pub const DWT_MASK_MASK_POS: u32 = 0;
/// DWT MASK: MASK mask
pub const DWT_MASK_MASK_MSK: u32 = 0x1F << DWT_MASK_MASK_POS;

/// DWT FUNCTION: MATCHED position
pub const DWT_FUNCTION_MATCHED_POS: u32 = 24;
/// DWT FUNCTION: MATCHED mask
pub const DWT_FUNCTION_MATCHED_MSK: u32 = 0x1 << DWT_FUNCTION_MATCHED_POS;
/// DWT FUNCTION: DATAVADDR1 position
pub const DWT_FUNCTION_DATAVADDR1_POS: u32 = 16;
/// DWT FUNCTION: DATAVADDR1 mask
pub const DWT_FUNCTION_DATAVADDR1_MSK: u32 = 0xF << DWT_FUNCTION_DATAVADDR1_POS;
/// DWT FUNCTION: DATAVADDR0 position
pub const DWT_FUNCTION_DATAVADDR0_POS: u32 = 12;
/// DWT FUNCTION: DATAVADDR0 mask
pub const DWT_FUNCTION_DATAVADDR0_MSK: u32 = 0xF << DWT_FUNCTION_DATAVADDR0_POS;
/// DWT FUNCTION: DATAVSIZE position
pub const DWT_FUNCTION_DATAVSIZE_POS: u32 = 10;
/// DWT FUNCTION: DATAVSIZE mask
pub const DWT_FUNCTION_DATAVSIZE_MSK: u32 = 0x3 << DWT_FUNCTION_DATAVSIZE_POS;
/// DWT FUNCTION: LNK1ENA position
pub const DWT_FUNCTION_LNK1ENA_POS: u32 = 9;
/// DWT FUNCTION: LNK1ENA mask
pub const DWT_FUNCTION_LNK1ENA_MSK: u32 = 0x1 << DWT_FUNCTION_LNK1ENA_POS;
/// DWT FUNCTION: DATAVMATCH position
pub const DWT_FUNCTION_DATAVMATCH_POS: u32 = 8;
/// DWT FUNCTION: DATAVMATCH mask
pub const DWT_FUNCTION_DATAVMATCH_MSK: u32 = 0x1 << DWT_FUNCTION_DATAVMATCH_POS;
/// DWT FUNCTION: CYCMATCH position
pub const DWT_FUNCTION_CYCMATCH_POS: u32 = 7;
/// DWT FUNCTION: CYCMATCH mask
pub const DWT_FUNCTION_CYCMATCH_MSK: u32 = 0x1 << DWT_FUNCTION_CYCMATCH_POS;
/// DWT FUNCTION: EMITRANGE position
pub const DWT_FUNCTION_EMITRANGE_POS: u32 = 5;
/// DWT FUNCTION: EMITRANGE mask
pub const DWT_FUNCTION_EMITRANGE_MSK: u32 = 0x1 << DWT_FUNCTION_EMITRANGE_POS;
/// DWT FUNCTION: FUNCTION position
pub const DWT_FUNCTION_FUNCTION_POS: u32 = 0;
/// DWT FUNCTION: FUNCTION mask
pub const DWT_FUNCTION_FUNCTION_MSK: u32 = 0xF << DWT_FUNCTION_FUNCTION_POS;

// ---------------------------------------------------------------------------
// TPI — Trace Port Interface
// ---------------------------------------------------------------------------

define_reg! {
    /// Trace Port Interface (TPI) register block.
    Tpi
    __Tpi {
        /// Supported Parallel Port Size Register (offset 0x000, read-only)
        sspsr: VolatileRO<u32>,
        /// Current Parallel Port Size Register (offset 0x004)
        cspsr: VolatileRW<u32>,
        _reserved0: [u32; 2],
        /// Asynchronous Clock Prescaler Register (offset 0x010)
        acpr: VolatileRW<u32>,
        _reserved1: [u32; 55],
        /// Selected Pin Protocol Register (offset 0x0F0)
        sppr: VolatileRW<u32>,
        _reserved2: [u32; 131],
        /// Formatter and Flush Status Register (offset 0x300, read-only)
        ffsr: VolatileRO<u32>,
        /// Formatter and Flush Control Register (offset 0x304)
        ffcr: VolatileRW<u32>,
        /// Formatter Synchronization Counter Register (offset 0x308, read-only)
        fscr: VolatileRO<u32>,
        _reserved3: [u32; 759],
        /// TRIGGER Register (offset 0xEE8, read-only)
        trigger: VolatileRO<u32>,
        /// Integration ETM Data (offset 0xEEC, read-only)
        fifo0: VolatileRO<u32>,
        /// ITATBCTR2 (offset 0xEF0, read-only)
        itatbctr2: VolatileRO<u32>,
        _reserved4: [u32; 1],
        /// ITATBCTR0 (offset 0xEF8, read-only)
        itatbctr0: VolatileRO<u32>,
        /// Integration ITM Data (offset 0xEFC, read-only)
        fifo1: VolatileRO<u32>,
        /// Integration Mode Control (offset 0xF00)
        itctrl: VolatileRW<u32>,
        _reserved5: [u32; 39],
        /// Claim tag set (offset 0xFA0)
        claimset: VolatileRW<u32>,
        /// Claim tag clear (offset 0xFA4)
        claimclr: VolatileRW<u32>,
        _reserved7: [u32; 8],
        /// TPIU_DEVID (offset 0xFC8, read-only)
        devid: VolatileRO<u32>,
        /// TPIU_DEVTYPE (offset 0xFCC, read-only)
        devtype: VolatileRO<u32>,
    }
}

/// TPI ACPR: PRESCALER position
pub const TPI_ACPR_PRESCALER_POS: u32 = 0;
/// TPI ACPR: PRESCALER mask
pub const TPI_ACPR_PRESCALER_MSK: u32 = 0x1FFF << TPI_ACPR_PRESCALER_POS;
/// TPI SPPR: TXMODE position
pub const TPI_SPPR_TXMODE_POS: u32 = 0;
/// TPI SPPR: TXMODE mask
pub const TPI_SPPR_TXMODE_MSK: u32 = 0x3 << TPI_SPPR_TXMODE_POS;
/// TPI FFSR: FtNonStop position
pub const TPI_FFSR_FT_NON_STOP_POS: u32 = 3;
/// TPI FFSR: FtNonStop mask
pub const TPI_FFSR_FT_NON_STOP_MSK: u32 = 0x1 << TPI_FFSR_FT_NON_STOP_POS;
/// TPI FFSR: TCPresent position
pub const TPI_FFSR_TC_PRESENT_POS: u32 = 2;
/// TPI FFSR: TCPresent mask
pub const TPI_FFSR_TC_PRESENT_MSK: u32 = 0x1 << TPI_FFSR_TC_PRESENT_POS;
/// TPI FFSR: FtStopped position
pub const TPI_FFSR_FT_STOPPED_POS: u32 = 1;
/// TPI FFSR: FtStopped mask
pub const TPI_FFSR_FT_STOPPED_MSK: u32 = 0x1 << TPI_FFSR_FT_STOPPED_POS;
/// TPI FFSR: FlInProg position
pub const TPI_FFSR_FL_IN_PROG_POS: u32 = 0;
/// TPI FFSR: FlInProg mask
pub const TPI_FFSR_FL_IN_PROG_MSK: u32 = 0x1 << TPI_FFSR_FL_IN_PROG_POS;
/// TPI FFCR: TrigIn position
pub const TPI_FFCR_TRIG_IN_POS: u32 = 8;
/// TPI FFCR: TrigIn mask
pub const TPI_FFCR_TRIG_IN_MSK: u32 = 0x1 << TPI_FFCR_TRIG_IN_POS;
/// TPI FFCR: EnFCont position
pub const TPI_FFCR_EN_FCONT_POS: u32 = 1;
/// TPI FFCR: EnFCont mask
pub const TPI_FFCR_EN_FCONT_MSK: u32 = 0x1 << TPI_FFCR_EN_FCONT_POS;
/// TPI TRIGGER: TRIGGER position
pub const TPI_TRIGGER_TRIGGER_POS: u32 = 0;
/// TPI TRIGGER: TRIGGER mask
pub const TPI_TRIGGER_TRIGGER_MSK: u32 = 0x1 << TPI_TRIGGER_TRIGGER_POS;
/// TPI ITCTRL: Mode position
pub const TPI_ITCTRL_MODE_POS: u32 = 0;
/// TPI ITCTRL: Mode mask
pub const TPI_ITCTRL_MODE_MSK: u32 = 0x1 << TPI_ITCTRL_MODE_POS;
/// TPI DEVID: NRZVALID position
pub const TPI_DEVID_NRZVALID_POS: u32 = 11;
/// TPI DEVID: NRZVALID mask
pub const TPI_DEVID_NRZVALID_MSK: u32 = 0x1 << TPI_DEVID_NRZVALID_POS;
/// TPI DEVID: MANCVALID position
pub const TPI_DEVID_MANCVALID_POS: u32 = 10;
/// TPI DEVID: MANCVALID mask
pub const TPI_DEVID_MANCVALID_MSK: u32 = 0x1 << TPI_DEVID_MANCVALID_POS;
/// TPI DEVID: PTINVALID position
pub const TPI_DEVID_PTINVALID_POS: u32 = 9;
/// TPI DEVID: PTINVALID mask
pub const TPI_DEVID_PTINVALID_MSK: u32 = 0x1 << TPI_DEVID_PTINVALID_POS;
/// TPI DEVID: MinBufSz position
pub const TPI_DEVID_MIN_BUF_SZ_POS: u32 = 6;
/// TPI DEVID: MinBufSz mask
pub const TPI_DEVID_MIN_BUF_SZ_MSK: u32 = 0x7 << TPI_DEVID_MIN_BUF_SZ_POS;
/// TPI DEVID: AsynClkIn position
pub const TPI_DEVID_ASYN_CLK_IN_POS: u32 = 5;
/// TPI DEVID: AsynClkIn mask
pub const TPI_DEVID_ASYN_CLK_IN_MSK: u32 = 0x1 << TPI_DEVID_ASYN_CLK_IN_POS;
/// TPI DEVID: NrTraceInput position
pub const TPI_DEVID_NR_TRACE_INPUT_POS: u32 = 0;
/// TPI DEVID: NrTraceInput mask
pub const TPI_DEVID_NR_TRACE_INPUT_MSK: u32 = 0x1F << TPI_DEVID_NR_TRACE_INPUT_POS;
/// TPI DEVTYPE: SubType position
pub const TPI_DEVTYPE_SUB_TYPE_POS: u32 = 0;
/// TPI DEVTYPE: SubType mask
pub const TPI_DEVTYPE_SUB_TYPE_MSK: u32 = 0xF << TPI_DEVTYPE_SUB_TYPE_POS;
/// TPI DEVTYPE: MajorType position
pub const TPI_DEVTYPE_MAJOR_TYPE_POS: u32 = 4;
/// TPI DEVTYPE: MajorType mask
pub const TPI_DEVTYPE_MAJOR_TYPE_MSK: u32 = 0xF << TPI_DEVTYPE_MAJOR_TYPE_POS;

// ---------------------------------------------------------------------------
// MPU — Memory Protection Unit
// ---------------------------------------------------------------------------

define_reg! {
    /// Memory Protection Unit (MPU) register block.
    Mpu
    __Mpu {
        /// MPU Type Register (offset 0x000, read-only)
        type_: VolatileRO<u32>,
        /// MPU Control Register (offset 0x004)
        ctrl: VolatileRW<u32>,
        /// MPU Region Number Register (offset 0x008)
        rnr: VolatileRW<u32>,
        /// MPU Region Base Address Register (offset 0x00C)
        rbar: VolatileRW<u32>,
        /// MPU Region Attribute and Size Register (offset 0x010)
        rasr: VolatileRW<u32>,
        /// MPU Alias 1 Region Base Address Register (offset 0x014)
        rbar_a1: VolatileRW<u32>,
        /// MPU Alias 1 Region Attribute and Size Register (offset 0x018)
        rasr_a1: VolatileRW<u32>,
        /// MPU Alias 2 Region Base Address Register (offset 0x01C)
        rbar_a2: VolatileRW<u32>,
        /// MPU Alias 2 Region Attribute and Size Register (offset 0x020)
        rasr_a2: VolatileRW<u32>,
        /// MPU Alias 3 Region Base Address Register (offset 0x024)
        rbar_a3: VolatileRW<u32>,
        /// MPU Alias 3 Region Attribute and Size Register (offset 0x028)
        rasr_a3: VolatileRW<u32>,
    }
}

/// MPU TYPE: IREGION position
pub const MPU_TYPE_IREGION_POS: u32 = 16;
/// MPU TYPE: IREGION mask
pub const MPU_TYPE_IREGION_MSK: u32 = 0xFF << MPU_TYPE_IREGION_POS;
/// MPU TYPE: DREGION position
pub const MPU_TYPE_DREGION_POS: u32 = 8;
/// MPU TYPE: DREGION mask
pub const MPU_TYPE_DREGION_MSK: u32 = 0xFF << MPU_TYPE_DREGION_POS;
/// MPU TYPE: SEPARATE position
pub const MPU_TYPE_SEPARATE_POS: u32 = 0;
/// MPU TYPE: SEPARATE mask
pub const MPU_TYPE_SEPARATE_MSK: u32 = 1 << MPU_TYPE_SEPARATE_POS;
/// MPU CTRL: PRIVDEFENA position
pub const MPU_CTRL_PRIVDEFENA_POS: u32 = 2;
/// MPU CTRL: PRIVDEFENA mask
pub const MPU_CTRL_PRIVDEFENA_MSK: u32 = 1 << MPU_CTRL_PRIVDEFENA_POS;
/// MPU CTRL: HFNMIENA position
pub const MPU_CTRL_HFNMIENA_POS: u32 = 1;
/// MPU CTRL: HFNMIENA mask
pub const MPU_CTRL_HFNMIENA_MSK: u32 = 1 << MPU_CTRL_HFNMIENA_POS;
/// MPU CTRL: ENABLE position
pub const MPU_CTRL_ENABLE_POS: u32 = 0;
/// MPU CTRL: ENABLE mask
pub const MPU_CTRL_ENABLE_MSK: u32 = 1 << MPU_CTRL_ENABLE_POS;
/// MPU RNR: REGION position
pub const MPU_RNR_REGION_POS: u32 = 0;
/// MPU RNR: REGION mask
pub const MPU_RNR_REGION_MSK: u32 = 0xFF << MPU_RNR_REGION_POS;
/// MPU RBAR: ADDR position
pub const MPU_RBAR_ADDR_POS: u32 = 5;
/// MPU RBAR: ADDR mask
pub const MPU_RBAR_ADDR_MSK: u32 = 0x7FFFFFF << MPU_RBAR_ADDR_POS;
/// MPU RBAR: VALID position
pub const MPU_RBAR_VALID_POS: u32 = 4;
/// MPU RBAR: VALID mask
pub const MPU_RBAR_VALID_MSK: u32 = 1 << MPU_RBAR_VALID_POS;
/// MPU RBAR: REGION position
pub const MPU_RBAR_REGION_POS: u32 = 0;
/// MPU RBAR: REGION mask
pub const MPU_RBAR_REGION_MSK: u32 = 0xF << MPU_RBAR_REGION_POS;
/// MPU RASR: ATTRS position
pub const MPU_RASR_ATTRS_POS: u32 = 16;
/// MPU RASR: ATTRS mask
pub const MPU_RASR_ATTRS_MSK: u32 = 0xFFFF << MPU_RASR_ATTRS_POS;
/// MPU RASR: XN position
pub const MPU_RASR_XN_POS: u32 = 28;
/// MPU RASR: XN mask
pub const MPU_RASR_XN_MSK: u32 = 1 << MPU_RASR_XN_POS;
/// MPU RASR: AP position
pub const MPU_RASR_AP_POS: u32 = 24;
/// MPU RASR: AP mask
pub const MPU_RASR_AP_MSK: u32 = 0x7 << MPU_RASR_AP_POS;
/// MPU RASR: TEX position
pub const MPU_RASR_TEX_POS: u32 = 19;
/// MPU RASR: TEX mask
pub const MPU_RASR_TEX_MSK: u32 = 0x7 << MPU_RASR_TEX_POS;
/// MPU RASR: S position
pub const MPU_RASR_S_POS: u32 = 18;
/// MPU RASR: S mask
pub const MPU_RASR_S_MSK: u32 = 1 << MPU_RASR_S_POS;
/// MPU RASR: C position
pub const MPU_RASR_C_POS: u32 = 17;
/// MPU RASR: C mask
pub const MPU_RASR_C_MSK: u32 = 1 << MPU_RASR_C_POS;
/// MPU RASR: B position
pub const MPU_RASR_B_POS: u32 = 16;
/// MPU RASR: B mask
pub const MPU_RASR_B_MSK: u32 = 1 << MPU_RASR_B_POS;
/// MPU RASR: Sub-Region Disable position
pub const MPU_RASR_SRD_POS: u32 = 8;
/// MPU RASR: Sub-Region Disable mask
pub const MPU_RASR_SRD_MSK: u32 = 0xFF << MPU_RASR_SRD_POS;
/// MPU RASR: Region Size Field position
pub const MPU_RASR_SIZE_POS: u32 = 1;
/// MPU RASR: Region Size Field mask
pub const MPU_RASR_SIZE_MSK: u32 = 0x1F << MPU_RASR_SIZE_POS;
/// MPU RASR: Region enable bit position
pub const MPU_RASR_ENABLE_POS: u32 = 0;
/// MPU RASR: Region enable bit mask
pub const MPU_RASR_ENABLE_MSK: u32 = 1 << MPU_RASR_ENABLE_POS;

// ---------------------------------------------------------------------------
// FPU — Floating Point Unit
// ---------------------------------------------------------------------------

define_reg! {
    /// Floating Point Unit (FPU) register block.
    Fpu
    __Fpu {
        _reserved0: [u32; 1],
        /// Floating-Point Context Control Register (offset 0x004)
        fpccr: VolatileRW<u32>,
        /// Floating-Point Context Address Register (offset 0x008)
        fpcar: VolatileRW<u32>,
        /// Floating-Point Default Status Control Register (offset 0x00C)
        fpdscr: VolatileRW<u32>,
        /// Media and FP Feature Register 0 (offset 0x010, read-only)
        mvfr0: VolatileRO<u32>,
        /// Media and FP Feature Register 1 (offset 0x014, read-only)
        mvfr1: VolatileRO<u32>,
    }
}

/// FPCCR: ASPEN position
pub const FPU_FPCCR_ASPEN_POS: u32 = 31;
/// FPCCR: ASPEN mask
pub const FPU_FPCCR_ASPEN_MSK: u32 = 1 << FPU_FPCCR_ASPEN_POS;
/// FPCCR: LSPEN position
pub const FPU_FPCCR_LSPEN_POS: u32 = 30;
/// FPCCR: LSPEN mask
pub const FPU_FPCCR_LSPEN_MSK: u32 = 1 << FPU_FPCCR_LSPEN_POS;
/// FPCCR: MONRDY position
pub const FPU_FPCCR_MONRDY_POS: u32 = 8;
/// FPCCR: MONRDY mask
pub const FPU_FPCCR_MONRDY_MSK: u32 = 1 << FPU_FPCCR_MONRDY_POS;
/// FPCCR: BFRDY position
pub const FPU_FPCCR_BFRDY_POS: u32 = 6;
/// FPCCR: BFRDY mask
pub const FPU_FPCCR_BFRDY_MSK: u32 = 1 << FPU_FPCCR_BFRDY_POS;
/// FPCCR: MMRDY position
pub const FPU_FPCCR_MMRDY_POS: u32 = 5;
/// FPCCR: MMRDY mask
pub const FPU_FPCCR_MMRDY_MSK: u32 = 1 << FPU_FPCCR_MMRDY_POS;
/// FPCCR: HFRDY position
pub const FPU_FPCCR_HFRDY_POS: u32 = 4;
/// FPCCR: HFRDY mask
pub const FPU_FPCCR_HFRDY_MSK: u32 = 1 << FPU_FPCCR_HFRDY_POS;
/// FPCCR: processor mode bit position
pub const FPU_FPCCR_THREAD_POS: u32 = 3;
/// FPCCR: processor mode active bit mask
pub const FPU_FPCCR_THREAD_MSK: u32 = 1 << FPU_FPCCR_THREAD_POS;
/// FPCCR: privilege level bit position
pub const FPU_FPCCR_USER_POS: u32 = 1;
/// FPCCR: privilege level bit mask
pub const FPU_FPCCR_USER_MSK: u32 = 1 << FPU_FPCCR_USER_POS;
/// FPCCR: Lazy state preservation active bit position
pub const FPU_FPCCR_LSPACT_POS: u32 = 0;
/// FPCCR: Lazy state preservation active bit mask
pub const FPU_FPCCR_LSPACT_MSK: u32 = 1 << FPU_FPCCR_LSPACT_POS;
/// FPCAR: ADDRESS position
pub const FPU_FPCAR_ADDRESS_POS: u32 = 3;
/// FPCAR: ADDRESS mask
pub const FPU_FPCAR_ADDRESS_MSK: u32 = 0x1FFFFFFF << FPU_FPCAR_ADDRESS_POS;
/// FPDSCR: AHP position
pub const FPU_FPDSCR_AHP_POS: u32 = 26;
/// FPDSCR: AHP mask
pub const FPU_FPDSCR_AHP_MSK: u32 = 1 << FPU_FPDSCR_AHP_POS;
/// FPDSCR: DN position
pub const FPU_FPDSCR_DN_POS: u32 = 25;
/// FPDSCR: DN mask
pub const FPU_FPDSCR_DN_MSK: u32 = 1 << FPU_FPDSCR_DN_POS;
/// FPDSCR: FZ position
pub const FPU_FPDSCR_FZ_POS: u32 = 24;
/// FPDSCR: FZ mask
pub const FPU_FPDSCR_FZ_MSK: u32 = 1 << FPU_FPDSCR_FZ_POS;
/// FPDSCR: RMode position
pub const FPU_FPDSCR_RMODE_POS: u32 = 22;
/// FPDSCR: RMode mask
pub const FPU_FPDSCR_RMODE_MSK: u32 = 3 << FPU_FPDSCR_RMODE_POS;
/// MVFR0: FP rounding modes position
pub const FPU_MVFR0_FP_ROUNDING_MODES_POS: u32 = 28;
/// MVFR0: FP rounding modes mask
pub const FPU_MVFR0_FP_ROUNDING_MODES_MSK: u32 = 0xF << FPU_MVFR0_FP_ROUNDING_MODES_POS;
/// MVFR0: Short vectors position
pub const FPU_MVFR0_SHORT_VECTORS_POS: u32 = 24;
/// MVFR0: Short vectors mask
pub const FPU_MVFR0_SHORT_VECTORS_MSK: u32 = 0xF << FPU_MVFR0_SHORT_VECTORS_POS;
/// MVFR0: Square root position
pub const FPU_MVFR0_SQUARE_ROOT_POS: u32 = 20;
/// MVFR0: Square root mask
pub const FPU_MVFR0_SQUARE_ROOT_MSK: u32 = 0xF << FPU_MVFR0_SQUARE_ROOT_POS;
/// MVFR0: Divide position
pub const FPU_MVFR0_DIVIDE_POS: u32 = 16;
/// MVFR0: Divide mask
pub const FPU_MVFR0_DIVIDE_MSK: u32 = 0xF << FPU_MVFR0_DIVIDE_POS;
/// MVFR0: FP exception trapping position
pub const FPU_MVFR0_FP_EXCEP_TRAPPING_POS: u32 = 12;
/// MVFR0: FP exception trapping mask
pub const FPU_MVFR0_FP_EXCEP_TRAPPING_MSK: u32 = 0xF << FPU_MVFR0_FP_EXCEP_TRAPPING_POS;
/// MVFR0: Double-precision position
pub const FPU_MVFR0_DOUBLE_PRECISION_POS: u32 = 8;
/// MVFR0: Double-precision mask
pub const FPU_MVFR0_DOUBLE_PRECISION_MSK: u32 = 0xF << FPU_MVFR0_DOUBLE_PRECISION_POS;
/// MVFR0: Single-precision position
pub const FPU_MVFR0_SINGLE_PRECISION_POS: u32 = 4;
/// MVFR0: Single-precision mask
pub const FPU_MVFR0_SINGLE_PRECISION_MSK: u32 = 0xF << FPU_MVFR0_SINGLE_PRECISION_POS;
/// MVFR0: A_SIMD registers position
pub const FPU_MVFR0_A_SIMD_REGISTERS_POS: u32 = 0;
/// MVFR0: A_SIMD registers mask
pub const FPU_MVFR0_A_SIMD_REGISTERS_MSK: u32 = 0xF << FPU_MVFR0_A_SIMD_REGISTERS_POS;
/// MVFR1: FP fused MAC position
pub const FPU_MVFR1_FP_FUSED_MAC_POS: u32 = 28;
/// MVFR1: FP fused MAC mask
pub const FPU_MVFR1_FP_FUSED_MAC_MSK: u32 = 0xF << FPU_MVFR1_FP_FUSED_MAC_POS;
/// MVFR1: FP HPFP position
pub const FPU_MVFR1_FP_HPFP_POS: u32 = 24;
/// MVFR1: FP HPFP mask
pub const FPU_MVFR1_FP_HPFP_MSK: u32 = 0xF << FPU_MVFR1_FP_HPFP_POS;
/// MVFR1: D_NaN mode position
pub const FPU_MVFR1_D_NAN_MODE_POS: u32 = 4;
/// MVFR1: D_NaN mode mask
pub const FPU_MVFR1_D_NAN_MODE_MSK: u32 = 0xF << FPU_MVFR1_D_NAN_MODE_POS;
/// MVFR1: FtZ mode position
pub const FPU_MVFR1_FTZ_MODE_POS: u32 = 0;
/// MVFR1: FtZ mode mask
pub const FPU_MVFR1_FTZ_MODE_MSK: u32 = 0xF << FPU_MVFR1_FTZ_MODE_POS;

// ---------------------------------------------------------------------------
// CoreDebug — Core Debug Registers
// ---------------------------------------------------------------------------

define_reg! {
    /// Core Debug Register (CoreDebug) block.
    CoreDebug
    __CoreDebug {
        /// Debug Halting Control and Status Register (offset 0x000)
        dhcsr: VolatileRW<u32>,
        /// Debug Core Register Selector Register (offset 0x004, write-only)
        dcrsr: VolatileWO<u32>,
        /// Debug Core Register Data Register (offset 0x008)
        dcrdr: VolatileRW<u32>,
        /// Debug Exception and Monitor Control Register (offset 0x00C)
        demcr: VolatileRW<u32>,
    }
}

/// CoreDebug DHCSR: DBGKEY position
pub const CORE_DEBUG_DHCSR_DBGKEY_POS: u32 = 16;
/// CoreDebug DHCSR: DBGKEY mask
pub const CORE_DEBUG_DHCSR_DBGKEY_MSK: u32 = 0xFFFF << CORE_DEBUG_DHCSR_DBGKEY_POS;
/// CoreDebug DHCSR: S_RESET_ST position
pub const CORE_DEBUG_DHCSR_S_RESET_ST_POS: u32 = 25;
/// CoreDebug DHCSR: S_RESET_ST mask
pub const CORE_DEBUG_DHCSR_S_RESET_ST_MSK: u32 = 1 << CORE_DEBUG_DHCSR_S_RESET_ST_POS;
/// CoreDebug DHCSR: S_RETIRE_ST position
pub const CORE_DEBUG_DHCSR_S_RETIRE_ST_POS: u32 = 24;
/// CoreDebug DHCSR: S_RETIRE_ST mask
pub const CORE_DEBUG_DHCSR_S_RETIRE_ST_MSK: u32 = 1 << CORE_DEBUG_DHCSR_S_RETIRE_ST_POS;
/// CoreDebug DHCSR: S_LOCKUP position
pub const CORE_DEBUG_DHCSR_S_LOCKUP_POS: u32 = 19;
/// CoreDebug DHCSR: S_LOCKUP mask
pub const CORE_DEBUG_DHCSR_S_LOCKUP_MSK: u32 = 1 << CORE_DEBUG_DHCSR_S_LOCKUP_POS;
/// CoreDebug DHCSR: S_SLEEP position
pub const CORE_DEBUG_DHCSR_S_SLEEP_POS: u32 = 18;
/// CoreDebug DHCSR: S_SLEEP mask
pub const CORE_DEBUG_DHCSR_S_SLEEP_MSK: u32 = 1 << CORE_DEBUG_DHCSR_S_SLEEP_POS;
/// CoreDebug DHCSR: S_HALT position
pub const CORE_DEBUG_DHCSR_S_HALT_POS: u32 = 17;
/// CoreDebug DHCSR: S_HALT mask
pub const CORE_DEBUG_DHCSR_S_HALT_MSK: u32 = 1 << CORE_DEBUG_DHCSR_S_HALT_POS;
/// CoreDebug DHCSR: S_REGRDY position
pub const CORE_DEBUG_DHCSR_S_REGRDY_POS: u32 = 16;
/// CoreDebug DHCSR: S_REGRDY mask
pub const CORE_DEBUG_DHCSR_S_REGRDY_MSK: u32 = 1 << CORE_DEBUG_DHCSR_S_REGRDY_POS;
/// CoreDebug DHCSR: C_SNAPSTALL position
pub const CORE_DEBUG_DHCSR_C_SNAPSTALL_POS: u32 = 5;
/// CoreDebug DHCSR: C_SNAPSTALL mask
pub const CORE_DEBUG_DHCSR_C_SNAPSTALL_MSK: u32 = 1 << CORE_DEBUG_DHCSR_C_SNAPSTALL_POS;
/// CoreDebug DHCSR: C_MASKINTS position
pub const CORE_DEBUG_DHCSR_C_MASKINTS_POS: u32 = 3;
/// CoreDebug DHCSR: C_MASKINTS mask
pub const CORE_DEBUG_DHCSR_C_MASKINTS_MSK: u32 = 1 << CORE_DEBUG_DHCSR_C_MASKINTS_POS;
/// CoreDebug DHCSR: C_STEP position
pub const CORE_DEBUG_DHCSR_C_STEP_POS: u32 = 2;
/// CoreDebug DHCSR: C_STEP mask
pub const CORE_DEBUG_DHCSR_C_STEP_MSK: u32 = 1 << CORE_DEBUG_DHCSR_C_STEP_POS;
/// CoreDebug DHCSR: C_HALT position
pub const CORE_DEBUG_DHCSR_C_HALT_POS: u32 = 1;
/// CoreDebug DHCSR: C_HALT mask
pub const CORE_DEBUG_DHCSR_C_HALT_MSK: u32 = 1 << CORE_DEBUG_DHCSR_C_HALT_POS;
/// CoreDebug DHCSR: C_DEBUGEN position
pub const CORE_DEBUG_DHCSR_C_DEBUGEN_POS: u32 = 0;
/// CoreDebug DHCSR: C_DEBUGEN mask
pub const CORE_DEBUG_DHCSR_C_DEBUGEN_MSK: u32 = 1 << CORE_DEBUG_DHCSR_C_DEBUGEN_POS;
/// CoreDebug DCRSR: REGWnR position
pub const CORE_DEBUG_DCRSR_REGWNR_POS: u32 = 16;
/// CoreDebug DCRSR: REGWnR mask
pub const CORE_DEBUG_DCRSR_REGWNR_MSK: u32 = 1 << CORE_DEBUG_DCRSR_REGWNR_POS;
/// CoreDebug DCRSR: REGSEL position
pub const CORE_DEBUG_DCRSR_REGSEL_POS: u32 = 0;
/// CoreDebug DCRSR: REGSEL mask
pub const CORE_DEBUG_DCRSR_REGSEL_MSK: u32 = 0x1F << CORE_DEBUG_DCRSR_REGSEL_POS;
/// CoreDebug DEMCR: TRCENA position
pub const CORE_DEBUG_DEMCR_TRCENA_POS: u32 = 24;
/// CoreDebug DEMCR: TRCENA mask
pub const CORE_DEBUG_DEMCR_TRCENA_MSK: u32 = 1 << CORE_DEBUG_DEMCR_TRCENA_POS;
/// CoreDebug DEMCR: MON_REQ position
pub const CORE_DEBUG_DEMCR_MON_REQ_POS: u32 = 19;
/// CoreDebug DEMCR: MON_REQ mask
pub const CORE_DEBUG_DEMCR_MON_REQ_MSK: u32 = 1 << CORE_DEBUG_DEMCR_MON_REQ_POS;
/// CoreDebug DEMCR: MON_STEP position
pub const CORE_DEBUG_DEMCR_MON_STEP_POS: u32 = 18;
/// CoreDebug DEMCR: MON_STEP mask
pub const CORE_DEBUG_DEMCR_MON_STEP_MSK: u32 = 1 << CORE_DEBUG_DEMCR_MON_STEP_POS;
/// CoreDebug DEMCR: MON_PEND position
pub const CORE_DEBUG_DEMCR_MON_PEND_POS: u32 = 17;
/// CoreDebug DEMCR: MON_PEND mask
pub const CORE_DEBUG_DEMCR_MON_PEND_MSK: u32 = 1 << CORE_DEBUG_DEMCR_MON_PEND_POS;
/// CoreDebug DEMCR: MON_EN position
pub const CORE_DEBUG_DEMCR_MON_EN_POS: u32 = 16;
/// CoreDebug DEMCR: MON_EN mask
pub const CORE_DEBUG_DEMCR_MON_EN_MSK: u32 = 1 << CORE_DEBUG_DEMCR_MON_EN_POS;
/// CoreDebug DEMCR: VC_HARDERR position
pub const CORE_DEBUG_DEMCR_VC_HARDERR_POS: u32 = 10;
/// CoreDebug DEMCR: VC_HARDERR mask
pub const CORE_DEBUG_DEMCR_VC_HARDERR_MSK: u32 = 1 << CORE_DEBUG_DEMCR_VC_HARDERR_POS;
/// CoreDebug DEMCR: VC_INTERR position
pub const CORE_DEBUG_DEMCR_VC_INTERR_POS: u32 = 9;
/// CoreDebug DEMCR: VC_INTERR mask
pub const CORE_DEBUG_DEMCR_VC_INTERR_MSK: u32 = 1 << CORE_DEBUG_DEMCR_VC_INTERR_POS;
/// CoreDebug DEMCR: VC_BUSERR position
pub const CORE_DEBUG_DEMCR_VC_BUSERR_POS: u32 = 8;
/// CoreDebug DEMCR: VC_BUSERR mask
pub const CORE_DEBUG_DEMCR_VC_BUSERR_MSK: u32 = 1 << CORE_DEBUG_DEMCR_VC_BUSERR_POS;
/// CoreDebug DEMCR: VC_STATERR position
pub const CORE_DEBUG_DEMCR_VC_STATERR_POS: u32 = 7;
/// CoreDebug DEMCR: VC_STATERR mask
pub const CORE_DEBUG_DEMCR_VC_STATERR_MSK: u32 = 1 << CORE_DEBUG_DEMCR_VC_STATERR_POS;
/// CoreDebug DEMCR: VC_CHKERR position
pub const CORE_DEBUG_DEMCR_VC_CHKERR_POS: u32 = 6;
/// CoreDebug DEMCR: VC_CHKERR mask
pub const CORE_DEBUG_DEMCR_VC_CHKERR_MSK: u32 = 1 << CORE_DEBUG_DEMCR_VC_CHKERR_POS;
/// CoreDebug DEMCR: VC_NOCPERR position
pub const CORE_DEBUG_DEMCR_VC_NOCPERR_POS: u32 = 5;
/// CoreDebug DEMCR: VC_NOCPERR mask
pub const CORE_DEBUG_DEMCR_VC_NOCPERR_MSK: u32 = 1 << CORE_DEBUG_DEMCR_VC_NOCPERR_POS;
/// CoreDebug DEMCR: VC_MMERR position
pub const CORE_DEBUG_DEMCR_VC_MMERR_POS: u32 = 4;
/// CoreDebug DEMCR: VC_MMERR mask
pub const CORE_DEBUG_DEMCR_VC_MMERR_MSK: u32 = 1 << CORE_DEBUG_DEMCR_VC_MMERR_POS;
/// CoreDebug DEMCR: VC_CORERESET position
pub const CORE_DEBUG_DEMCR_VC_CORERESET_POS: u32 = 0;
/// CoreDebug DEMCR: VC_CORERESET mask
pub const CORE_DEBUG_DEMCR_VC_CORERESET_MSK: u32 = 1 << CORE_DEBUG_DEMCR_VC_CORERESET_POS;

// ---------------------------------------------------------------------------
// Volatile register access wrappers
// ---------------------------------------------------------------------------

/// A read-write volatile register.
#[repr(transparent)]
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

/// A write-only volatile register.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VolatileWO<T: Copy> {
    value: T,
}

impl<T: Copy> VolatileWO<T> {
    /// Write a value to the register.
    #[inline]
    #[allow(invalid_reference_casting)]
    pub fn write(&self, val: T) {
        unsafe { ptr::write_volatile(&self.value as *const T as *mut T, val) }
    }
}

// ---------------------------------------------------------------------------
// Memory-mapped peripheral base addresses
// ---------------------------------------------------------------------------

/// System Control Space base address.
pub const SCS_BASE: u32 = 0xE000_E000;
/// ITM base address.
pub const ITM_BASE: u32 = 0xE000_0000;
/// DWT base address.
pub const DWT_BASE: u32 = 0xE000_1000;
/// TPI base address.
pub const TPI_BASE: u32 = 0xE004_0000;
/// Core Debug base address.
pub const CORE_DEBUG_BASE: u32 = 0xE000_EDF0;
/// SysTick base address.
pub const SYSTICK_BASE: u32 = SCS_BASE + 0x0010;
/// NVIC base address.
pub const NVIC_BASE: u32 = SCS_BASE + 0x0100;
/// SCB base address.
pub const SCB_BASE: u32 = SCS_BASE + 0x0D00;
/// MPU base address.
pub const MPU_BASE: u32 = SCS_BASE + 0x0D90;
/// FPU base address.
pub const FPU_BASE: u32 = SCS_BASE + 0x0F30;

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
pub fn nvic_set_priority_grouping(priority_group: u32) {
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
pub fn nvic_get_priority_grouping() -> u32 {
    (SCB.aircr.read() & SCB_AIRCR_PRIGROUP_MSK) >> SCB_AIRCR_PRIGROUP_POS
}

/// Enable an external interrupt in the NVIC.
///
/// `irqn` must be non-negative (device-specific interrupt).
#[inline]
pub fn nvic_enable_irq(irqn: IRQType) {
    let n = irqn as u32;
    NVIC.iser[(n >> 5) as usize].write(1 << (n & 0x1F));
}

/// Disable an external interrupt in the NVIC.
///
/// `irqn` must be non-negative (device-specific interrupt).
#[inline]
pub fn nvic_disable_irq(irqn: IRQType) {
    let n = irqn as u32;
    NVIC.icer[(n >> 5) as usize].write(1 << (n & 0x1F));
}

/// Get the pending status of an interrupt.
///
/// Returns `true` if the interrupt is pending.
#[inline]
pub fn nvic_get_pending_irq(irqn: IRQType) -> bool {
    let n = irqn as u32;
    (NVIC.ispr[(n >> 5) as usize].read() & (1 << (n & 0x1F))) != 0
}

/// Set an interrupt to pending.
#[inline]
pub fn nvic_set_pending_irq(irqn: IRQType) {
    let n = irqn as u32;
    NVIC.ispr[(n >> 5) as usize].write(1 << (n & 0x1F));
}

/// Clear the pending status of an interrupt.
#[inline]
pub fn nvic_clear_pending_irq(irqn: IRQType) {
    let n = irqn as u32;
    NVIC.icpr[(n >> 5) as usize].write(1 << (n & 0x1F));
}

/// Get the active status of an interrupt.
///
/// Returns `true` if the interrupt is active.
#[inline]
pub fn nvic_get_active(irqn: IRQType) -> bool {
    let n = irqn as u32;
    (NVIC.iabr[(n >> 5) as usize].read() & (1 << (n & 0x1F))) != 0
}

/// Set the priority of an interrupt.
///
/// Negative `irqn` values address system handler priorities in `SCB->SHP`.
/// Non-negative values address device-specific interrupts via `NVIC->IP`.
#[inline]
pub fn nvic_set_priority(irqn: IRQType, priority: u32) {
    let encoded = ((priority << (8 - NVIC_PRIO_BITS)) & 0xFF) as u8;
    if (irqn as i8) < 0 {
        let idx = ((irqn as u32) & 0xF).wrapping_sub(4) as usize;
        SCB.shp[idx].write(encoded);
    } else {
        NVIC.ip[irqn as usize].write(encoded);
    }
}

/// Get the priority of an interrupt.
///
/// Returns the priority value, right-shifted to the implemented bits.
#[inline]
pub fn nvic_get_priority(irqn: IRQType) -> u32 {
    if (irqn as i8) < 0 {
        let idx = ((irqn as u32) & 0xF).wrapping_sub(4) as usize;
        (SCB.shp[idx].read() as u32) >> (8 - NVIC_PRIO_BITS)
    } else {
        (NVIC.ip[irqn as usize].read() as u32) >> (8 - NVIC_PRIO_BITS)
    }
}

/// Encode a priority value from group, preempt, and sub-priority.
///
/// Returns the encoded priority suitable for [`nvic_set_priority`].
#[inline]
pub fn nvic_encode_priority(priority_group: u32, preempt_priority: u32, sub_priority: u32) -> u32 {
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
pub fn nvic_decode_priority(priority: u32, priority_group: u32) -> (u32, u32) {
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
pub fn systick_config(ticks: u32) -> Result<(), ReloadValueTooLarge> {
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
pub const ITM_RXBUFFER_EMPTY: i32 = 0x5AA5_5AA5u32 as i32;

/// Transmit a character via ITM stimulus port 0.
///
/// Returns the character. If no debugger is connected the function returns
/// immediately. If a debugger is connected it blocks until the port is ready.
#[inline]
pub fn itm_send_char(ch: u32) -> u32 {
    if (ITM.tcr.read() & ITM_TCR_ITMENA_MSK) != 0 && (ITM.ter.read() & 1) != 0 {
        while unsafe { ITM.port[0].u32.read_like_ro() } == 0 {}
        unsafe { ITM.port[0].u8.write(ch as u8) };
    }
    ch
}

// Internal helper: read through a write-only port (the hardware actually
// supports reads on stimulus ports to check readiness).
impl VolatileWO<u32> {
    /// Read the register value (used for ITM stimulus port readiness check).
    #[inline]
    unsafe fn read_like_ro(&self) -> u32 {
        unsafe { ptr::read_volatile(&self.value) }
    }
}
