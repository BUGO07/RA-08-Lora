use crate::{
    cortex::func::{_disable_irq, _enable_irq},
    lora::{
        driver::sx1262_board::{
            sx126x_get_board_tcxo_wakeup_time, sx126x_read_register, sx126x_set_rf_tx_power,
            sx126x_write_registers,
        },
        radio::sx126x::{
            GfskModulationParams, GfskPacketParams, LORA_MAC_PRIVATE_SYNCWORD,
            LORA_MAC_PUBLIC_SYNCWORD, LoRaModulationParams, LoRaPacketParams, LoRaPacketStatus,
            ModulationParams, PacketParams, PacketStatus, RADIO_WAKEUP_TIME, REG_LR_SYNCWORD,
            RadioAddressComp, RadioCadExitModes, RadioCrcTypes, RadioDcFree, RadioIrqMasks,
            RadioLoRaBandwidths, RadioLoRaCadSymbols, RadioLoRaCodingRates, RadioLoRaCrcModes,
            RadioLoRaIQModes, RadioLoRaPacketLengthsMode, RadioLoRaSpreadingFactors,
            RadioModShapings, RadioOperatingModes, RadioPacketLengthModes, RadioPacketTypes,
            RadioPreambleDetection, RadioRampTimes, RadioRegulatorMode, RadioStandbyModes,
            SleepParams, Sx126x, sx126x_clear_irq_status, sx126x_get_irq_status,
            sx126x_get_operating_mode, sx126x_get_packet_status, sx126x_get_payload,
            sx126x_get_rssi_inst, sx126x_init, sx126x_send_payload, sx126x_set_buffer_base_address,
            sx126x_set_cad, sx126x_set_cad_params, sx126x_set_dio_irq_params,
            sx126x_set_lora_symb_num_timeout, sx126x_set_modulation_params,
            sx126x_set_operating_mode, sx126x_set_packet_params, sx126x_set_packet_type,
            sx126x_set_regulator_mode, sx126x_set_rf_frequency, sx126x_set_rx,
            sx126x_set_rx_boosted, sx126x_set_rx_duty_cycle, sx126x_set_sleep, sx126x_set_standby,
            sx126x_set_stop_rx_timer_on_preamble_detect, sx126x_set_sync_word, sx126x_set_tx,
            sx126x_set_tx_continuous_wave, sx126x_set_tx_params, sx126x_set_whitening_seed,
        },
        timer::{
            TimerEvent, timer_get_current_time, timer_get_elapsed_time, timer_init,
            timer_set_value, timer_start, timer_stop,
        },
    },
    peripherals::delay::delay_ms,
};

pub mod sx126x;

/// Radio driver supported modems
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioModem {
    Fsk = 0,
    LoRa = 1,
}

/// Radio driver internal state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioState {
    Idle = 0,
    RxRunning = 1,
    TxRunning = 2,
    Cad = 3,
}

/// Radio driver callback functions
pub struct RadioEvents {
    pub tx_done: Option<fn()>,
    pub tx_timeout: Option<fn()>,
    pub rx_done: Option<fn(payload: &[u8], rssi: i16, snr: i8)>,
    pub rx_timeout: Option<fn()>,
    pub rx_error: Option<fn()>,
    pub fhss_change_channel: Option<fn(current_channel: u8)>,
    pub cad_done: Option<fn(channel_activity_detected: bool)>,
}

// ── C-compatible types matching radio.h ──────────────────────────────────

/// C-compatible `RadioModems_t` matching radio.h.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum RadioModems_t {
    MODEM_FSK = 0,
    MODEM_LORA = 1,
}

impl RadioModems_t {
    fn to_rust(self) -> RadioModem {
        match self {
            RadioModems_t::MODEM_FSK => RadioModem::Fsk,
            RadioModems_t::MODEM_LORA => RadioModem::LoRa,
        }
    }
}

/// C-compatible `RadioState_t` matching radio.h.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum RadioState_t {
    RF_IDLE = 0,
    RF_RX_RUNNING = 1,
    RF_TX_RUNNING = 2,
    RF_CAD = 3,
}

impl RadioState_t {
    fn from_rust(s: RadioState) -> Self {
        match s {
            RadioState::Idle => RadioState_t::RF_IDLE,
            RadioState::RxRunning => RadioState_t::RF_RX_RUNNING,
            RadioState::TxRunning => RadioState_t::RF_TX_RUNNING,
            RadioState::Cad => RadioState_t::RF_CAD,
        }
    }
}

/// C-compatible `RadioEvents_t` matching radio.h.
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct RadioEvents_t {
    pub TxDone: Option<extern "C" fn()>,
    pub TxTimeout: Option<extern "C" fn()>,
    pub RxDone: Option<extern "C" fn(payload: *mut u8, size: u16, rssi: i16, snr: i8)>,
    pub RxTimeout: Option<extern "C" fn()>,
    pub RxError: Option<extern "C" fn()>,
    pub FhssChangeChannel: Option<extern "C" fn(currentChannel: u8)>,
    pub CadDone: Option<extern "C" fn(channelActivityDetected: bool)>,
}

/// Stored C events pointer and converted Rust events for the radio driver.
static mut C_RADIO_EVENTS_PTR: *const RadioEvents_t = core::ptr::null();
static mut C_RADIO_EVENTS_RUST: RadioEvents = RadioEvents {
    tx_done: None,
    tx_timeout: None,
    rx_done: None,
    rx_timeout: None,
    rx_error: None,
    fhss_change_channel: None,
    cad_done: None,
};

/// RxDone trampoline: calls the C `RxDone` callback with raw pointer + size.
fn c_rx_done_trampoline(payload: &[u8], rssi: i16, snr: i8) {
    unsafe {
        if let Some(cb) = (*C_RADIO_EVENTS_PTR).RxDone {
            cb(payload.as_ptr() as *mut u8, payload.len() as u16, rssi, snr);
        }
    }
}

/// Convert a C `RadioEvents_t` pointer into Rust `RadioEvents`, storing it
/// statically so the radio driver can use it.
///
/// # Safety
/// `events` must be a valid, non-null pointer that lives for the duration of the program.
unsafe fn store_c_events(events: *mut RadioEvents_t) {
    unsafe {
        C_RADIO_EVENTS_PTR = events;
        let e = &*events;
        C_RADIO_EVENTS_RUST = RadioEvents {
            tx_done: e
                .TxDone
                .map(|f| core::mem::transmute::<extern "C" fn(), fn()>(f)),
            tx_timeout: e
                .TxTimeout
                .map(|f| core::mem::transmute::<extern "C" fn(), fn()>(f)),
            // RxDone has a different signature in C (raw ptr + size) vs Rust (slice),
            // so we use a trampoline if the C callback is set.
            rx_done: if e.RxDone.is_some() {
                Some(c_rx_done_trampoline)
            } else {
                None
            },
            rx_timeout: e
                .RxTimeout
                .map(|f| core::mem::transmute::<extern "C" fn(), fn()>(f)),
            rx_error: e
                .RxError
                .map(|f| core::mem::transmute::<extern "C" fn(), fn()>(f)),
            fhss_change_channel: e
                .FhssChangeChannel
                .map(|f| core::mem::transmute::<extern "C" fn(u8), fn(u8)>(f)),
            cad_done: e
                .CadDone
                .map(|f| core::mem::transmute::<extern "C" fn(bool), fn(bool)>(f)),
        };
    }
}

/// C-compatible `Radio_s` struct matching radio.h.
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types, clippy::type_complexity)]
pub struct Radio_s {
    pub Init: Option<unsafe extern "C" fn(events: *mut RadioEvents_t) -> i32>,
    pub GetStatus: Option<extern "C" fn() -> RadioState_t>,
    pub SetModem: Option<extern "C" fn(modem: RadioModems_t)>,
    pub SetChannel: Option<extern "C" fn(freq: usize)>,
    pub IsChannelFree: Option<
        extern "C" fn(
            modem: RadioModems_t,
            freq: usize,
            rssiThresh: i16,
            maxCarrierSenseTime: usize,
        ) -> bool,
    >,
    pub Random: Option<extern "C" fn() -> usize>,
    pub SetRxConfig: Option<
        unsafe extern "C" fn(
            modem: RadioModems_t,
            bandwidth: usize,
            datarate: usize,
            coderate: u8,
            bandwidthAfc: usize,
            preambleLen: u16,
            symbTimeout: u16,
            fixLen: bool,
            payloadLen: u8,
            crcOn: bool,
            freqHopOn: bool,
            hopPeriod: u8,
            iqInverted: bool,
            rxContinuous: bool,
        ),
    >,
    pub SetTxConfig: Option<
        unsafe extern "C" fn(
            modem: RadioModems_t,
            power: i8,
            fdev: usize,
            bandwidth: usize,
            datarate: usize,
            coderate: u8,
            preambleLen: u16,
            fixLen: bool,
            crcOn: bool,
            freqHopOn: bool,
            hopPeriod: u8,
            iqInverted: bool,
            timeout: usize,
        ),
    >,
    pub CheckRfFrequency: Option<extern "C" fn(frequency: usize) -> bool>,
    pub TimeOnAir: Option<extern "C" fn(modem: RadioModems_t, pktLen: u8) -> usize>,
    pub Send: Option<unsafe extern "C" fn(buffer: *mut u8, size: u8)>,
    pub Sleep: Option<extern "C" fn()>,
    pub Standby: Option<extern "C" fn()>,
    pub Rx: Option<extern "C" fn(timeout: usize)>,
    pub StartCad: Option<extern "C" fn(symbols: u8)>,
    pub SetTxContinuousWave: Option<extern "C" fn(freq: usize, power: i8, time: u16)>,
    pub Rssi: Option<extern "C" fn(modem: RadioModems_t) -> i16>,
    pub Write: Option<extern "C" fn(addr: u16, data: u8)>,
    pub Read: Option<extern "C" fn(addr: u16) -> u8>,
    pub WriteBuffer: Option<unsafe extern "C" fn(addr: u16, buffer: *mut u8, size: u8)>,
    pub ReadBuffer: Option<unsafe extern "C" fn(addr: u16, buffer: *mut u8, size: u8)>,
    pub SetMaxPayloadLength: Option<extern "C" fn(modem: RadioModems_t, max: u8)>,
    pub SetPublicNetwork: Option<extern "C" fn(enable: bool)>,
    pub GetWakeupTime: Option<extern "C" fn() -> usize>,
    pub IrqProcess: Option<extern "C" fn()>,
    pub RxBoosted: Option<extern "C" fn(timeout: usize)>,
    pub SetRxDutyCycle: Option<extern "C" fn(rxTime: usize, sleepTime: usize)>,
}

struct FskBandwidth {
    bandwidth: usize,
    reg_value: u8,
}

const FSK_BANDWIDTHS: &[FskBandwidth] = &[
    FskBandwidth {
        bandwidth: 4800,
        reg_value: 0x1F,
    },
    FskBandwidth {
        bandwidth: 5800,
        reg_value: 0x17,
    },
    FskBandwidth {
        bandwidth: 7300,
        reg_value: 0x0F,
    },
    FskBandwidth {
        bandwidth: 9700,
        reg_value: 0x1E,
    },
    FskBandwidth {
        bandwidth: 11700,
        reg_value: 0x16,
    },
    FskBandwidth {
        bandwidth: 14600,
        reg_value: 0x0E,
    },
    FskBandwidth {
        bandwidth: 19500,
        reg_value: 0x1D,
    },
    FskBandwidth {
        bandwidth: 23400,
        reg_value: 0x15,
    },
    FskBandwidth {
        bandwidth: 29300,
        reg_value: 0x0D,
    },
    FskBandwidth {
        bandwidth: 39000,
        reg_value: 0x1C,
    },
    FskBandwidth {
        bandwidth: 46900,
        reg_value: 0x14,
    },
    FskBandwidth {
        bandwidth: 58600,
        reg_value: 0x0C,
    },
    FskBandwidth {
        bandwidth: 78200,
        reg_value: 0x1B,
    },
    FskBandwidth {
        bandwidth: 93800,
        reg_value: 0x13,
    },
    FskBandwidth {
        bandwidth: 117300,
        reg_value: 0x0B,
    },
    FskBandwidth {
        bandwidth: 156200,
        reg_value: 0x1A,
    },
    FskBandwidth {
        bandwidth: 187200,
        reg_value: 0x12,
    },
    FskBandwidth {
        bandwidth: 234300,
        reg_value: 0x0A,
    },
    FskBandwidth {
        bandwidth: 312000,
        reg_value: 0x19,
    },
    FskBandwidth {
        bandwidth: 373600,
        reg_value: 0x11,
    },
    FskBandwidth {
        bandwidth: 467000,
        reg_value: 0x09,
    },
    FskBandwidth {
        bandwidth: 500000,
        reg_value: 0x00,
    }, // Invalid Bandwidth
];

/// LoRa bandwidth index-to-enum mapping, matching the C `Bandwidths[]` array
const BANDWIDTHS: &[RadioLoRaBandwidths] = &[
    RadioLoRaBandwidths::Bw125,
    RadioLoRaBandwidths::Bw250,
    RadioLoRaBandwidths::Bw500,
    RadioLoRaBandwidths::Bw062,
    RadioLoRaBandwidths::Bw041,
    RadioLoRaBandwidths::Bw031,
    RadioLoRaBandwidths::Bw020,
    RadioLoRaBandwidths::Bw015,
    RadioLoRaBandwidths::Bw010,
    RadioLoRaBandwidths::Bw007,
];

static mut MAX_PAYLOAD_LENGTH: u8 = 0xFF;
static mut TX_TIMEOUT: usize = 0;
static mut RX_TIMEOUT: usize = 0;
static mut RX_CONTINUOUS: bool = false;

static mut RADIO_PKT_STATUS: Option<PacketStatus> = None;

static mut IRQ_FIRED: bool = false;
static mut IRQ_REGS: u16 = 0;

static mut RADIO_PUBLIC_NETWORK_PREVIOUS: bool = false;
static mut RADIO_PUBLIC_NETWORK_CURRENT: bool = false;

static mut RADIO_EVENTS: Option<&'static RadioEvents> = None;

static mut SX126X: Option<Sx126x> = None;

static mut TX_TIMEOUT_TIMER: TimerEvent = TimerEvent {
    id: 0,
    timestamp: 0,
    reload_value: 0,
    is_running: false,
    callback: None,
};

static mut RX_TIMEOUT_TIMER: TimerEvent = TimerEvent {
    id: 0,
    timestamp: 0,
    reload_value: 0,
    is_running: false,
    callback: None,
};

static mut CAD_TIMEOUT_TIMER: TimerEvent = TimerEvent {
    id: 0,
    timestamp: 0,
    reload_value: 0,
    is_running: false,
    callback: None,
};

static mut RNG_NEXT: usize = 1;

pub fn srand1(seed: usize) {
    unsafe { RNG_NEXT = seed };
}

pub fn rand1() -> i32 {
    const RAND_LOCAL_MAX: u64 = 2_147_483_647;
    unsafe {
        RNG_NEXT = ((RNG_NEXT as u64)
            .wrapping_mul(1_103_515_245)
            .wrapping_add(12345)
            % RAND_LOCAL_MAX) as usize;
        RNG_NEXT as i32
    }
}

pub fn randr(min: i32, max: i32) -> i32 {
    rand1() % (max - min + 1) + min
}

fn sx126x_state() -> &'static mut Sx126x {
    unsafe { SX126X.as_mut().expect("radio not initialised") }
}

fn radio_get_fsk_bandwidth_reg_value(bandwidth: usize) -> u8 {
    if bandwidth == 0 {
        return 0x1F;
    }
    for i in 0..FSK_BANDWIDTHS.len() - 1 {
        if bandwidth >= FSK_BANDWIDTHS[i].bandwidth && bandwidth < FSK_BANDWIDTHS[i + 1].bandwidth {
            return FSK_BANDWIDTHS[i + 1].reg_value;
        }
    }
    // Should not reach here — infinite loop in C; we panic in debug builds
    #[cfg(debug_assertions)]
    panic!("FSK bandwidth register value not found");
    #[cfg(not(debug_assertions))]
    loop {}
}

/// Compute symbol time in ms for a given LoRa BW + SF
pub fn radio_symb_time(bw: RadioLoRaBandwidths, sf: u8) -> f64 {
    let bw_khz: f64 = match bw {
        RadioLoRaBandwidths::Bw007 => 7.81,
        RadioLoRaBandwidths::Bw010 => 10.42,
        RadioLoRaBandwidths::Bw015 => 15.63,
        RadioLoRaBandwidths::Bw020 => 20.83,
        RadioLoRaBandwidths::Bw031 => 31.25,
        RadioLoRaBandwidths::Bw041 => 41.67,
        RadioLoRaBandwidths::Bw062 => 62.5,
        RadioLoRaBandwidths::Bw125 => 125.0,
        RadioLoRaBandwidths::Bw250 => 250.0,
        RadioLoRaBandwidths::Bw500 => 500.0,
    };
    (1usize << sf) as f64 / bw_khz
}

/// Initialises the radio hardware and internal state.
pub fn radio_init(events: &'static RadioEvents) -> i32 {
    unsafe {
        RADIO_EVENTS = Some(events);

        SX126X = Some(Sx126x {
            packet_params: PacketParams::LoRa(LoRaPacketParams {
                preamble_length: 0,
                header_type: RadioLoRaPacketLengthsMode::VariableLength,
                payload_length: 0,
                crc_mode: RadioLoRaCrcModes::Off,
                invert_iq: RadioLoRaIQModes::Normal,
            }),
            packet_status: PacketStatus::LoRa(LoRaPacketStatus {
                rssi_pkt: 0,
                snr_pkt: 0,
                signal_rssi_pkt: 0,
                freq_error: 0,
            }),
            modulation_params: ModulationParams::LoRa(LoRaModulationParams {
                spreading_factor: RadioLoRaSpreadingFactors::Sf7,
                bandwidth: RadioLoRaBandwidths::Bw125,
                coding_rate: RadioLoRaCodingRates::Cr4_5,
                low_datarate_optimize: 0,
            }),
        });
    }

    sx126x_init();
    sx126x_set_standby(RadioStandbyModes::StdbyRc);
    sx126x_set_regulator_mode(RadioRegulatorMode::UseDcdc);

    sx126x_set_buffer_base_address(0x00, 0x00);
    sx126x_set_tx_params(0, RadioRampTimes::Ramp200Us);
    sx126x_set_dio_irq_params(
        RadioIrqMasks::None as u16,
        RadioIrqMasks::None as u16,
        RadioIrqMasks::None as u16,
        RadioIrqMasks::None as u16,
    );

    // Initialise driver timeout timers
    timer_init(unsafe { &mut TX_TIMEOUT_TIMER }, radio_on_tx_timeout_irq);
    timer_init(unsafe { &mut RX_TIMEOUT_TIMER }, radio_on_rx_timeout_irq);
    timer_init(unsafe { &mut CAD_TIMEOUT_TIMER }, radio_on_cad_timeout_irq);

    unsafe { IRQ_FIRED = false };
    0
}

/// Returns the current radio state.
pub fn radio_get_status() -> RadioState {
    match sx126x_get_operating_mode() {
        RadioOperatingModes::Tx => RadioState::TxRunning,
        RadioOperatingModes::Rx => RadioState::RxRunning,
        RadioOperatingModes::Cad => RadioState::Cad,
        _ => RadioState::Idle,
    }
}

/// Sets the radio modem (FSK or LoRa).
pub fn radio_set_modem(modem: RadioModem) {
    match modem {
        RadioModem::Fsk => {
            sx126x_set_packet_type(RadioPacketTypes::Gfsk);
            unsafe { RADIO_PUBLIC_NETWORK_CURRENT = false };
        }
        RadioModem::LoRa => {
            sx126x_set_packet_type(RadioPacketTypes::LoRa);
            unsafe {
                if RADIO_PUBLIC_NETWORK_CURRENT != RADIO_PUBLIC_NETWORK_PREVIOUS {
                    RADIO_PUBLIC_NETWORK_CURRENT = RADIO_PUBLIC_NETWORK_PREVIOUS;
                    radio_set_public_network(RADIO_PUBLIC_NETWORK_CURRENT);
                }
            }
        }
    }
}

/// Sets the channel frequency.
pub fn radio_set_channel(freq: usize) {
    sx126x_set_rf_frequency(freq);
}

/// Checks if the channel is free for the given time.
pub fn radio_is_channel_free(
    modem: RadioModem,
    freq: usize,
    rssi_thresh: i16,
    max_carrier_sense_time: usize,
) -> bool {
    radio_set_modem(modem);
    radio_set_channel(freq);
    radio_rx(0);
    delay_ms(1);

    let carrier_sense_time = timer_get_current_time();

    while timer_get_elapsed_time(carrier_sense_time) < max_carrier_sense_time as u64 {
        let rssi = radio_rssi(modem);
        if rssi > rssi_thresh {
            radio_sleep();
            return false;
        }
    }
    radio_sleep();
    true
}

/// Generates a 32-bit random value based on RSSI readings.
pub fn radio_random() -> usize {
    radio_set_modem(RadioModem::LoRa);
    sx126x_set_rx(0);

    let mut rnd: usize = 0;
    for i in 0..32 {
        delay_ms(1);
        rnd |= ((sx126x_get_rssi_inst() as usize) & 0x01) << i;
    }
    rnd = rnd.wrapping_add(rand1() as usize);
    radio_sleep();
    rnd
}

/// Configures the radio for reception.
#[allow(clippy::too_many_arguments)]
pub fn radio_set_rx_config(
    modem: RadioModem,
    bandwidth: usize,
    datarate: usize,
    coderate: u8,
    _bandwidth_afc: usize,
    preamble_len: u16,
    mut symb_timeout: u16,
    fix_len: bool,
    payload_len: u8,
    crc_on: bool,
    _freq_hop_on: bool,
    _hop_period: u8,
    iq_inverted: bool,
    rx_continuous: bool,
) {
    unsafe { RX_CONTINUOUS = rx_continuous };

    if rx_continuous {
        symb_timeout = 0;
    }
    unsafe {
        MAX_PAYLOAD_LENGTH = if fix_len { payload_len } else { 0xFF };
    }

    let sx = sx126x_state();
    let max_payload = unsafe { MAX_PAYLOAD_LENGTH };

    match modem {
        RadioModem::Fsk => {
            sx126x_set_stop_rx_timer_on_preamble_detect(false);

            let mod_params = ModulationParams::Gfsk(GfskModulationParams {
                bit_rate: datarate,
                fdev: 0,
                modulation_shaping: RadioModShapings::GBt1,
                bandwidth: radio_get_fsk_bandwidth_reg_value(bandwidth),
            });

            let pkt_params = PacketParams::Gfsk(GfskPacketParams {
                preamble_length: preamble_len << 3, // bytes -> bits
                preamble_min_detect: RadioPreambleDetection::Detect08Bits,
                sync_word_length: 3 << 3,
                addr_comp: RadioAddressComp::FiltOff,
                header_type: if fix_len {
                    RadioPacketLengthModes::FixedLength
                } else {
                    RadioPacketLengthModes::VariableLength
                },
                payload_length: max_payload,
                crc_length: if crc_on {
                    RadioCrcTypes::TwoBytesCcit
                } else {
                    RadioCrcTypes::Off
                },
                dc_free: RadioDcFree::Whitening,
            });

            sx.modulation_params = mod_params;
            sx.packet_params = pkt_params;

            radio_standby();
            radio_set_modem(
                if matches!(sx.modulation_params, ModulationParams::Gfsk(_)) {
                    RadioModem::Fsk
                } else {
                    RadioModem::LoRa
                },
            );
            sx126x_set_modulation_params(&sx.modulation_params);
            sx126x_set_packet_params(&sx.packet_params);
            sx126x_set_sync_word(&[0xC1, 0x94, 0xC1, 0x00, 0x00, 0x00, 0x00, 0x00]);
            sx126x_set_whitening_seed(0x01FF);

            unsafe {
                RX_TIMEOUT =
                    (symb_timeout as f64 * ((1.0 / datarate as f64) * 8.0) * 1000.0) as usize;
            }
        }
        RadioModem::LoRa => {
            sx126x_set_stop_rx_timer_on_preamble_detect(false);
            sx126x_set_lora_symb_num_timeout(symb_timeout as u8);

            let bw = if bandwidth < BANDWIDTHS.len() {
                BANDWIDTHS[bandwidth]
            } else {
                RadioLoRaBandwidths::Bw125
            };

            let sf =
                unsafe { core::mem::transmute::<u8, RadioLoRaSpreadingFactors>(datarate as u8) };
            let cr = unsafe { core::mem::transmute::<u8, RadioLoRaCodingRates>(coderate) };

            let low_dr_opt = if ((bandwidth == 0) && (datarate == 11 || datarate == 12))
                || (bandwidth == 1 && datarate == 12)
                || (radio_symb_time(bw, datarate as u8) >= 16.38)
            {
                0x01
            } else {
                0x00
            };

            let mod_params = ModulationParams::LoRa(LoRaModulationParams {
                spreading_factor: sf,
                bandwidth: bw,
                coding_rate: cr,
                low_datarate_optimize: low_dr_opt,
            });

            let preamble = if matches!(
                sf,
                RadioLoRaSpreadingFactors::Sf5 | RadioLoRaSpreadingFactors::Sf6
            ) {
                preamble_len.max(12)
            } else {
                preamble_len
            };

            let header_type = if fix_len {
                RadioLoRaPacketLengthsMode::FixedLength
            } else {
                RadioLoRaPacketLengthsMode::VariableLength
            };
            let crc_mode = if crc_on {
                RadioLoRaCrcModes::On
            } else {
                RadioLoRaCrcModes::Off
            };
            let invert_iq = if iq_inverted {
                RadioLoRaIQModes::Inverted
            } else {
                RadioLoRaIQModes::Normal
            };

            let pkt_params = PacketParams::LoRa(LoRaPacketParams {
                preamble_length: preamble,
                header_type,
                payload_length: max_payload,
                crc_mode,
                invert_iq,
            });

            sx.modulation_params = mod_params;
            sx.packet_params = pkt_params;

            radio_set_modem(
                if matches!(sx.modulation_params, ModulationParams::Gfsk(_)) {
                    RadioModem::Fsk
                } else {
                    RadioModem::LoRa
                },
            );
            sx126x_set_modulation_params(&sx.modulation_params);
            sx126x_set_packet_params(&sx.packet_params);

            // WORKAROUND — Optimizing the Inverted IQ Operation (DS_SX1261-2_V1.2 ch. 15.4)
            if iq_inverted {
                let val = sx126x_read_register(0x0736) & !(1 << 2);
                sx126x_write_registers(0x0736, &[val]);
            } else {
                let val = sx126x_read_register(0x0736) | (1 << 2);
                sx126x_write_registers(0x0736, &[val]);
            }

            unsafe { RX_TIMEOUT = 0xFFFF };
        }
    }
}

/// Configures the radio for transmission.
#[allow(clippy::too_many_arguments)]
pub fn radio_set_tx_config(
    modem: RadioModem,
    power: i8,
    fdev: usize,
    bandwidth: usize,
    datarate: usize,
    coderate: u8,
    preamble_len: u16,
    fix_len: bool,
    crc_on: bool,
    _freq_hop_on: bool,
    _hop_period: u8,
    iq_inverted: bool,
    timeout: usize,
) {
    let sx = sx126x_state();
    let max_payload = unsafe { MAX_PAYLOAD_LENGTH };

    match modem {
        RadioModem::Fsk => {
            let mod_params = ModulationParams::Gfsk(GfskModulationParams {
                bit_rate: datarate,
                fdev,
                modulation_shaping: RadioModShapings::GBt1,
                bandwidth: radio_get_fsk_bandwidth_reg_value(bandwidth),
            });

            let pkt_params = PacketParams::Gfsk(GfskPacketParams {
                preamble_length: preamble_len << 3,
                preamble_min_detect: RadioPreambleDetection::Detect08Bits,
                sync_word_length: 3 << 3,
                addr_comp: RadioAddressComp::FiltOff,
                header_type: if fix_len {
                    RadioPacketLengthModes::FixedLength
                } else {
                    RadioPacketLengthModes::VariableLength
                },
                payload_length: max_payload,
                crc_length: if crc_on {
                    RadioCrcTypes::TwoBytesCcit
                } else {
                    RadioCrcTypes::Off
                },
                dc_free: RadioDcFree::Whitening,
            });

            sx.modulation_params = mod_params;
            sx.packet_params = pkt_params;

            radio_standby();
            radio_set_modem(
                if matches!(sx.modulation_params, ModulationParams::Gfsk(_)) {
                    RadioModem::Fsk
                } else {
                    RadioModem::LoRa
                },
            );
            sx126x_set_modulation_params(&sx.modulation_params);
            sx126x_set_packet_params(&sx.packet_params);
            sx126x_set_sync_word(&[0xC1, 0x94, 0xC1, 0x00, 0x00, 0x00, 0x00, 0x00]);
            sx126x_set_whitening_seed(0x01FF);
        }
        RadioModem::LoRa => {
            let bw = if bandwidth < BANDWIDTHS.len() {
                BANDWIDTHS[bandwidth]
            } else {
                RadioLoRaBandwidths::Bw125
            };

            let sf =
                unsafe { core::mem::transmute::<u8, RadioLoRaSpreadingFactors>(datarate as u8) };
            let cr = unsafe { core::mem::transmute::<u8, RadioLoRaCodingRates>(coderate) };

            let low_dr_opt = if ((bandwidth == 0) && (datarate == 11 || datarate == 12))
                || (bandwidth == 1 && datarate == 12)
                || (radio_symb_time(bw, datarate as u8) >= 16.38)
            {
                0x01
            } else {
                0x00
            };

            let mod_params = ModulationParams::LoRa(LoRaModulationParams {
                spreading_factor: sf,
                bandwidth: bw,
                coding_rate: cr,
                low_datarate_optimize: low_dr_opt,
            });

            let preamble = if matches!(
                sf,
                RadioLoRaSpreadingFactors::Sf5 | RadioLoRaSpreadingFactors::Sf6
            ) {
                preamble_len.max(12)
            } else {
                preamble_len
            };

            let header_type = if fix_len {
                RadioLoRaPacketLengthsMode::FixedLength
            } else {
                RadioLoRaPacketLengthsMode::VariableLength
            };
            let crc_mode = if crc_on {
                RadioLoRaCrcModes::On
            } else {
                RadioLoRaCrcModes::Off
            };
            let invert_iq = if iq_inverted {
                RadioLoRaIQModes::Inverted
            } else {
                RadioLoRaIQModes::Normal
            };

            let pkt_params = PacketParams::LoRa(LoRaPacketParams {
                preamble_length: preamble,
                header_type,
                payload_length: max_payload,
                crc_mode,
                invert_iq,
            });

            sx.modulation_params = mod_params;
            sx.packet_params = pkt_params;

            radio_standby();
            radio_set_modem(
                if matches!(sx.modulation_params, ModulationParams::Gfsk(_)) {
                    RadioModem::Fsk
                } else {
                    RadioModem::LoRa
                },
            );
            sx126x_set_modulation_params(&sx.modulation_params);
            sx126x_set_packet_params(&sx.packet_params);
        }
    }

    // WORKAROUND — Modulation Quality with 500 kHz LoRa Bandwidth (DS ch. 15.1)
    let is_lora_bw500 = matches!(modem, RadioModem::LoRa)
        && matches!(
            sx.modulation_params,
            ModulationParams::LoRa(LoRaModulationParams {
                bandwidth: RadioLoRaBandwidths::Bw500,
                ..
            })
        );
    if is_lora_bw500 {
        let val = sx126x_read_register(0x0889) & !(1 << 2);
        sx126x_write_registers(0x0889, &[val]);
    } else {
        let val = sx126x_read_register(0x0889) | (1 << 2);
        sx126x_write_registers(0x0889, &[val]);
    }

    sx126x_set_rf_tx_power(power);
    unsafe { TX_TIMEOUT = timeout };
}

/// Checks if the given RF frequency is supported.
pub fn radio_check_rf_frequency(_frequency: usize) -> bool {
    true
}

/// Computes the packet time on air in ms.
pub fn radio_time_on_air(modem: RadioModem, pkt_len: u8) -> usize {
    let sx = sx126x_state();
    match modem {
        RadioModem::Fsk => {
            if let (ModulationParams::Gfsk(mp), PacketParams::Gfsk(pp)) =
                (&sx.modulation_params, &sx.packet_params)
            {
                let header_len = if matches!(pp.header_type, RadioPacketLengthModes::FixedLength) {
                    0.0
                } else {
                    1.0
                };
                let crc_len = if matches!(pp.crc_length, RadioCrcTypes::TwoBytes) {
                    2.0
                } else {
                    0.0
                };
                let num_bits = 8.0
                    * (pp.preamble_length as f64
                        + (pp.sync_word_length >> 3) as f64
                        + header_len
                        + pkt_len as f64
                        + crc_len);
                libm::rint(num_bits / mp.bit_rate as f64 * 1e3) as usize
            } else {
                0
            }
        }
        RadioModem::LoRa => {
            if let (ModulationParams::LoRa(mp), PacketParams::LoRa(pp)) =
                (&sx.modulation_params, &sx.packet_params)
            {
                let sf = mp.spreading_factor as u8;
                let ts = radio_symb_time(mp.bandwidth, sf);
                let t_preamble = (pp.preamble_length as f64 + 4.25) * ts;
                let crc_bits = 16.0 * (mp.coding_rate as u8) as f64;
                let fixed_sub = if matches!(pp.header_type, RadioLoRaPacketLengthsMode::FixedLength)
                {
                    20.0
                } else {
                    0.0
                };
                let low_dr_sub = if mp.low_datarate_optimize > 0 { 2 } else { 0 };
                let numerator =
                    8.0 * pkt_len as f64 - 4.0 * sf as f64 + 28.0 + crc_bits - fixed_sub;
                let denominator = 4.0 * (sf - low_dr_sub) as f64;
                let tmp =
                    libm::ceil(numerator / denominator) * ((mp.coding_rate as u8 % 4) + 4) as f64;
                let n_payload = 8.0 + if tmp > 0.0 { tmp } else { 0.0 };
                let t_payload = n_payload * ts;
                let t_on_air = t_preamble + t_payload;
                libm::floor(t_on_air + 0.999) as usize
            } else {
                0
            }
        }
    }
}

/// Sends the buffer. Prepares the packet and sets the radio in transmission.
pub fn radio_send(buffer: &[u8]) {
    let irq_flags = RadioIrqMasks::TxDone as u16 | RadioIrqMasks::RxTxTimeout as u16;
    sx126x_set_dio_irq_params(
        irq_flags,
        irq_flags,
        RadioIrqMasks::None as u16,
        RadioIrqMasks::None as u16,
    );

    let sx = sx126x_state();
    let size = buffer.len() as u8;
    match &mut sx.packet_params {
        PacketParams::LoRa(pp) => pp.payload_length = size,
        PacketParams::Gfsk(pp) => pp.payload_length = size,
    }
    sx126x_set_packet_params(&sx.packet_params);

    sx126x_send_payload(buffer, 0);
    timer_set_value(unsafe { &mut TX_TIMEOUT_TIMER }, unsafe { TX_TIMEOUT });
    timer_start(unsafe { &mut TX_TIMEOUT_TIMER });
}

/// Sets the radio in sleep mode.
pub fn radio_sleep() {
    let params = SleepParams::default().set_warm_start(true);
    sx126x_set_sleep(params);
    delay_ms(2);
}

/// Sets the radio in standby mode.
pub fn radio_standby() {
    sx126x_set_standby(RadioStandbyModes::StdbyRc);
}

/// Sets the radio in reception mode for the given time.
pub fn radio_rx(timeout: usize) {
    let irq_flags = RadioIrqMasks::RxDone as u16
        | RadioIrqMasks::CrcError as u16
        | RadioIrqMasks::RxTxTimeout as u16;
    sx126x_set_dio_irq_params(
        irq_flags,
        irq_flags,
        RadioIrqMasks::None as u16,
        RadioIrqMasks::None as u16,
    );

    if timeout != 0 {
        timer_set_value(unsafe { &mut RX_TIMEOUT_TIMER }, timeout);
        timer_start(unsafe { &mut RX_TIMEOUT_TIMER });
    }

    if unsafe { RX_CONTINUOUS } {
        sx126x_set_rx(0xFFFFFF); // Rx Continuous
    } else {
        sx126x_set_rx(unsafe { RX_TIMEOUT } << 6);
    }
}

/// Sets the radio in boosted-gain reception mode.
pub fn radio_rx_boosted(timeout: usize) {
    let irq_flags = RadioIrqMasks::RxDone as u16;
    sx126x_set_dio_irq_params(
        irq_flags,
        irq_flags,
        RadioIrqMasks::None as u16,
        RadioIrqMasks::None as u16,
    );

    if timeout != 0 {
        timer_set_value(unsafe { &mut RX_TIMEOUT_TIMER }, timeout);
        timer_start(unsafe { &mut RX_TIMEOUT_TIMER });
    }

    if unsafe { RX_CONTINUOUS } {
        sx126x_set_rx_boosted(0xFFFFFF);
    } else {
        sx126x_set_rx_boosted(unsafe { RX_TIMEOUT } << 6);
    }
}

/// Sets the Rx duty cycle management parameters.
pub fn radio_set_rx_duty_cycle(rx_time: usize, sleep_time: usize) {
    sx126x_set_rx_duty_cycle(rx_time, sleep_time);
}

/// Starts a Channel Activity Detection.
pub fn radio_start_cad(symbols: u8) {
    let sx = sx126x_state();
    let cad_det_peak = if let ModulationParams::LoRa(mp) = &sx.modulation_params {
        mp.spreading_factor as u8 + 13
    } else {
        20
    };
    let cad_det_min = 10u8;

    let cad_symbol_num = if symbols >= 16 {
        RadioLoRaCadSymbols::CadOn16Symbol
    } else if symbols >= 8 {
        RadioLoRaCadSymbols::CadOn08Symbol
    } else if symbols >= 4 {
        RadioLoRaCadSymbols::CadOn04Symbol
    } else if symbols >= 2 {
        RadioLoRaCadSymbols::CadOn02Symbol
    } else {
        RadioLoRaCadSymbols::CadOn01Symbol
    };

    let irq_flags = RadioIrqMasks::CadDone as u16 | RadioIrqMasks::CadActivityDetected as u16;
    sx126x_set_dio_irq_params(
        irq_flags,
        irq_flags,
        RadioIrqMasks::None as u16,
        RadioIrqMasks::None as u16,
    );
    sx126x_set_cad_params(
        cad_symbol_num,
        cad_det_peak,
        cad_det_min,
        RadioCadExitModes::CadOnly,
        0,
    );

    sx126x_set_cad();

    timer_set_value(unsafe { &mut CAD_TIMEOUT_TIMER }, 2000);
    timer_start(unsafe { &mut CAD_TIMEOUT_TIMER });
}

/// Transmit with a raw timeout (timeout << 6).
pub fn radio_tx(timeout: usize) {
    sx126x_set_tx(timeout << 6);
}

/// Sets the radio in continuous wave transmission mode.
pub fn radio_set_tx_continuous_wave(freq: usize, power: i8, time: u16) {
    sx126x_set_rf_frequency(freq);
    sx126x_set_rf_tx_power(power);
    sx126x_set_tx_continuous_wave();

    timer_set_value(unsafe { &mut RX_TIMEOUT_TIMER }, time as usize * 1000);
    timer_start(unsafe { &mut RX_TIMEOUT_TIMER });
}

/// Reads the current RSSI value.
pub fn radio_rssi(_modem: RadioModem) -> i16 {
    sx126x_get_rssi_inst() as i16
}

/// Writes a radio register.
pub fn radio_write(addr: u16, data: u8) {
    sx126x_write_registers(addr, &[data]);
}

/// Reads a radio register.
pub fn radio_read(addr: u16) -> u8 {
    sx126x_read_register(addr)
}

/// Writes multiple radio registers.
pub fn radio_write_buffer(addr: u16, buffer: &[u8]) {
    sx126x_write_registers(addr, buffer);
}

/// Reads multiple radio registers.
pub fn radio_read_buffer(addr: u16, buffer: &mut [u8]) {
    crate::lora::driver::sx1262_board::sx126x_read_registers(addr, buffer);
}

/// Sets the maximum payload length.
pub fn radio_set_max_payload_length(modem: RadioModem, max: u8) {
    let sx = sx126x_state();
    match modem {
        RadioModem::LoRa => {
            if let PacketParams::LoRa(pp) = &mut sx.packet_params {
                pp.payload_length = max;
                unsafe { MAX_PAYLOAD_LENGTH = max };
                sx126x_set_packet_params(&sx.packet_params);
            }
        }
        RadioModem::Fsk => {
            if let PacketParams::Gfsk(pp) = &mut sx.packet_params
                && matches!(pp.header_type, RadioPacketLengthModes::VariableLength)
            {
                pp.payload_length = max;
                unsafe { MAX_PAYLOAD_LENGTH = max };
                sx126x_set_packet_params(&sx.packet_params);
            }
        }
    }
}

/// Sets the network to public or private (updates sync word).
pub fn radio_set_public_network(enable: bool) {
    unsafe {
        RADIO_PUBLIC_NETWORK_CURRENT = enable;
        RADIO_PUBLIC_NETWORK_PREVIOUS = enable;
    }

    radio_set_modem(RadioModem::LoRa);
    if enable {
        sx126x_write_registers(
            REG_LR_SYNCWORD,
            &[
                (LORA_MAC_PUBLIC_SYNCWORD >> 8) as u8,
                LORA_MAC_PUBLIC_SYNCWORD as u8,
            ],
        );
    } else {
        sx126x_write_registers(
            REG_LR_SYNCWORD,
            &[
                (LORA_MAC_PRIVATE_SYNCWORD >> 8) as u8,
                LORA_MAC_PRIVATE_SYNCWORD as u8,
            ],
        );
    }
}

/// Gets the time required for the board + radio to get out of sleep (ms).
pub fn radio_get_wakeup_time() -> usize {
    sx126x_get_board_tcxo_wakeup_time() + RADIO_WAKEUP_TIME
}

/// Dispatches a radio event callback if both the events struct and the
/// specific callback are set.
macro_rules! dispatch_event {
    ($field:ident $(, $arg:expr)*) => {
        if let Some(events) = unsafe { RADIO_EVENTS }
            && let Some(cb) = events.$field
        {
            (cb)($($arg),*);
        }
    };
}

fn radio_on_tx_timeout_irq() {
    dispatch_event!(tx_timeout);
}

fn radio_on_rx_timeout_irq() {
    dispatch_event!(rx_timeout);
}

fn radio_on_cad_timeout_irq() {
    sx126x_set_operating_mode(RadioOperatingModes::Sleep);
    dispatch_event!(cad_done, false);
}

/// Called from the DIO1 interrupt handler.
pub fn radio_on_dio_irq() {
    unsafe {
        IRQ_FIRED = true;
        IRQ_REGS = sx126x_get_irq_status();
    }
    sx126x_clear_irq_status(RadioIrqMasks::All as u16);
}

/// Processes pending radio IRQ events. Should be called from the main loop.
pub fn radio_irq_process() {
    if !unsafe { IRQ_FIRED } {
        return;
    }

    _disable_irq();
    unsafe { IRQ_FIRED = false };
    _enable_irq();

    let irq = unsafe { IRQ_REGS };

    // TX Done
    if irq & RadioIrqMasks::TxDone as u16 != 0 {
        timer_stop(unsafe { &mut TX_TIMEOUT_TIMER });
        sx126x_set_operating_mode(RadioOperatingModes::StdbyRc);
        dispatch_event!(tx_done);
    }

    // RX Done
    if irq & RadioIrqMasks::RxDone as u16 != 0 {
        timer_stop(unsafe { &mut RX_TIMEOUT_TIMER });
        if !unsafe { RX_CONTINUOUS } {
            sx126x_set_operating_mode(RadioOperatingModes::StdbyRc);
            // WORKAROUND — Implicit Header Mode Timeout Behavior (DS ch. 15.3)
            sx126x_write_registers(0x0902, &[0x00]);
            let val = sx126x_read_register(0x0944) | (1 << 1);
            sx126x_write_registers(0x0944, &[val]);
        }

        let mut rx_buf = [0u8; 255];
        let size = sx126x_get_payload(&mut rx_buf, 255).unwrap_or(0) as usize;
        let pkt_status = sx126x_get_packet_status();
        unsafe { RADIO_PKT_STATUS = Some(pkt_status) };

        if irq & RadioIrqMasks::CrcError as u16 == 0
            && let Some(events) = unsafe { RADIO_EVENTS }
            && let Some(cb) = events.rx_done
        {
            let (rssi, snr) = match &pkt_status {
                PacketStatus::LoRa(ls) => (ls.rssi_pkt as i16 + ls.snr_pkt as i16, ls.snr_pkt),
                PacketStatus::Gfsk(gs) => (gs.rssi_avg as i16, 0),
            };
            (cb)(&rx_buf[..size], rssi, snr);
        }
    }

    // CRC Error
    if irq & RadioIrqMasks::CrcError as u16 != 0 {
        if !unsafe { RX_CONTINUOUS } {
            sx126x_set_operating_mode(RadioOperatingModes::StdbyRc);
        }
        dispatch_event!(rx_error);
    }

    // CAD Done
    if irq & RadioIrqMasks::CadDone as u16 != 0 {
        timer_stop(unsafe { &mut CAD_TIMEOUT_TIMER });
        sx126x_set_operating_mode(RadioOperatingModes::StdbyRc);
        dispatch_event!(
            cad_done,
            irq & RadioIrqMasks::CadActivityDetected as u16 != 0
        );
    }

    // RX/TX Timeout
    if irq & RadioIrqMasks::RxTxTimeout as u16 != 0 {
        match sx126x_get_operating_mode() {
            RadioOperatingModes::Tx => {
                timer_stop(unsafe { &mut TX_TIMEOUT_TIMER });
                sx126x_set_operating_mode(RadioOperatingModes::StdbyRc);
                dispatch_event!(tx_timeout);
            }
            RadioOperatingModes::Rx => {
                timer_stop(unsafe { &mut RX_TIMEOUT_TIMER });
                sx126x_set_operating_mode(RadioOperatingModes::StdbyRc);
                dispatch_event!(rx_timeout);
            }
            _ => {}
        }
    }

    // Header Error
    if irq & RadioIrqMasks::HeaderError as u16 != 0 {
        timer_stop(unsafe { &mut RX_TIMEOUT_TIMER });
        if !unsafe { RX_CONTINUOUS } {
            sx126x_set_operating_mode(RadioOperatingModes::StdbyRc);
        }
        dispatch_event!(rx_timeout);
    }
}

// ── extern "C" wrappers matching radio.h ──────────────────────────────────

/// # Safety
/// `events` must be a valid, non-null pointer to a `RadioEvents_t` that
/// outlives the radio driver (typically `'static`).
#[allow(non_snake_case, clippy::missing_safety_doc)]
unsafe extern "C" fn RadioInit(events: *mut RadioEvents_t) -> i32 {
    unsafe { store_c_events(events) };
    radio_init(unsafe { &C_RADIO_EVENTS_RUST })
}

#[allow(non_snake_case)]
extern "C" fn RadioGetStatus() -> RadioState_t {
    RadioState_t::from_rust(radio_get_status())
}

#[allow(non_snake_case)]
extern "C" fn RadioSetModem(modem: RadioModems_t) {
    radio_set_modem(modem.to_rust());
}

#[allow(non_snake_case)]
extern "C" fn RadioSetChannel(freq: usize) {
    radio_set_channel(freq);
}

#[allow(non_snake_case)]
extern "C" fn RadioIsChannelFree(
    modem: RadioModems_t,
    freq: usize,
    rssiThresh: i16,
    maxCarrierSenseTime: usize,
) -> bool {
    radio_is_channel_free(modem.to_rust(), freq, rssiThresh, maxCarrierSenseTime)
}

#[allow(non_snake_case)]
extern "C" fn RadioRandom() -> usize {
    radio_random()
}

/// # Safety
/// Called from C; all parameters must be valid.
#[allow(non_snake_case, clippy::too_many_arguments, clippy::missing_safety_doc)]
unsafe extern "C" fn RadioSetRxConfig(
    modem: RadioModems_t,
    bandwidth: usize,
    datarate: usize,
    coderate: u8,
    bandwidthAfc: usize,
    preambleLen: u16,
    symbTimeout: u16,
    fixLen: bool,
    payloadLen: u8,
    crcOn: bool,
    freqHopOn: bool,
    hopPeriod: u8,
    iqInverted: bool,
    rxContinuous: bool,
) {
    radio_set_rx_config(
        modem.to_rust(),
        bandwidth,
        datarate,
        coderate,
        bandwidthAfc,
        preambleLen,
        symbTimeout,
        fixLen,
        payloadLen,
        crcOn,
        freqHopOn,
        hopPeriod,
        iqInverted,
        rxContinuous,
    );
}

/// # Safety
/// Called from C; all parameters must be valid.
#[allow(non_snake_case, clippy::too_many_arguments, clippy::missing_safety_doc)]
unsafe extern "C" fn RadioSetTxConfig(
    modem: RadioModems_t,
    power: i8,
    fdev: usize,
    bandwidth: usize,
    datarate: usize,
    coderate: u8,
    preambleLen: u16,
    fixLen: bool,
    crcOn: bool,
    freqHopOn: bool,
    hopPeriod: u8,
    iqInverted: bool,
    timeout: usize,
) {
    radio_set_tx_config(
        modem.to_rust(),
        power,
        fdev,
        bandwidth,
        datarate,
        coderate,
        preambleLen,
        fixLen,
        crcOn,
        freqHopOn,
        hopPeriod,
        iqInverted,
        timeout,
    );
}

#[allow(non_snake_case)]
extern "C" fn RadioCheckRfFrequency(frequency: usize) -> bool {
    radio_check_rf_frequency(frequency)
}

#[allow(non_snake_case)]
extern "C" fn RadioTimeOnAir(modem: RadioModems_t, pktLen: u8) -> usize {
    radio_time_on_air(modem.to_rust(), pktLen)
}

/// # Safety
/// `buffer` must be a valid pointer to at least `size` bytes.
#[allow(non_snake_case, clippy::missing_safety_doc)]
unsafe extern "C" fn RadioSend(buffer: *mut u8, size: u8) {
    let slice = unsafe { core::slice::from_raw_parts(buffer, size as usize) };
    radio_send(slice);
}

#[allow(non_snake_case)]
extern "C" fn RadioSleep() {
    radio_sleep();
}

#[allow(non_snake_case)]
extern "C" fn RadioStandby() {
    radio_standby();
}

#[allow(non_snake_case)]
extern "C" fn RadioRx(timeout: usize) {
    radio_rx(timeout);
}

#[allow(non_snake_case)]
extern "C" fn RadioStartCad(symbols: u8) {
    radio_start_cad(symbols);
}

#[allow(non_snake_case)]
extern "C" fn RadioSetTxContinuousWave(freq: usize, power: i8, time: u16) {
    radio_set_tx_continuous_wave(freq, power, time);
}

#[allow(non_snake_case)]
extern "C" fn RadioRssi(modem: RadioModems_t) -> i16 {
    radio_rssi(modem.to_rust())
}

#[allow(non_snake_case)]
extern "C" fn RadioWrite(addr: u16, data: u8) {
    radio_write(addr, data);
}

#[allow(non_snake_case)]
extern "C" fn RadioRead(addr: u16) -> u8 {
    radio_read(addr)
}

/// # Safety
/// `buffer` must be a valid pointer to at least `size` bytes.
#[allow(non_snake_case, clippy::missing_safety_doc)]
unsafe extern "C" fn RadioWriteBuffer(addr: u16, buffer: *mut u8, size: u8) {
    let slice = unsafe { core::slice::from_raw_parts(buffer, size as usize) };
    radio_write_buffer(addr, slice);
}

/// # Safety
/// `buffer` must be a valid pointer to at least `size` bytes of writable memory.
#[allow(non_snake_case, clippy::missing_safety_doc)]
unsafe extern "C" fn RadioReadBuffer(addr: u16, buffer: *mut u8, size: u8) {
    let slice = unsafe { core::slice::from_raw_parts_mut(buffer, size as usize) };
    radio_read_buffer(addr, slice);
}

#[allow(non_snake_case)]
extern "C" fn RadioSetMaxPayloadLength(modem: RadioModems_t, max: u8) {
    radio_set_max_payload_length(modem.to_rust(), max);
}

#[allow(non_snake_case)]
extern "C" fn RadioSetPublicNetwork(enable: bool) {
    radio_set_public_network(enable);
}

#[allow(non_snake_case)]
extern "C" fn RadioGetWakeupTime() -> usize {
    radio_get_wakeup_time()
}

#[allow(non_snake_case)]
extern "C" fn RadioIrqProcess() {
    radio_irq_process();
}

#[allow(non_snake_case)]
extern "C" fn RadioRxBoosted(timeout: usize) {
    radio_rx_boosted(timeout);
}

#[allow(non_snake_case)]
extern "C" fn RadioSetRxDutyCycle(rxTime: usize, sleepTime: usize) {
    radio_set_rx_duty_cycle(rxTime, sleepTime);
}

/// Global `Radio` instance matching `extern const struct Radio_s Radio` in radio.h.
#[unsafe(no_mangle)]
#[allow(non_upper_case_globals)]
pub static Radio: Radio_s = Radio_s {
    Init: Some(RadioInit),
    GetStatus: Some(RadioGetStatus),
    SetModem: Some(RadioSetModem),
    SetChannel: Some(RadioSetChannel),
    IsChannelFree: Some(RadioIsChannelFree),
    Random: Some(RadioRandom),
    SetRxConfig: Some(RadioSetRxConfig),
    SetTxConfig: Some(RadioSetTxConfig),
    CheckRfFrequency: Some(RadioCheckRfFrequency),
    TimeOnAir: Some(RadioTimeOnAir),
    Send: Some(RadioSend),
    Sleep: Some(RadioSleep),
    Standby: Some(RadioStandby),
    Rx: Some(RadioRx),
    StartCad: Some(RadioStartCad),
    SetTxContinuousWave: Some(RadioSetTxContinuousWave),
    Rssi: Some(RadioRssi),
    Write: Some(RadioWrite),
    Read: Some(RadioRead),
    WriteBuffer: Some(RadioWriteBuffer),
    ReadBuffer: Some(RadioReadBuffer),
    SetMaxPayloadLength: Some(RadioSetMaxPayloadLength),
    SetPublicNetwork: Some(RadioSetPublicNetwork),
    GetWakeupTime: Some(RadioGetWakeupTime),
    IrqProcess: Some(RadioIrqProcess),
    RxBoosted: Some(RadioRxBoosted),
    SetRxDutyCycle: Some(RadioSetRxDutyCycle),
};
