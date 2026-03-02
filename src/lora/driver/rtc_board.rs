use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU16, AtomicU32, Ordering};

use crate::{
    cortex::{IRQType, nvic_enable_irq},
    peripherals::{
        pwr::PWR_LP_MODE_STOP3,
        rcc::{RCC_PERIPHERAL_RTC, RCC_RTC_CLK_SOURCE_RCO32K},
        regs::{PWR, RCC, Rtc},
    },
    toggle_reg_bits,
};

pub const SECONDS_IN_MINUTE: u64 = 60;
pub const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;
pub const SECONDS_IN_DAY: u64 = 24 * SECONDS_IN_HOUR;
pub const SECONDS_IN_LEAP_YEAR: u64 = 366 * SECONDS_IN_DAY;
pub const SECONDS_IN_NON_LEAP_YEAR: u64 = 365 * SECONDS_IN_DAY;
pub const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const DAYS_IN_MONTH_LEAP_YEAR: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub const RTC_MICROSECOND: u32 = 1000000;
pub const RTC_ENABLE_TIME: u32 = 0x10000000;
pub const RTC_ENABLE_ALARM: u32 = 0x80000000;

#[repr(u32)]
pub enum RtcAlarmWakeup {
    Alarm0WkEn = 0x8000000,
    Alarm1WkEn = 0x4000000,
}

#[repr(u32)]
pub enum RtcCycControl {
    CycWkEn = 0x2000000,
    CycCounter = 0x1000000,
}

#[repr(u32)]
pub enum RtcTamperControl {
    Tamper = 0x800000,
    TamperLevel = 0x400000,
    TamperWkEn0 = 0x200000,
    TamperWkEn1 = 0x100000,
}

pub enum RtcWakeupControl {
    Wakeup0 = 0x20000,
    Wakeup0Level = 0x10000,
    Wakeup0Wken0 = 0x8000,
    Wakeup0Wken1 = 0x4000,
    Wakeup1 = 0x800,
    Wakeup1Level = 0x400,
    Wakeup1Wken0 = 0x200,
    Wakeup1Wken1 = 0x100,
    Wakeup2 = 0x20,
    Wakeup2Level = 0x10,
    Wakeup2Wken0 = 0x8,
    Wakeup2Wken1 = 0x4,
}

#[repr(u32)]
pub enum RtcFilter {
    NoFilter,
    Filter1,
    Filter3,
    Filter7,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum RtcStatus {
    Alarm0Sr = 0x40,
    Alarm1Sr = 0x20,
    CycSr = 0x10,
    TamperSr = 0x8,
    Wakeup0Sr = 0x4,
    Wakeup1Sr = 0x2,
    Wakeup2Sr = 0x1,
}

#[repr(u32)]
pub enum RtcInterruptFlag {
    Sec = 0x80,
    Alarm0 = 0x40,
    Alarm1 = 0x20,
    Cyc = 0x10,
    Tamper = 0x8,
    Wakeup0 = 0x4,
    Wakeup1 = 0x2,
    Wakeup2 = 0x1,
}

pub struct RtcCalendar {
    pub year: AtomicU16,
    pub week: AtomicU8,
    pub month: AtomicU8,
    pub day: AtomicU8,
    pub hour: AtomicU8,
    pub minute: AtomicU8,
    pub second: AtomicU8,
    pub subsecond: AtomicU32,
}

pub struct RtcAlarmMask {
    pub day_mask: bool,
    pub week_mask: bool,
    pub hour_mask: bool,
    pub minute_mask: bool,
    pub second_mask: bool,
    pub subsecond_mask: bool,
}

#[repr(u32)]
pub enum RtcIoLevel {
    NoInvert = 0x0,
    Invert = 0x80,
}

#[repr(u32)]
pub enum RtcIo {
    LowLevel = 0x30,
    Alarm0 = 0x40,
    Alarm1 = 0x50,
    Cyc = 0x60,
    Sec = 0x70,
}

#[repr(u32)]
pub enum RtcRetSramErase {
    Wakeup0Enable = 0x1,
    Wakeup1Enable = 0x2,
    Wakeup2Enable = 0x4,
    TamperEnable = 0x8,
}

pub const RTC_ALARM_MAX_NUM: u32 = 2;
pub const RTC_WAKEUP_MAX_NUM: u32 = 3;

static RTC_CALENDAR_CTX: RtcCalendar = RtcCalendar {
    year: AtomicU16::new(0),
    week: AtomicU8::new(0),
    month: AtomicU8::new(0),
    day: AtomicU8::new(0),
    hour: AtomicU8::new(0),
    minute: AtomicU8::new(0),
    second: AtomicU8::new(0),
    subsecond: AtomicU32::new(0),
};

static mut RTC_TIMER_CTX: u64 = 0;
static RTC_TIMER_EVENT_ALLOWS_LOW_POWER: AtomicBool = AtomicBool::new(false);
static LOW_POWER_DISABLE_DURING_TASK: AtomicBool = AtomicBool::new(false);
static RTC_INITIALIZED: AtomicBool = AtomicBool::new(false);
static MCU_WAKEUP_TIME: AtomicU32 = AtomicU32::new(0);
static NON_SCHEDULED_WAKE_UP: AtomicBool = AtomicBool::new(false);

impl Rtc {
    /// Initializes the RTC timer.
    ///
    /// The timer is based on the RTC peripheral.
    /// TODO: maybe use local variables instead of statics
    pub fn init(&self) {
        if RTC_INITIALIZED.load(Ordering::Relaxed) {
            self.deinit();
            RCC.enable_peripheral_clk(RCC_PERIPHERAL_RTC, true);

            self.calendar_cmd(true);
            nvic_enable_irq(IRQType::Rtc);

            RTC_INITIALIZED.store(true, Ordering::Relaxed);
        }
    }

    /// Starts the RTC timer.
    ///
    /// The timer is based on the RTC cyc counter running from the RTC clock.
    ///
    /// # Parameters
    /// - `timeout`: Duration of the timer in milliseconds.
    pub fn set_timeout(&self, timeout: u32) {
        self.start_wakeup_alarm(timeout);
    }

    /// Stops the RTC timeout source.
    pub fn stop_timeout(&self) {
        self.cyc_cmd(false);
        self.config_interrupt(RtcInterruptFlag::Cyc, false);
    }

    /// Gets the current RTC timer context.
    pub fn get_timer_ctx(&self) -> u64 {
        unsafe { RTC_TIMER_CTX }
    }

    /// Sets the RTC timer context to the current timer value.
    ///
    /// # Returns
    /// The newly stored timer context.
    pub fn set_timer_ctx(&self) -> u64 {
        unsafe {
            RTC_TIMER_CTX = self.get_timer_value();
            RTC_TIMER_CTX
        }
    }

    /// Gets the elapsed RTC time since the last timer context was set.
    pub fn get_elapsed_time(&self) -> u64 {
        self.get_timer_value() - unsafe { RTC_TIMER_CTX }
    }

    /// Adjusts a timeout value to compensate for wake-up latency.
    ///
    /// Handles compensation for alarm/GPIO wake-up overhead and determines
    /// whether the next timer event is long enough to allow low-power mode.
    ///
    /// # Parameters
    /// - `timeout`: Timeout duration without wake-up compensation, in milliseconds.
    ///
    /// # Returns
    /// Timeout value with applied compensation.
    pub fn get_adjusted_timeout_value(&self, mut timeout: u32) -> u32 {
        let mcu_wakeup_time = MCU_WAKEUP_TIME.load(Ordering::Relaxed);
        let non_scheduled_wakeup = NON_SCHEDULED_WAKE_UP.load(Ordering::Relaxed);
        if timeout > mcu_wakeup_time && non_scheduled_wakeup {
            NON_SCHEDULED_WAKE_UP.store(false, Ordering::Relaxed);
            timeout -= mcu_wakeup_time;
        }

        if timeout > mcu_wakeup_time {
            if timeout < 50 {
                RTC_TIMER_EVENT_ALLOWS_LOW_POWER.store(false, Ordering::Relaxed);
            } else {
                RTC_TIMER_EVENT_ALLOWS_LOW_POWER.store(true, Ordering::Relaxed);
                timeout -= mcu_wakeup_time;
            }
        }

        timeout
    }

    /// Gets the current RTC timer value.
    pub fn get_timer_value(&self) -> u64 {
        self.convert_calendar_tick_to_time(None)
    }

    /// Blocks or unblocks low-power mode during a task.
    ///
    /// # Parameters
    /// - `status`: `true` to block low-power mode, `false` to allow it.
    pub fn block_low_power_during_task(&self, status: bool) {
        if status {
            self.recover_mcu_status();
        }
        LOW_POWER_DISABLE_DURING_TASK.store(status, Ordering::Relaxed);
    }

    /// Enters MCU low-power STOP mode.
    pub fn enter_low_power_stop_mode(&self) {
        // if(Radio.GetStatus() != RF_IDLE)
        // return;
        self.check_syn();
        PWR.deepsleep_wfi(PWR_LP_MODE_STOP3);
    }

    /// Restores MCU status after low-power mode.
    pub fn recover_mcu_status(&self) {}

    /// Starts the RTC wake-up alarm.
    ///
    /// # Parameters
    /// - `timeout`: Wake-up timeout in milliseconds.
    pub fn start_wakeup_alarm(&self, mut timeout: u32) {
        if timeout < 5 {
            timeout = 5;
        }

        let now = self.get_calendar();

        // AAAAAAAAAAAAAAAAAAHHHHHHHHHHHHHHHHHHHHHHHH
        RTC_CALENDAR_CTX
            .year
            .store(now.year.load(Ordering::Relaxed), Ordering::Relaxed);
        RTC_CALENDAR_CTX
            .week
            .store(now.week.load(Ordering::Relaxed), Ordering::Relaxed);
        RTC_CALENDAR_CTX
            .month
            .store(now.month.load(Ordering::Relaxed), Ordering::Relaxed);
        RTC_CALENDAR_CTX
            .day
            .store(now.day.load(Ordering::Relaxed), Ordering::Relaxed);
        RTC_CALENDAR_CTX
            .hour
            .store(now.hour.load(Ordering::Relaxed), Ordering::Relaxed);
        RTC_CALENDAR_CTX
            .minute
            .store(now.minute.load(Ordering::Relaxed), Ordering::Relaxed);
        RTC_CALENDAR_CTX
            .second
            .store(now.second.load(Ordering::Relaxed), Ordering::Relaxed);
        RTC_CALENDAR_CTX
            .subsecond
            .store(now.subsecond.load(Ordering::Relaxed), Ordering::Relaxed);

        self.cyc_cmd(false);
        self.config_cyc_max(self.convert_ms_to_tick(timeout as u64) as u32);
        self.config_cyc_wakeup(true);
        self.cyc_cmd(true);
        self.config_interrupt(RtcInterruptFlag::Cyc, true);
    }

    /// Converts an RTC calendar tick value into timer time in milliseconds.
    ///
    /// # Parameters
    /// - `calendar`: Optional calendar source.
    ///   - `None`: compute from current RTC time
    ///   - `Some(...)`: compute from the provided calendar
    ///
    /// # Returns
    /// Converted timer time value in milliseconds.
    pub fn convert_calendar_tick_to_time(&self, calendar: Option<&RtcCalendar>) -> u64 {
        let now = if let Some(calendar) = calendar {
            calendar
        } else {
            &self.get_calendar()
        };

        let year = now.year.load(Ordering::Relaxed);
        let mut total_time = 0;
        // slow, maybe calculate the number of leap years instead of using this loop
        for i in 0..(year - 2000) {
            total_time += if i == 0 || i % 4 == 0 {
                SECONDS_IN_LEAP_YEAR
            } else {
                SECONDS_IN_NON_LEAP_YEAR
            };
        }
        let month = now.month.load(Ordering::Relaxed);

        for i in 0..month - 1 {
            total_time += if i == 1 && year % 4 == 0 {
                DAYS_IN_MONTH_LEAP_YEAR[i as usize] as u64 * SECONDS_IN_DAY
            } else {
                DAYS_IN_MONTH[i as usize] as u64 * SECONDS_IN_DAY
            };
        }

        total_time += now.second.load(Ordering::Relaxed) as u64
            + now.minute.load(Ordering::Relaxed) as u64 * SECONDS_IN_MINUTE
            + now.hour.load(Ordering::Relaxed) as u64 * SECONDS_IN_HOUR
            + (now.day.load(Ordering::Relaxed) as u64 - 1) * SECONDS_IN_DAY;

        total_time * 1000 + now.subsecond.load(Ordering::Relaxed) as u64 / 1000
    }

    /// Converts milliseconds to RTC ticks.
    ///
    /// # Parameters
    /// - `ticks`: Timeout value in milliseconds.
    ///
    /// # Returns
    /// RTC tick count.
    pub fn convert_ms_to_tick(&self, ticks: u64) -> u64 {
        let tps = if RCC.get_rtc_clk_src() == RCC_RTC_CLK_SOURCE_RCO32K {
            32000
        } else {
            32768
        };

        // TODO: maybe use float rounding
        (ticks * tps + 500/* round to the nearest integer */) / 1000
    }

    /// RTC IRQ handler for RTC cyc events.
    pub fn on_irq(&self) {
        if self.get_status(RtcStatus::CycSr) {
            self.cyc_cmd(false);
            self.config_interrupt(RtcInterruptFlag::Cyc, false);
            self.set_status(RtcStatus::CycSr, false);

            // TODO
            // TimerIrqHandler();

            self.config_interrupt(RtcInterruptFlag::Cyc, true);
        }
    }

    /// Processes pending RTC timer events.
    ///
    /// Not used on this platform.
    pub fn process() {
        // not used on this platform
    }

    /// Deinitializes the RTC peripheral.
    pub fn deinit(&self) {
        RCC.enable_peripheral_clk(RCC_PERIPHERAL_RTC, false);
        RCC.rst_peripheral(RCC_PERIPHERAL_RTC, true);
        RCC.rst_peripheral(RCC_PERIPHERAL_RTC, false);
    }

    /// Checks RTC synchronization status.
    pub fn check_syn(&self) {
        loop {
            if (self.sr1.read() & 0xdff) == 0xdff {
                break;
            }
        }
    }

    /// Enables or disables the RTC calendar.
    ///
    /// # Parameters
    /// - `enable`: `true` to enable, `false` to disable.
    pub fn calendar_cmd(&self, enable: bool) {
        self.check_syn();
        toggle_reg_bits!(self.ctrl, RTC_ENABLE_TIME, enable);
    }

    /// Enables or disables an RTC alarm.
    ///
    /// # Parameters
    /// - `alarm_idx`: Alarm index.
    /// - `enable`: `true` to enable, `false` to disable.
    pub fn alarm_cmd(&self, alarm_idx: u8, enable: bool) {
        self.check_syn();
        match alarm_idx {
            0 => toggle_reg_bits!(self.alarm0, RTC_ENABLE_ALARM, enable),
            1 => toggle_reg_bits!(self.alarm1, RTC_ENABLE_ALARM, enable),
            _ => panic!("Invalid alarm index"),
        }
    }

    /// Configures alarm wake-up enable state.
    ///
    /// # Parameters
    /// - `alarm_idx`: Alarm index.
    /// - `state`: `true` to enable wake-up, `false` to disable.
    pub fn config_alarm_wakeup(&self, alarm_idx: u8, state: bool) {
        let alarm_wakeup = match alarm_idx {
            0 => RtcAlarmWakeup::Alarm0WkEn,
            1 => RtcAlarmWakeup::Alarm1WkEn,
            _ => panic!("Invalid alarm index"),
        };

        self.check_syn();
        toggle_reg_bits!(self.ctrl, alarm_wakeup as u32, state);
    }

    /// Enables or disables the RTC cyc counter.
    pub fn cyc_cmd(&self, enable: bool) {
        self.check_syn();
        toggle_reg_bits!(self.ctrl, RtcCycControl::CycCounter as u32, enable);
    }

    /// Configures RTC cyc wake-up.
    pub fn config_cyc_wakeup(&self, state: bool) {
        self.check_syn();
        toggle_reg_bits!(self.ctrl, RtcCycControl::CycWkEn as u32, state);
    }

    /// Enables or disables RTC tamper detection.
    pub fn tamper_cmd(&self, enable: bool) {
        self.check_syn();
        toggle_reg_bits!(self.ctrl, RtcTamperControl::Tamper as u32, enable);
    }

    /// Configures tamper high-level trigger.
    pub fn config_tamper_high_level(&self, state: bool) {
        self.check_syn();
        toggle_reg_bits!(self.ctrl, RtcTamperControl::TamperLevel as u32, state);
    }

    /// Configures tamper level wake-up.
    pub fn config_tamper_level_wakeup(&self, state: bool) {
        self.check_syn();
        toggle_reg_bits!(self.ctrl, RtcTamperControl::TamperWkEn0 as u32, state);
    }

    /// Configures tamper wake-up.
    pub fn config_tamper_wakeup(&self, state: bool) {
        self.check_syn();
        toggle_reg_bits!(self.ctrl, RtcTamperControl::TamperWkEn1 as u32, state);
    }

    /// Configures tamper filter type.
    pub fn config_tamper_filter(&self, filter: RtcFilter) {
        self.check_syn();
        toggle_reg_bits!(self.ctrl, (RtcFilter::Filter7 as u32) << 18, false);
        self.check_syn();
        toggle_reg_bits!(self.ctrl, (filter as u32) << 18, true);
    }

    /// Enables or disables a wake-up IO source.
    pub fn wakeup_io_cmd(&self, wakeup_io_idx: u8, state: bool) {
        let wakeup = match wakeup_io_idx {
            0 => RtcWakeupControl::Wakeup0 as u32,
            1 => RtcWakeupControl::Wakeup1 as u32,
            2 => RtcWakeupControl::Wakeup2 as u32,
            _ => panic!("Invalid wakeup io index"),
        };
        self.check_syn();
        toggle_reg_bits!(self.ctrl, wakeup, state);
    }

    /// Configures wake-up IO high-level trigger.
    pub fn config_wakeup_io_high_level(&self, wakeup_io_idx: u8, state: bool) {
        let wakeup_level = match wakeup_io_idx {
            0 => RtcWakeupControl::Wakeup0Level as u32,
            1 => RtcWakeupControl::Wakeup1Level as u32,
            2 => RtcWakeupControl::Wakeup2Level as u32,
            _ => panic!("Invalid wakeup io index"),
        };
        self.check_syn();
        toggle_reg_bits!(self.ctrl, wakeup_level, state);
    }

    /// Configures wake-up IO level wake-up behavior.
    pub fn config_wakeup_io_level_wakeup(&self, wakeup_io_idx: u8, state: bool) {
        let wakeup_wken = match wakeup_io_idx {
            0 => RtcWakeupControl::Wakeup0Wken0 as u32,
            1 => RtcWakeupControl::Wakeup1Wken0 as u32,
            2 => RtcWakeupControl::Wakeup2Wken0 as u32,
            _ => panic!("Invalid wakeup io index"),
        };
        self.check_syn();
        toggle_reg_bits!(self.ctrl, wakeup_wken, state);
    }

    /// Configures wake-up IO wake-up behavior.
    pub fn config_wakeup_io_wakeup(&self, wakeup_io_idx: u8, state: bool) {
        let wakeup_wken = match wakeup_io_idx {
            0 => RtcWakeupControl::Wakeup0Wken1 as u32,
            1 => RtcWakeupControl::Wakeup1Wken1 as u32,
            2 => RtcWakeupControl::Wakeup2Wken1 as u32,
            _ => panic!("Invalid wakeup io index"),
        };
        self.check_syn();
        toggle_reg_bits!(self.ctrl, wakeup_wken, state);
    }

    /// Configures wake-up IO filter type.
    pub fn config_wakeup_io_filter(&self, wakeup_idx: u8, filter: RtcFilter) {
        self.check_syn();
        match wakeup_idx {
            0 => {
                toggle_reg_bits!(self.ctrl, (RtcFilter::Filter7 as u32) << 12, false);
                self.check_syn();
                toggle_reg_bits!(self.ctrl, (filter as u32) << 12, true);
            }
            1 => {
                toggle_reg_bits!(self.ctrl, (RtcFilter::Filter7 as u32) << 6, false);
                self.check_syn();
                toggle_reg_bits!(self.ctrl, (filter as u32) << 6, true);
            }
            _ => {
                toggle_reg_bits!(self.ctrl, RtcFilter::Filter7 as u32, false);
                self.check_syn();
                toggle_reg_bits!(self.ctrl, filter as u32, true);
            }
        }
    }

    /// Sets RTC calendar values.
    ///
    /// # Parameters
    /// - `calendar`: Calendar settings to write to RTC.
    pub fn set_calendar(&self, calendar: &RtcCalendar) {
        let year = calendar.year.load(Ordering::Relaxed) as u32;
        let week = calendar.week.load(Ordering::Relaxed) as u32;
        let month = calendar.month.load(Ordering::Relaxed) as u32;
        let day = calendar.day.load(Ordering::Relaxed) as u32;
        let hour = calendar.hour.load(Ordering::Relaxed) as u32;
        let minute = calendar.minute.load(Ordering::Relaxed) as u32;
        let second = calendar.second.load(Ordering::Relaxed) as u32;
        if year > 2099
            || week > 7
            || month > 12
            || day > 31
            || hour > 39
            || minute > 60
            || second > 60
        {
            panic!("Invalid calendar");
        }
        let year = year - 2000;
        self.check_syn();
        self.calendar_h.write(
            ((year / 10) << 18)
                | ((year % 10) << 14)
                | ((week) << 11)
                | ((month / 10) << 10)
                | ((month % 10) << 6)
                | ((day / 10) << 4)
                | (day % 10),
        );
        self.check_syn();
        self.calendar.write(
            ((hour / 10) << 18)
                | ((hour % 10) << 14)
                | ((minute / 10) << 11)
                | ((minute % 10) << 7)
                | ((second / 10) << 4)
                | (second % 10),
        );
    }

    /// Gets RTC subsecond counter value.
    pub fn get_subsecond_count(&self) -> u32 {
        self.sub_second_cnt.read()
    }

    /// Gets RTC cyc counter value.
    pub fn get_cyc_cnt(&self) -> u32 {
        // this is how it's done in c, idk what but there should be a reason for doing it this way
        let mut cyc_count;

        loop {
            cyc_count = self.cyc_cnt.read();
            if cyc_count == self.cyc_cnt.read() {
                break cyc_count;
            }
        }
    }

    /// Gets RTC calendar value.
    pub fn get_calendar(&self) -> RtcCalendar {
        let subsecond_cnt;
        let mut syn_data;
        let mut syn_data_h;

        loop {
            let cnt = self.get_subsecond_count();
            loop {
                let data = self.calendar.read();
                if data == self.calendar.read() {
                    syn_data = data;
                    break;
                }
            }
            loop {
                let data_h = self.calendar_h.read();
                if data_h == self.calendar_h.read() {
                    syn_data_h = data_h;
                    break;
                }
            }
            let cnt2 = self.get_subsecond_count();
            if cnt == cnt2 && cnt >= 1 {
                subsecond_cnt = cnt;
                break;
            }
        }

        let subsecond = ((RTC_MICROSECOND as f32 / 32768.0) * subsecond_cnt as f32 + 0.5) as u32;

        let second = (syn_data & 0x0F) + ((syn_data >> 4) & 0x07) * 10;
        let minute = ((syn_data >> 7) & 0x0F) + ((syn_data >> 11) & 0x07) * 10;
        let hour = ((syn_data >> 14) & 0x0F) + ((syn_data >> 18) & 0x03) * 10;

        let day = (syn_data_h & 0x0F) + ((syn_data_h >> 4) & 0x03) * 10;
        let month = ((syn_data_h >> 6) & 0x0F) + ((syn_data_h >> 10) & 0x01) * 10;
        let week = (syn_data_h >> 11) & 0x07;
        let year = 2000 + ((syn_data_h >> 14) & 0x0F) + ((syn_data_h >> 18) & 0x0F) * 10;

        RtcCalendar {
            year: AtomicU16::new(year as u16),
            week: AtomicU8::new(week as u8),
            month: AtomicU8::new(month as u8),
            day: AtomicU8::new(day as u8),
            hour: AtomicU8::new(hour as u8),
            minute: AtomicU8::new(minute as u8),
            second: AtomicU8::new(second as u8),
            subsecond: AtomicU32::new(subsecond),
        }
    }

    /// Sets/clears an RTC status flag.
    ///
    /// # Parameters
    /// - `status`: Status flag to update.
    /// - `set`: When `false`, clears the status flag.
    pub fn set_status(&self, status: RtcStatus, set: bool) {
        self.check_syn();
        if !set {
            self.sr.write(status as u32);
        }
    }

    /// Reads an RTC status flag.
    pub fn get_status(&self, status: RtcStatus) -> bool {
        self.check_syn();
        self.sr.read() & (status as u32) == status as u32
    }

    /// Clears second interrupt status.
    pub fn clear_sec_interrupt_status(&self) {
        toggle_reg_bits!(self.sr, 0x200, true);
    }

    /// Enables or disables an RTC interrupt source.
    pub fn config_interrupt(&self, interrupt: RtcInterruptFlag, enable: bool) {
        toggle_reg_bits!(self.cr1, interrupt as u32, enable);
    }

    /// Sets an RTC alarm.
    ///
    /// # Parameters
    /// - `alarm_index`: Alarm index.
    /// - `alarm_mask`: Alarm mask configuration.
    /// - `time`: Alarm time settings.
    pub fn set_alarm(&self, alarm_index: u8, alarm_mask: &RtcAlarmMask, time: &RtcCalendar) {
        if alarm_index >= RTC_ALARM_MAX_NUM as u8 {
            return;
        }

        self.alarm_cmd(alarm_index, false);

        let subsecond = time.subsecond.load(Ordering::Relaxed);
        let subsec_mask = alarm_mask.subsecond_mask;

        if subsec_mask && subsecond >= RTC_MICROSECOND {
            return;
        }

        if subsec_mask {
            let temp = (subsecond as f32 / (RTC_MICROSECOND as f32 / 32768.0) + 0.5) as u16;
            self.check_syn();
            match alarm_index {
                0 => {
                    let val = self.alarm0_subsecond.read() & 0xFFFF0000;
                    self.alarm0_subsecond.write(val | temp as u32);
                }
                _ => {
                    let val = self.alarm1_subsecond.read() & 0xFFFF0000;
                    self.alarm1_subsecond.write(val | temp as u32);
                }
            }
            self.check_syn();
            if temp != 0 {
                // subsec_mask here would need to be a u32 mask value; skipping as original C used a field
            }
        }

        let day = time.day.load(Ordering::Relaxed);
        let week = time.week.load(Ordering::Relaxed);
        let hour = time.hour.load(Ordering::Relaxed);
        let minute = time.minute.load(Ordering::Relaxed);
        let second = time.second.load(Ordering::Relaxed);

        if alarm_mask.day_mask && alarm_mask.week_mask {
            return;
        }

        let mut day_or_week_mask_value: u32 = 1;
        let mut hr_mask_value: u32 = 1;
        let mut min_mask_value: u32 = 1;
        let mut sec_mask_value: u32 = 1;
        let mut day_match_flag: u32 = 0;
        let mut day_or_week_value: u32 = 0;
        let mut hr_value: u32 = 0;
        let mut min_value: u32 = 0;
        let mut sec_value: u32 = 0;

        if alarm_mask.day_mask {
            if day > 31 {
                return;
            }
            day_or_week_mask_value = 0;
        }
        if alarm_mask.week_mask {
            if week > 7 {
                return;
            }
            day_or_week_mask_value = 0;
            day_match_flag = 1;
        }
        if alarm_mask.hour_mask {
            if hour > 39 {
                return;
            }
            hr_mask_value = 0;
        }
        if alarm_mask.minute_mask {
            if minute > 59 {
                return;
            }
            min_mask_value = 0;
        }
        if alarm_mask.second_mask {
            if second > 59 {
                return;
            }
            sec_mask_value = 0;
        }

        if day_or_week_mask_value == 0 {
            if day_match_flag == 0 {
                day_or_week_value = ((day as u32 / 10) << 24) | ((day as u32 % 10) << 20);
            } else {
                day_or_week_value = (week as u32 % 10) << 20;
            }
        }
        if hr_mask_value == 0 {
            hr_value = ((hour as u32 / 10) << 18) | ((hour as u32 % 10) << 14);
        }
        if min_mask_value == 0 {
            min_value = ((minute as u32 / 10) << 11) | ((minute as u32 % 10) << 7);
        }
        if sec_mask_value == 0 {
            sec_value = ((second as u32 / 10) << 4) | (second as u32 % 10);
        }

        let alarm_value = (day_match_flag << 30)
            | ((day_or_week_mask_value << 3
                | hr_mask_value << 2
                | min_mask_value << 1
                | sec_mask_value)
                << 26)
            | day_or_week_value
            | hr_value
            | min_value
            | sec_value;

        self.check_syn();
        match alarm_index {
            0 => self.alarm0.write(alarm_value),
            _ => self.alarm1.write(alarm_value),
        }
    }

    /// Configures maximum value for the RTC cyc counter.
    ///
    /// # Parameters
    /// - `max_value`: Maximum cyc counter value.
    pub fn config_cyc_max(&self, max_value: u32) {
        self.check_syn();
        self.cyc_max.write(max_value);
    }

    /// Configures RTC PPM adjustment.
    ///
    /// # Parameters
    /// - `adjust_value`: PPM value where effective PPM is `adjust_value / 2`.
    pub fn config_ppm(&self, adjust_value: i16) {
        if !(-2048..=2048).contains(&adjust_value) {
            return;
        }

        self.check_syn();
        self.ppm_adjust.write(if adjust_value > 0 {
            (0x7FFF - adjust_value) as u32
        } else if adjust_value < 0 {
            (0x7FFF + (-adjust_value)) as u32
        } else {
            0x7FFF
        });
    }

    /// Configures RTC IO output source.
    pub fn config_io_output(&self, io: RtcIo) {
        self.check_syn();
        toggle_reg_bits!(self.cr2, RtcIo::Sec as u32, false);
        self.check_syn();
        toggle_reg_bits!(self.cr2, io as u32, true);
    }

    /// Gets RTC IO output selection.
    pub fn get_io_output(&self) -> u8 {
        self.check_syn();
        ((self.cr2.read() & RtcIo::Sec as u32) >> 4) as u8
    }

    /// Configures erase retention SRAM source.
    ///
    /// # Parameters
    /// - `ret_sram`: Retention SRAM erase source.
    /// - `enable`: `true` to enable, `false` to disable.
    pub fn config_erase_ret_sram(&self, ret_sram: RtcRetSramErase, enable: bool) {
        self.check_syn();
        toggle_reg_bits!(self.cr2, ret_sram as u32, enable);
    }
}
