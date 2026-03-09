use crate::{analog_read, analog_write, peripherals::regs::Lcd, toggle_reg_bits};
#[repr(usize)]
pub enum LcdDuty {
    Static = 0x0,
    Half = 0x1,
    Third = 0x2,
    Quarter = 0x3,
    Reserved = 0x7,
}

#[repr(usize)]
pub enum LcdComUse {
    One = 0x1,
    Two = 0x2,
    Three = 0x3,
    Four = 0x4,
}

#[repr(usize)]
pub enum LcdDriveMode {
    SmallCurrent = 0x0,
    LargeCurrent = 0x1,
    SmallCurrentBuffer = 0x2,
    LargeCurrentBuffer = 0x3,
}

#[repr(usize)]
pub enum LcdBias {
    Quarter = 0x0,
    Third = 0x8,
    Half = 0x10,
    Static = 0x18,
}

#[repr(usize)]
pub enum LcdEnable {
    Enabled = 0x20,
}

#[repr(usize)]
pub enum LcdPrescaler {
    Div1 = 0x0,
    Div2 = 0x1,
    Div4 = 0x2,
    Div8 = 0x3,
    Div16 = 0x4,
    Div32 = 0x5,
    Div64 = 0x6,
    Div128 = 0x7,
    Div256 = 0x8,
    Div512 = 0x9,
    Div1024 = 0xa,
    Div2048 = 0xb,
    Div4096 = 0xc,
    Div8192 = 0xd,
    Div16384 = 0xe,
    Div32768 = 0xf,
}

#[repr(usize)]
pub enum LcdDivision {
    Div16 = 0x0,
    Div17 = 0x10,
    Div18 = 0x20,
    Div19 = 0x30,
    Div20 = 0x40,
    Div21 = 0x50,
    Div22 = 0x60,
    Div23 = 0x70,
    Div24 = 0x80,
    Div25 = 0x90,
    Div26 = 0xa0,
    Div27 = 0xb0,
    Div28 = 0xc0,
    Div29 = 0xd0,
    Div30 = 0xe0,
    Div31 = 0xf0,
}

#[repr(usize)]
pub enum LcdBlinkFreq {
    Div8 = 0x0,
    Div16 = 0x100,
    Div32 = 0x200,
    Div64 = 0x300,
    Div128 = 0x400,
    Div256 = 0x500,
    Div512 = 0x600,
    Div1024 = 0x700,
}

#[repr(usize)]
pub enum LcdBlinkSel {
    Disabled = 0x0,
    Seg0Com0 = 0x800,
    Seg0AllCom = 0x1000,
    AllSegCom = 0x1800,
}

#[repr(usize)]
pub enum LcdDeadCycle {
    Zero = 0x0,
    One = 0x2000,
    Two = 0x4000,
    Three = 0x6000,
    Four = 0x8000,
    Five = 0xa000,
    Six = 0xc000,
    Seven = 0xe000,
}

#[repr(usize)]
pub enum LcdSwitchEnable {
    Enabled = 0x10000,
}

#[repr(usize)]
pub enum LcdLargeCurrentNum {
    Zero = 0x0,
    One = 0x20000,
    Two = 0x40000,
    Three = 0x60000,
    Four = 0x80000,
    Five = 0xa0000,
    Six = 0xc0000,
    Seven = 0xe0000,
}

#[repr(usize)]
pub enum LcdComNum {
    Com0,
    Com1,
    Com2,
    Com3,
}

#[repr(usize)]
pub enum LcdComSegMask {
    Com0 = 0x7ffffff,
    Com1 = 0x3ffffff,
    Com2 = 0x1ffffff,
    Com3 = 0xffffff,
}

#[repr(usize)]
pub enum LcdStatus {
    Cr0Done = 0x1,
    Cr1Done = 0x2,
}

#[repr(usize)]
pub enum LcdInterrupt {
    EvenFrameDone = 0x2,
}

#[repr(usize)]
pub enum LcdInterruptStatus {
    EvenFrameDone = 0x1,
}

impl Lcd {
    pub fn init(
        &self,
        duty: LcdDuty,
        bias: LcdBias,
        prescaler: LcdPrescaler,
        division: LcdDivision,
    ) {
        self.config_duty(duty);
        self.config_bias(bias);
        self.config_prescaler(prescaler);
        self.config_division_clock(division);
    }

    pub fn enable(&self, enable_flag: bool) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr0, LcdEnable::Enabled as usize, enable_flag);
    }

    pub fn config_duty(&self, duty: LcdDuty) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr0, LcdDuty::Reserved as usize, false);
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr0, duty as usize, true);
    }

    pub fn config_bias(&self, bias: LcdBias) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr0, LcdBias::Static as usize, false);
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr0, bias as usize, true);
    }

    pub fn config_prescaler(&self, prescaler: LcdPrescaler) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, LcdPrescaler::Div32768 as usize, false);
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, prescaler as usize, true);
    }

    pub fn config_division_clock(&self, division: LcdDivision) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, LcdDivision::Div31 as usize, false);
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, division as usize, true);
    }

    pub fn config_blink_freq(&self, blink_freq: LcdBlinkFreq) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, LcdBlinkFreq::Div1024 as usize, false);
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, blink_freq as usize, true);
    }

    pub fn config_blink_sel(&self, blink_sel: LcdBlinkSel) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, LcdBlinkSel::AllSegCom as usize, false);
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, blink_sel as usize, true);
    }

    pub fn config_dead_cycle(&self, dead_cycle: LcdDeadCycle) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, LcdDeadCycle::Seven as usize, false);
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, dead_cycle as usize, true);
    }

    pub fn enable_switch(&self, enable_flag: bool) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, LcdSwitchEnable::Enabled as usize, enable_flag);
    }

    pub fn config_large_current_num(&self, num: LcdLargeCurrentNum) {
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, LcdLargeCurrentNum::Seven as usize, false);
        while !self.check_sync_done() {}
        toggle_reg_bits!(self.cr1, num as usize, true);
    }

    pub fn write_com_seg_bit(&self, com: LcdComNum, bit_pos: u8, value: u8) {
        let bit_mask = 1usize << bit_pos;
        let bit_value = (value as usize) << bit_pos;

        while !self.check_sync_done() {}
        let dr = match com {
            LcdComNum::Com0 => &self.dr0,
            LcdComNum::Com1 => &self.dr1,
            LcdComNum::Com2 => &self.dr2,
            LcdComNum::Com3 => &self.dr3,
        };
        toggle_reg_bits!(dr, bit_mask, false);
        while !self.check_sync_done() {}
        toggle_reg_bits!(dr, bit_value, true);
    }

    pub fn clear_com_seg_state(&self, com: LcdComNum) {
        while !self.check_sync_done() {}

        let (dr, mask) = match com {
            LcdComNum::Com0 => (&self.dr0, LcdComSegMask::Com0 as usize),
            LcdComNum::Com1 => (&self.dr1, LcdComSegMask::Com1 as usize),
            LcdComNum::Com2 => (&self.dr2, LcdComSegMask::Com2 as usize),
            LcdComNum::Com3 => (&self.dr3, LcdComSegMask::Com3 as usize),
        };
        toggle_reg_bits!(dr, mask, false);
    }

    pub fn check_sync_done(&self) -> bool {
        self.sr.read() & (LcdStatus::Cr0Done as usize) == (LcdStatus::Cr0Done as usize)
            && self.sr.read() & (LcdStatus::Cr1Done as usize) == (LcdStatus::Cr1Done as usize)
    }

    pub fn config_drive_mode(&self, mode: LcdDriveMode) {
        let read_data = analog_read!(0xb) & !(0x3 << 3);
        analog_write!(0xb, read_data);
        analog_write!(0xb, read_data | ((mode as usize) << 3));
    }

    pub fn config_interrupt(&self, enable_flag: bool) {
        toggle_reg_bits!(self.cr2, LcdInterrupt::EvenFrameDone as usize, enable_flag);
    }

    pub fn get_interrupt_status(&self) -> bool {
        self.cr2.read() & (LcdInterrupt::EvenFrameDone as usize)
            == (LcdInterruptStatus::EvenFrameDone as usize)
    }

    pub fn clear_interrupt_status(&self) {
        toggle_reg_bits!(self.cr2, LcdInterrupt::EvenFrameDone as usize, true);
    }

    pub fn enable_com(&self, com_use: LcdComUse) {
        let com = com_use as usize;
        let read_data = analog_read!(0x9) & !(0x7 << 1);
        analog_write!(0x9, read_data);
        analog_write!(0x9, read_data | ((com - 1) << 1));

        let mut read_data = analog_read!(0xb);
        for i in (31 - (com - 1) + 1..=31).rev() {
            read_data |= 0x1 << i;
        }
        analog_write!(0xb, read_data);

        analog_write!(0xa, analog_read!(0xa) | (0x1 << 22));
    }

    pub fn enable_seg(&self, seg_begin: u8, seg_num: u8) {
        let mut bit_sel = 1 << (seg_begin + 5);
        for i in (seg_begin + 1)..(seg_begin + seg_num) {
            bit_sel |= 1 << (i + 5);
        }
        analog_write!(0xb, analog_read!(0xb) | bit_sel);
    }

    pub fn enable_analog(&self) {
        analog_write!(0x6, analog_read!(0x6) & !(0x1 << 7));
    }
}
