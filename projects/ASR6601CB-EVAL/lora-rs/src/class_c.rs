//! Rustified LoRaMac Class C device example.
//! Assumptions (per your request):
//! - There is an `ffi` module that exposes the same types, constants, globals,
//!   and functions as the C code (LoRaMac, Timer, Radio, etc.).
//! - The ABI matches (repr(C) where needed).
//! - This is "embedded-style" Rust: no heap required, minimal std use.
//!
//! Notes:
//! - We avoid global mutable C statics by putting state in a `static mut APP`
//!   and exposing C callbacks that delegate into it.
//! - This keeps the structure close to the original, but “Rust-shaped”.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]

use core::ptr;

use crate::ffi;

// -------------------------- "Constants" --------------------------

const APP_TX_DUTYCYCLE: u32 = 30_000;
const APP_TX_DUTYCYCLE_RND: u32 = 1_000;

const LORAWAN_DEFAULT_DATARATE: u8 = ffi::DR_0;
const LORAWAN_CONFIRMED_MSG_ON: bool = true;
const LORAWAN_ADR_ON: u8 = 1;
const LORAWAN_APP_PORT: u8 = 2;

const LORAWAN_APP_DATA_MAX_SIZE: usize = 16;

const ACTIVE_REGION: ffi::eLoRaMacRegion_t = ffi::LORAMAC_REGION_EU868;

// -------------------------- App State --------------------------

#[derive(Copy, Clone, Eq, PartialEq)]
enum DeviceState {
    Init,
    Join,
    Send,
    Cycle,
    Sleep,
}

struct App {
    // Commissioning params (like the macros)
    DevEui: [u8; 8],
    AppEui: [u8; 8],
    AppKey: [u8; 16],

    // ABP params (only used if OTAA disabled)
    #[allow(unused)]
    NwkSKey: [u8; 16],
    #[allow(unused)]
    AppSKey: [u8; 16],
    #[allow(unused)]
    DevAddr: u32,

    // Runtime
    AppPort: u8,
    AppDataSize: u16,
    AppData: [u8; LORAWAN_APP_DATA_MAX_SIZE],
    IsTxConfirmed: bool,
    TxDutyCycleTime: u32,
    NextTx: bool,
    DeviceState: DeviceState,

    TxNextPacketTimer: ffi::TimerEvent_t,
}

impl App {
    const fn new() -> Self {
        Self {
            // Commissioning.h
            DevEui: [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
            AppEui: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            AppKey: [
                0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x01,
            ],
            NwkSKey: [
                0xd7, 0x2c, 0x78, 0x75, 0x8c, 0xdc, 0xca, 0xbf, 0x55, 0xee, 0x4a, 0x77, 0x8d, 0x16,
                0xef, 0x67,
            ],
            AppSKey: [
                0x15, 0xb1, 0xd0, 0xef, 0xa4, 0x63, 0xdf, 0xbe, 0x3d, 0x11, 0x18, 0x1e, 0x1e, 0xc7,
                0xda, 0x85,
            ],
            DevAddr: ffi::LORAWAN_DEVICE_ADDRESS,

            AppPort: LORAWAN_APP_PORT,
            AppDataSize: 4,
            AppData: [0u8; LORAWAN_APP_DATA_MAX_SIZE],
            IsTxConfirmed: LORAWAN_CONFIRMED_MSG_ON,
            TxDutyCycleTime: APP_TX_DUTYCYCLE,
            NextTx: true,
            DeviceState: DeviceState::Init,

            TxNextPacketTimer: unsafe { core::mem::zeroed() },
        }
    }

    fn prepare_tx_frame(&mut self, _port: u8) {
        self.AppDataSize = 4;
        self.AppData[0] = 0x00;
        self.AppData[1] = 0x01;
        self.AppData[2] = 0x02;
        self.AppData[3] = 0x03;
    }

    /// Returns `true` on error (matches original C behavior)
    fn send_frame(&mut self) -> bool {
        let mut mcpsReq: ffi::McpsReq_t = unsafe { core::mem::zeroed() };
        let mut txInfo: ffi::LoRaMacTxInfo_t = unsafe { core::mem::zeroed() };

        let q = unsafe { ffi::LoRaMacQueryTxPossible(self.AppDataSize as u8, &mut txInfo) };
        if q != ffi::LORAMAC_STATUS_OK {
            // Send empty frame to flush MAC commands
            mcpsReq.Type = ffi::MCPS_UNCONFIRMED;
            mcpsReq.Req.Unconfirmed.fBuffer = ptr::null_mut();
            mcpsReq.Req.Unconfirmed.fBufferSize = 0;
            mcpsReq.Req.Unconfirmed.Datarate = LORAWAN_DEFAULT_DATARATE as i8;
        } else if !self.IsTxConfirmed {
            mcpsReq.Type = ffi::MCPS_UNCONFIRMED;
            mcpsReq.Req.Unconfirmed.fPort = self.AppPort;
            mcpsReq.Req.Unconfirmed.fBuffer = self.AppData.as_mut_ptr().cast();
            mcpsReq.Req.Unconfirmed.fBufferSize = self.AppDataSize;
            mcpsReq.Req.Unconfirmed.Datarate = LORAWAN_DEFAULT_DATARATE as i8;
        } else {
            mcpsReq.Type = ffi::MCPS_CONFIRMED;
            mcpsReq.Req.Confirmed.fPort = self.AppPort;
            mcpsReq.Req.Confirmed.fBuffer = self.AppData.as_mut_ptr().cast();
            mcpsReq.Req.Confirmed.fBufferSize = self.AppDataSize;
            mcpsReq.Req.Confirmed.NbTrials = 8;
            mcpsReq.Req.Confirmed.Datarate = LORAWAN_DEFAULT_DATARATE as i8;
        }

        let s = unsafe { ffi::LoRaMacMcpsRequest(&mut mcpsReq) };
        if s == ffi::LORAMAC_STATUS_OK {
            return false; // no error
        }
        return true; // error
    }

    fn on_tx_next_packet_timer_event(&mut self) {
        let mut mibReq: ffi::MibRequestConfirm_t = unsafe { core::mem::zeroed() };
        mibReq.Type = ffi::MIB_NETWORK_JOINED;

        unsafe { ffi::TimerStop(&mut self.TxNextPacketTimer) };

        let status = unsafe { ffi::LoRaMacMibGetRequestConfirm(&mut mibReq) };
        if status == ffi::LORAMAC_STATUS_OK {
            let joined = unsafe { mibReq.Param.IsNetworkJoined };
            if joined {
                self.DeviceState = DeviceState::Send;
                self.NextTx = true;
            } else {
                // Not joined → join again
                let mut mlmeReq: ffi::MlmeReq_t = unsafe { core::mem::zeroed() };
                mlmeReq.Type = ffi::MLME_JOIN;
                mlmeReq.Req.Join.DevEui = self.DevEui.as_mut_ptr();
                mlmeReq.Req.Join.AppEui = self.AppEui.as_mut_ptr();
                mlmeReq.Req.Join.AppKey = self.AppKey.as_mut_ptr();

                let st = unsafe { ffi::LoRaMacMlmeRequest(&mut mlmeReq) };
                if st == ffi::LORAMAC_STATUS_OK {
                    self.DeviceState = DeviceState::Sleep;
                } else {
                    self.DeviceState = DeviceState::Cycle;
                }
            }
        }
    }

    fn mcps_confirm(&mut self, mcpsConfirm: &ffi::McpsConfirm_t) {
        if mcpsConfirm.Status == ffi::LORAMAC_EVENT_INFO_STATUS_OK {
            match mcpsConfirm.McpsRequest {
                x if x == ffi::MCPS_UNCONFIRMED => {
                    // Check Datarate, TxPower...
                }
                x if x == ffi::MCPS_CONFIRMED => {
                    // Check AckReceived, NbTrials...
                }
                x if x == ffi::MCPS_PROPRIETARY => {}
                _ => {}
            }
        }
        self.NextTx = true;
    }

    fn mcps_indication(&mut self, mcpsIndication: &ffi::McpsIndication_t) {
        if mcpsIndication.Status != ffi::LORAMAC_EVENT_INFO_STATUS_OK {
            return;
        }

        unsafe {
            ffi::printf(
                c"receive data: rssi = %d, snr = %d, datarate = %d\r\n".as_ptr(),
                mcpsIndication.Rssi as i32,
                mcpsIndication.Snr as i32,
                mcpsIndication.RxDatarate as i32,
            );
        }

        // FramePending → schedule uplink ASAP
        if mcpsIndication.FramePending != 0 {
            self.on_tx_next_packet_timer_event();
        }

        if mcpsIndication.BufferSize != 0 {
            unsafe {
                ffi::printf(c"Received: ".as_ptr());
                for i in 0..(mcpsIndication.BufferSize as usize) {
                    let b = *mcpsIndication.Buffer.add(i);
                    ffi::printf(c"%x ".as_ptr(), b as i32);
                }
                ffi::printf(c"\r\n".as_ptr());
            }
        } else {
            unsafe { ffi::printf(c"\r\n".as_ptr()) };
        }
    }

    fn mlme_confirm(&mut self, mlmeConfirm: &ffi::MlmeConfirm_t) {
        if mlmeConfirm.MlmeRequest == ffi::MLME_JOIN {
            if mlmeConfirm.Status == ffi::LORAMAC_EVENT_INFO_STATUS_OK {
                unsafe { ffi::printf(c"joined\r\n".as_ptr()) };
                self.DeviceState = DeviceState::Send;
            } else {
                unsafe { ffi::printf(c"join failed\r\n".as_ptr()) };

                let mut mlmeReq: ffi::MlmeReq_t = unsafe { core::mem::zeroed() };
                mlmeReq.Type = ffi::MLME_JOIN;
                mlmeReq.Req.Join.DevEui = self.DevEui.as_mut_ptr();
                mlmeReq.Req.Join.AppEui = self.AppEui.as_mut_ptr();
                mlmeReq.Req.Join.AppKey = self.AppKey.as_mut_ptr();
                mlmeReq.Req.Join.NbTrials = 8;

                let st = unsafe { ffi::LoRaMacMlmeRequest(&mut mlmeReq) };
                if st == ffi::LORAMAC_STATUS_OK {
                    self.DeviceState = DeviceState::Sleep;
                } else {
                    self.DeviceState = DeviceState::Cycle;
                }
            }
        } else if mlmeConfirm.MlmeRequest == ffi::MLME_LINK_CHECK
            && mlmeConfirm.Status == ffi::LORAMAC_EVENT_INFO_STATUS_OK
        {
            // Check DemodMargin, NbGateways...
        }

        self.NextTx = true;
    }

    fn mlme_indication(&mut self, mlmeIndication: &ffi::MlmeIndication_t) {
        if mlmeIndication.MlmeIndication == ffi::MLME_SCHEDULE_UPLINK {
            self.on_tx_next_packet_timer_event();
        }
    }

    fn lwan_dev_params_update(&mut self) {
        let mut mibReq: ffi::MibRequestConfirm_t = unsafe { core::mem::zeroed() };

        // Same mask as C code
        let mut channelsMaskTemp: [u16; 6] = [0; 6];
        channelsMaskTemp[0] = 0x00FF;

        mibReq.Type = ffi::MIB_CHANNELS_DEFAULT_MASK;
        unsafe {
            mibReq.Param.ChannelsDefaultMask = channelsMaskTemp.as_mut_ptr();
            ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);
        }

        mibReq.Type = ffi::MIB_CHANNELS_MASK;
        unsafe {
            mibReq.Param.ChannelsMask = channelsMaskTemp.as_mut_ptr();
            ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);
        }

        mibReq.Type = ffi::MIB_DEVICE_CLASS;
        unsafe {
            mibReq.Param.Class = ffi::CLASS_C;
            ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);
        }
    }
}

// -------------------------- Global App + C Callbacks --------------------------
// This mirrors the C pattern where callbacks are plain function pointers.

static mut APP: App = App::new();

pub extern "C" fn BoardGetBatteryLevel() -> u8 {
    0
}

pub extern "C" fn McpsConfirm(mcpsConfirm: *mut ffi::McpsConfirm_t) {
    if mcpsConfirm.is_null() {
        return;
    }
    unsafe { APP.mcps_confirm(&*mcpsConfirm) }
}

pub extern "C" fn McpsIndication(mcpsIndication: *mut ffi::McpsIndication_t) {
    if mcpsIndication.is_null() {
        return;
    }
    unsafe { APP.mcps_indication(&*mcpsIndication) }
}

pub extern "C" fn MlmeConfirm(mlmeConfirm: *mut ffi::MlmeConfirm_t) {
    if mlmeConfirm.is_null() {
        return;
    }
    unsafe { APP.mlme_confirm(&*mlmeConfirm) }
}

pub extern "C" fn MlmeIndication(mlmeIndication: *mut ffi::MlmeIndication_t) {
    if mlmeIndication.is_null() {
        return;
    }
    unsafe { APP.mlme_indication(&*mlmeIndication) }
}

pub extern "C" fn OnTxNextPacketTimerEvent() {
    unsafe { APP.on_tx_next_packet_timer_event() }
}

// -------------------------- Entry Point --------------------------

pub fn app_start() -> ! {
    unsafe { ffi::printf(c"ClassC app start\r\n".as_ptr()) };

    unsafe { APP.DeviceState = DeviceState::Init };

    loop {
        // Read state once to avoid borrowing issues in match
        let state = unsafe { APP.DeviceState };

        match state {
            DeviceState::Init => unsafe {
                // Setup primitives/callbacks
                let mut primitives: ffi::LoRaMacPrimitives_t = core::mem::zeroed();
                primitives.MacMcpsConfirm = Some(McpsConfirm);
                primitives.MacMcpsIndication = Some(McpsIndication);
                primitives.MacMlmeConfirm = Some(MlmeConfirm);
                primitives.MacMlmeIndication = Some(MlmeIndication);

                let mut callbacks: ffi::LoRaMacCallback_t = core::mem::zeroed();
                callbacks.GetBatteryLevel = Some(BoardGetBatteryLevel);

                ffi::LoRaMacInitialization(&mut primitives, &mut callbacks, ACTIVE_REGION);

                ffi::TimerInit(&mut APP.TxNextPacketTimer, Some(OnTxNextPacketTimerEvent));

                // ADR
                let mut mibReq: ffi::MibRequestConfirm_t = core::mem::zeroed();
                mibReq.Type = ffi::MIB_ADR;
                mibReq.Param.AdrEnable = LORAWAN_ADR_ON != 0;
                ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);

                // Public network
                mibReq.Type = ffi::MIB_PUBLIC_NETWORK;
                mibReq.Param.EnablePublicNetwork = true; // Commissioning.h LORAWAN_PUBLIC_NETWORK
                ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);

                APP.lwan_dev_params_update();

                APP.DeviceState = DeviceState::Join;
            },

            DeviceState::Join => unsafe {
                if ffi::OVER_THE_AIR_ACTIVATION != 0 {
                    let mut mlmeReq: ffi::MlmeReq_t = core::mem::zeroed();

                    // If you want unique DevEUI, your ffi could provide BoardGetUniqueId:
                    // ffi::BoardGetUniqueId(APP.DevEui.as_mut_ptr());

                    mlmeReq.Type = ffi::MLME_JOIN;
                    mlmeReq.Req.Join.DevEui = APP.DevEui.as_mut_ptr();
                    mlmeReq.Req.Join.AppEui = APP.AppEui.as_mut_ptr();
                    mlmeReq.Req.Join.AppKey = APP.AppKey.as_mut_ptr();
                    mlmeReq.Req.Join.NbTrials = 8;

                    if ffi::LoRaMacMlmeRequest(&mut mlmeReq) == ffi::LORAMAC_STATUS_OK {
                        APP.DeviceState = DeviceState::Sleep;
                    } else {
                        APP.DeviceState = DeviceState::Cycle;
                    }
                } else {
                    // ABP branch (mirrors C)
                    let mut mibReq: ffi::MibRequestConfirm_t = core::mem::zeroed();

                    mibReq.Type = ffi::MIB_NET_ID;
                    mibReq.Param.NetID = ffi::LORAWAN_NETWORK_ID as u32;
                    ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);

                    mibReq.Type = ffi::MIB_DEV_ADDR;
                    mibReq.Param.DevAddr = APP.DevAddr;
                    ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);

                    mibReq.Type = ffi::MIB_NWK_SKEY;
                    mibReq.Param.NwkSKey = APP.NwkSKey.as_mut_ptr();
                    ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);

                    mibReq.Type = ffi::MIB_APP_SKEY;
                    mibReq.Param.AppSKey = APP.AppSKey.as_mut_ptr();
                    ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);

                    mibReq.Type = ffi::MIB_NETWORK_JOINED;
                    mibReq.Param.IsNetworkJoined = true;
                    ffi::LoRaMacMibSetRequestConfirm(&mut mibReq);

                    APP.DeviceState = DeviceState::Send;
                }
            },

            DeviceState::Send => unsafe {
                if APP.NextTx {
                    APP.prepare_tx_frame(APP.AppPort);
                    APP.NextTx = APP.send_frame(); // true means error; matches C code
                }

                APP.TxDutyCycleTime =
                    APP_TX_DUTYCYCLE + ffi::randr(0, APP_TX_DUTYCYCLE_RND as i32) as u32;
                APP.DeviceState = DeviceState::Cycle;
            },

            DeviceState::Cycle => unsafe {
                APP.DeviceState = DeviceState::Sleep;
                ffi::TimerSetValue(&mut APP.TxNextPacketTimer, APP.TxDutyCycleTime);
                ffi::TimerStart(&mut APP.TxNextPacketTimer);
            },

            DeviceState::Sleep => unsafe {
                // Process Radio IRQ
                (ffi::Radio.IrqProcess.unwrap())(); // assume ffi wraps `Radio.IrqProcess()`
            },
        }
    }
}
