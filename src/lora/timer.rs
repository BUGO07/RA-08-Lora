use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

use crate::{
    cortex::func::{_disable_irq, _enable_irq},
    peripherals::regs::RTC,
};

const MAX_TIMERS: usize = 16;

static NEXT_TIMER_ID: AtomicUsize = AtomicUsize::new(1);
static HAS_LOOPED_THROUGH_MAIN: AtomicU8 = AtomicU8::new(0);
static mut G_SYSTIME_REF: u64 = 0;
static mut TIMER_EVENTS: heapless::Vec<TimerEvent, MAX_TIMERS> = heapless::Vec::new();

pub struct TimerEvent {
    pub id: usize,
    pub timestamp: usize,
    pub reload_value: usize,
    pub is_running: bool,
    pub callback: Option<fn()>,
}

pub struct TimerSysTime {
    pub seconds: usize,
    pub subseconds: i16,
}

// ── C-compatible types matching timer.h ──────────────────────────────────

/// C-compatible timer event struct matching `TimerEvent_t` from timer.h.
/// C code allocates these; the Rust timer system manages them internally.
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct TimerEvent_t {
    pub Timestamp: usize,
    pub ReloadValue: usize,
    pub IsRunning: bool,
    pub Callback: Option<extern "C" fn()>,
    pub Next: *mut TimerEvent_t,
}

/// C-compatible sys time struct matching `TimerSysTime_t` from timer.h.
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct TimerSysTime_t {
    pub Seconds: usize,
    pub SubSeconds: i16,
}

impl TimerSysTime_t {
    fn into_rust(self) -> TimerSysTime {
        TimerSysTime {
            seconds: self.Seconds,
            subseconds: self.SubSeconds,
        }
    }

    fn from_rust(t: TimerSysTime) -> Self {
        Self {
            Seconds: t.seconds,
            SubSeconds: t.subseconds,
        }
    }
}

/// Build a temporary Rust `TimerEvent` from the C struct, using the pointer
/// address as the unique timer ID.
///
/// # Safety
/// `obj` must be a valid, non-null pointer to an initialized `TimerEvent_t`.
unsafe fn c_timer_to_rust(obj: *mut TimerEvent_t) -> TimerEvent {
    let c = unsafe { &*obj };
    TimerEvent {
        id: obj as usize,
        timestamp: c.Timestamp,
        reload_value: c.ReloadValue,
        is_running: c.IsRunning,
        callback: c
            .Callback
            .map(|f| unsafe { core::mem::transmute::<extern "C" fn(), fn()>(f) }),
    }
}

/// Write back relevant fields from a Rust `TimerEvent` to the C struct.
///
/// # Safety
/// `obj` must be a valid, non-null pointer to a `TimerEvent_t`.
unsafe fn rust_timer_to_c(obj: *mut TimerEvent_t, event: &TimerEvent) {
    let c = unsafe { &mut *obj };
    c.Timestamp = event.timestamp;
    c.ReloadValue = event.reload_value;
    c.IsRunning = event.is_running;
}

pub fn timer_set_sys_time(sys_time: TimerSysTime) {
    let cur_time = RTC.get_timer_value();
    let set_time = sys_time.seconds as u64 * 1000 + sys_time.subseconds as u64;
    unsafe {
        G_SYSTIME_REF = set_time - cur_time;
    }
}

pub fn timer_get_sys_time() -> TimerSysTime {
    let cur_time = timer_get_current_time();
    TimerSysTime {
        seconds: (cur_time / 1000) as usize,
        subseconds: (cur_time % 1000) as i16,
    }
}

pub fn timer_get_current_time() -> u64 {
    RTC.get_timer_value() + unsafe { G_SYSTIME_REF }
}

pub fn timer_get_elapsed_time(saved_time: u64) -> u64 {
    timer_get_current_time() - saved_time
}

/// Update every queued timer's timestamp by subtracting the elapsed time since
/// the last RTC timer context was captured.  Mirrors `TimeStampsUpdate()` in C.
fn time_stamps_update() {
    let old = RTC.get_timer_ctx();
    let now = RTC.set_timer_ctx();

    let delta_ctx = (now - old) as usize;
    for event in unsafe { TIMER_EVENTS.iter_mut() } {
        if event.timestamp > delta_ctx {
            event.timestamp -= delta_ctx;
        } else {
            event.timestamp = 0;
        }
    }
}

pub fn timer_init(event: &mut TimerEvent, callback: fn()) {
    event.id = NEXT_TIMER_ID.fetch_add(1, Ordering::Relaxed);
    event.timestamp = 0;
    event.reload_value = 0;
    event.is_running = false;
    event.callback = Some(callback);
}

pub fn timer_start(timer: &mut TimerEvent) {
    _disable_irq();

    if timer_exists(timer) {
        _enable_irq();
        return;
    }

    timer.timestamp = timer.reload_value;
    timer.is_running = false;

    if unsafe { TIMER_EVENTS.is_empty() } {
        RTC.set_timer_ctx();

        timer_insert_new_head_timer(timer);
    } else {
        let elapsed_time = RTC.get_elapsed_time() as usize;
        timer.timestamp += elapsed_time;

        let head_ts = unsafe { TIMER_EVENTS[0].timestamp };

        if timer.timestamp < head_ts {
            time_stamps_update();

            timer.timestamp -= elapsed_time;
            timer_insert_new_head_timer(timer);
        } else {
            timer_insert_timer(timer);
        }
    }
    _enable_irq();
}

pub fn timer_stop(timer: &mut TimerEvent) {
    _disable_irq();

    let list = unsafe { &mut TIMER_EVENTS };

    if list.is_empty() {
        _enable_irq();
        return;
    }

    // Find the timer in the list by ID
    let pos = list.iter().position(|t| t.id == timer.id);
    let Some(idx) = pos else {
        _enable_irq();
        return;
    };

    if idx == 0 {
        // Stopping the head timer
        if list[0].is_running {
            list.remove(0);
            if !list.is_empty() {
                time_stamps_update();
                timer_set_timeout_head();
            } else {
                RTC.stop_timeout();
            }
        } else {
            list.remove(0);
        }
    } else {
        list.remove(idx);
    }

    timer.is_running = false;
    _enable_irq();
}

pub fn timer_reset(timer: &mut TimerEvent) {
    timer_stop(timer);
    timer_start(timer);
}

pub fn timer_set_value(timer: &mut TimerEvent, value: usize) {
    timer_stop(timer);
    timer.timestamp = value;
    timer.reload_value = value;
}

pub fn timer_irq_handler() {
    // Update timer context for callbacks
    time_stamps_update();

    // Execute the head alarm callback immediately
    if let Some(head) = unsafe { TIMER_EVENTS.first() } {
        let cb = head.callback;
        unsafe { TIMER_EVENTS.remove(0) };
        if let Some(cb) = cb {
            cb();
        }
    }

    // Remove and fire all expired timers
    loop {
        let should_fire = unsafe {
            TIMER_EVENTS
                .first()
                .is_some_and(|h| h.timestamp < RTC.get_elapsed_time() as usize || h.timestamp == 0)
        };
        if !should_fire {
            break;
        }
        let cb = unsafe { TIMER_EVENTS[0].callback };
        unsafe { TIMER_EVENTS.remove(0) };
        if let Some(cb) = cb {
            cb();
        }
    }

    // Update timestamps after callbacks
    time_stamps_update();

    // Start the next timer if it exists and is not already running
    if unsafe { TIMER_EVENTS.first() }.is_some_and(|h| !h.is_running) {
        timer_set_timeout_head();
    }
}

pub fn timer_low_power_handler() {
    let count = HAS_LOOPED_THROUGH_MAIN.load(Ordering::Relaxed);
    if count < 5 {
        HAS_LOOPED_THROUGH_MAIN.store(count + 1, Ordering::Relaxed);
    } else {
        HAS_LOOPED_THROUGH_MAIN.store(0, Ordering::Relaxed);
        RTC.enter_low_power_stop_mode();
    }
}

pub fn timer_temp_compensation(period: u64, _temperature: f32) -> u64 {
    period
}

/// Insert `timer` as the new head of the list and arm the RTC alarm.
fn timer_insert_new_head_timer(timer: &mut TimerEvent) {
    let list = unsafe { &mut TIMER_EVENTS };

    // Mark the current head (if any) as not running
    if let Some(head) = list.first_mut() {
        head.is_running = false;
    }

    // Build a new event value from the caller's mutable reference.
    // We copy the fields because the Vec owns its storage.
    let new_event = TimerEvent {
        id: timer.id,
        timestamp: timer.timestamp,
        reload_value: timer.reload_value,
        is_running: false,
        callback: timer.callback,
    };

    // Shift everything right and place the new head at index 0
    if list.is_empty() {
        let _ = list.push(new_event);
    } else {
        let _ = list.push(TimerEvent {
            id: 0,
            timestamp: 0,
            reload_value: 0,
            is_running: false,
            callback: None,
        });

        // Shift elements to the right
        let len = list.len();
        for i in (1..len).rev() {
            list.swap(i, i - 1);
        }

        list[0] = new_event;
    }

    timer_set_timeout_head();
}

/// Insert `timer` into the sorted list (not as head).
fn timer_insert_timer(timer: &mut TimerEvent) {
    let list = unsafe { &mut TIMER_EVENTS };

    let new_event = TimerEvent {
        id: timer.id,
        timestamp: timer.timestamp,
        reload_value: timer.reload_value,
        is_running: false,
        callback: timer.callback,
    };

    // Find the correct sorted position (after head)
    let mut insert_idx = list.len(); // default: append at end
    for i in 1..list.len() {
        if new_event.timestamp <= list[i].timestamp {
            insert_idx = i;
            break;
        }
    }

    // Insert by pushing a placeholder, then shifting right from insert_idx
    let _ = list.push(TimerEvent {
        id: 0,
        timestamp: 0,
        reload_value: 0,
        is_running: false,
        callback: None,
    });
    let len = list.len();
    for i in (insert_idx + 1..len).rev() {
        list.swap(i, i - 1);
    }
    list[insert_idx] = new_event;
}

/// Arms the RTC timeout for the head timer.
fn timer_set_timeout_head() {
    if let Some(head) = unsafe { TIMER_EVENTS.first_mut() } {
        head.is_running = true;
        RTC.set_timeout(head.timestamp);
    }
}

fn timer_exists(timer: &mut TimerEvent) -> bool {
    unsafe { TIMER_EVENTS.iter().any(|t| t.id == timer.id) }
}

pub fn timer_add_sys_time(a: TimerSysTime, b: TimerSysTime) -> TimerSysTime {
    let mut c = TimerSysTime {
        seconds: a.seconds + b.seconds,
        subseconds: a.subseconds + b.subseconds,
    };
    if c.subseconds >= 1000 {
        c.seconds += 1;
        c.subseconds -= 1000;
    }
    c
}

pub fn timer_sub_sys_time(a: TimerSysTime, b: TimerSysTime) -> TimerSysTime {
    let mut c = TimerSysTime {
        seconds: a.seconds.saturating_sub(b.seconds),
        subseconds: a.subseconds - b.subseconds,
    };
    if c.subseconds < 0 {
        c.seconds = c.seconds.saturating_sub(1);
        c.subseconds += 1000;
    }
    c
}

// ── extern "C" wrappers matching timer.h ──────────────────────────────────

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn TimerSetSysTime(sysTime: TimerSysTime_t) {
    timer_set_sys_time(sysTime.into_rust());
}

#[unsafe(no_mangle)]
pub extern "C" fn TimerGetSysTime() -> TimerSysTime_t {
    TimerSysTime_t::from_rust(timer_get_sys_time())
}

/// # Safety
/// `obj` must be a valid, non-null pointer to a `TimerEvent_t`.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TimerInit(obj: *mut TimerEvent_t, callback: extern "C" fn()) {
    let c = unsafe { &mut *obj };
    c.Timestamp = 0;
    c.ReloadValue = 0;
    c.IsRunning = false;
    c.Callback = Some(callback);
    c.Next = core::ptr::null_mut();
}

#[unsafe(no_mangle)]
pub extern "C" fn TimerIrqHandler() {
    timer_irq_handler();
}

/// # Safety
/// `obj` must be a valid, non-null pointer to an initialized `TimerEvent_t`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TimerStart(obj: *mut TimerEvent_t) {
    let mut event = unsafe { c_timer_to_rust(obj) };
    timer_start(&mut event);
    unsafe { rust_timer_to_c(obj, &event) };
}

/// # Safety
/// `obj` must be a valid, non-null pointer to an initialized `TimerEvent_t`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TimerStop(obj: *mut TimerEvent_t) {
    let mut event = unsafe { c_timer_to_rust(obj) };
    timer_stop(&mut event);
    unsafe { rust_timer_to_c(obj, &event) };
}

/// # Safety
/// `obj` must be a valid, non-null pointer to an initialized `TimerEvent_t`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TimerReset(obj: *mut TimerEvent_t) {
    let mut event = unsafe { c_timer_to_rust(obj) };
    timer_reset(&mut event);
    unsafe { rust_timer_to_c(obj, &event) };
}

/// # Safety
/// `obj` must be a valid, non-null pointer to an initialized `TimerEvent_t`.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TimerSetValue(obj: *mut TimerEvent_t, value: usize) {
    let mut event = unsafe { c_timer_to_rust(obj) };
    timer_set_value(&mut event, value);
    unsafe { rust_timer_to_c(obj, &event) };
}

#[unsafe(no_mangle)]
pub extern "C" fn TimerGetCurrentTime() -> u64 {
    timer_get_current_time()
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn TimerGetElapsedTime(savedTime: u64) -> u64 {
    timer_get_elapsed_time(savedTime)
}

#[unsafe(no_mangle)]
pub extern "C" fn TimerTempCompensation(period: u64, temperature: f32) -> u64 {
    timer_temp_compensation(period, temperature)
}

#[unsafe(no_mangle)]
pub extern "C" fn TimerLowPowerHandler() {
    timer_low_power_handler();
}

#[unsafe(no_mangle)]
pub extern "C" fn TimerAddSysTime(a: TimerSysTime_t, b: TimerSysTime_t) -> TimerSysTime_t {
    TimerSysTime_t::from_rust(timer_add_sys_time(a.into_rust(), b.into_rust()))
}

#[unsafe(no_mangle)]
pub extern "C" fn TimerSubSysTime(a: TimerSysTime_t, b: TimerSysTime_t) -> TimerSysTime_t {
    TimerSysTime_t::from_rust(timer_sub_sys_time(a.into_rust(), b.into_rust()))
}
