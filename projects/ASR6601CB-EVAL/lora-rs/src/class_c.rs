#![allow(clippy::field_reassign_with_default)]

use core::ptr;

use crate::ffi;

/// 30 seconds between transmissions (ms)
pub const APP_TX_DUTYCYCLE: u32 = 30_000;
/// random delay range on top of `APP_TX_DUTYCYCLE`
pub const APP_TX_DUTYCYCLE_RND: u32 = 1_000;

/// SF12 - BW125 /// lora/mac/region/Region.h
pub const LORAWAN_DEFAULT_DATARATE: u8 = ffi::DR_0;
/// wether or not to expect ACK for sent messages
pub const LORAWAN_CONFIRMED_MSG_ON: bool = true;
/// adaptive data rate
pub const LORAWAN_ADR_ON: bool = true;
/// think of this as a channel
pub const LORAWAN_APP_PORT: u8 = 2;
/// bigger buffer size just in case, currently we only send 4 bytes
pub const LORAWAN_APP_DATA_MAX_SIZE: usize = 16;

/// EU 868 MHz region
pub const ACTIVE_REGION: ffi::eLoRaMacRegion_t = ffi::LORAMAC_REGION_EU868;

/// lora state
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DeviceState {
    /// app start
    Init,
    /// joining network
    Join,
    /// sending a packet
    Send,
    /// waiting for next transmission opportunity
    Cycle,
    /// sleeping until an event occurs
    Sleep,
}

/// application state
struct App {
    /// Device EUI, 8 bytes
    dev_eui: [u8; 8],
    /// Application (Join) EUI, 8 bytes
    app_eui: [u8; 8],
    /// Application Key, 16 bytes
    app_key: [u8; 16],

    /// Application port (channel)
    app_port: u8,
    /// size of the data to be sent
    app_data_size: u16,
    /// buffer for data to be sent
    app_data: [u8; LORAWAN_APP_DATA_MAX_SIZE],
    /// whether or not to request ACK for sent messages
    is_tx_confirmed: bool,
    /// time between transmissions
    tx_duty_cycle_time: u32,
    /// whether or not it's time to send next packet
    next_tx: bool,
    /// current device state
    device_state: DeviceState,

    /// timer for scheduling next packet transmission
    tx_next_packet_timer: ffi::TimerEvent_t,
}

impl App {
    /// create application instance
    pub const fn new() -> Self {
        Self {
            dev_eui: [0x70, 0xB3, 0xD5, 0x7E, 0xD0, 0x07, 0x5C, 0xD4],
            app_eui: [0xAC, 0x1F, 0x09, 0xFF, 0xFE, 0x21, 0x7E, 0x7D],
            app_key: [
                0xEB, 0xB4, 0xEB, 0x14, 0x6C, 0xF1, 0xF9, 0x02, 0xED, 0x7B, 0xE1, 0x7B, 0x71, 0x31,
                0x35, 0xD2,
            ],

            app_port: LORAWAN_APP_PORT,
            app_data_size: 4,
            app_data: [0u8; LORAWAN_APP_DATA_MAX_SIZE],
            is_tx_confirmed: LORAWAN_CONFIRMED_MSG_ON,
            tx_duty_cycle_time: APP_TX_DUTYCYCLE,
            next_tx: true,
            device_state: DeviceState::Init,

            tx_next_packet_timer: ffi::TimerEvent_s {
                Timestamp: 0,
                ReloadValue: 0,
                IsRunning: false,
                Callback: None,
                Next: ptr::null_mut(),
            },
        }
    }

    /// transmit data
    fn prepare_tx_frame(&mut self, _port: u8) {
        self.app_data_size = 4;
        self.app_data[0] = 0x00;
        self.app_data[1] = 0x01;
        self.app_data[2] = 0x02;
        self.app_data[3] = 0x03;
    }

    /// true on errorprepare_tx_frame
    fn send_frame(&mut self) -> bool {
        let mut mcps_req: ffi::McpsReq_t = Default::default();
        let mut tx_info: ffi::LoRaMacTxInfo_t = Default::default();

        let q = unsafe { ffi::LoRaMacQueryTxPossible(self.app_data_size as u8, &mut tx_info) };
        if q != ffi::LORAMAC_STATUS_OK {
            // Send empty frame to flush MAC commands
            mcps_req.Type = ffi::MCPS_UNCONFIRMED;
            mcps_req.Req.Unconfirmed.fBuffer = ptr::null_mut();
            mcps_req.Req.Unconfirmed.fBufferSize = 0;
            mcps_req.Req.Unconfirmed.Datarate = LORAWAN_DEFAULT_DATARATE as i8;
        } else if !self.is_tx_confirmed {
            mcps_req.Type = ffi::MCPS_UNCONFIRMED;
            mcps_req.Req.Unconfirmed.fPort = self.app_port;
            mcps_req.Req.Unconfirmed.fBuffer = self.app_data.as_mut_ptr().cast();
            mcps_req.Req.Unconfirmed.fBufferSize = self.app_data_size;
            mcps_req.Req.Unconfirmed.Datarate = LORAWAN_DEFAULT_DATARATE as i8;
        } else {
            mcps_req.Type = ffi::MCPS_CONFIRMED;
            mcps_req.Req.Confirmed.fPort = self.app_port;
            mcps_req.Req.Confirmed.fBuffer = self.app_data.as_mut_ptr().cast();
            mcps_req.Req.Confirmed.fBufferSize = self.app_data_size;
            mcps_req.Req.Confirmed.NbTrials = 8;
            mcps_req.Req.Confirmed.Datarate = LORAWAN_DEFAULT_DATARATE as i8;
        }

        let s = unsafe { ffi::LoRaMacMcpsRequest(&mut mcps_req) };
        if s == ffi::LORAMAC_STATUS_OK {
            return false; // no error
        }

        true // error
    }

    /// called when it's time to send next packet
    fn on_tx_next_packet_timer_event(&mut self) {
        let mut mib_req: ffi::MibRequestConfirm_t = ffi::MibRequestConfirm_t {
            Type: ffi::MIB_NETWORK_JOINED,
            ..Default::default()
        };

        unsafe { ffi::TimerStop(&mut self.tx_next_packet_timer) };

        let status = unsafe { ffi::LoRaMacMibGetRequestConfirm(&mut mib_req) };
        if status == ffi::LORAMAC_STATUS_OK {
            let joined = unsafe { mib_req.Param.IsNetworkJoined };
            if joined {
                self.device_state = DeviceState::Send;
                self.next_tx = true;
            } else {
                // Not joined → join again
                let mut mlme_req: ffi::MlmeReq_t = Default::default();
                mlme_req.Type = ffi::MLME_JOIN;
                mlme_req.Req.Join.DevEui = self.dev_eui.as_mut_ptr();
                mlme_req.Req.Join.AppEui = self.app_eui.as_mut_ptr();
                mlme_req.Req.Join.AppKey = self.app_key.as_mut_ptr();

                let st = unsafe { ffi::LoRaMacMlmeRequest(&mut mlme_req) };
                if st == ffi::LORAMAC_STATUS_OK {
                    self.device_state = DeviceState::Sleep;
                } else {
                    self.device_state = DeviceState::Cycle;
                }
            }
        }
    }

    /// MAC layer confirm callback (e.g. after sending a packet)
    fn mcps_confirm(&mut self, mcps_confirm: &ffi::McpsConfirm_t) {
        if mcps_confirm.Status == ffi::LORAMAC_EVENT_INFO_STATUS_OK {
            match mcps_confirm.McpsRequest {
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
        self.next_tx = true;
    }

    /// MAC layer indication callback (e.g. after receiving a packet)
    fn mcps_indication(&mut self, mcps_indirection: &ffi::McpsIndication_t) {
        if mcps_indirection.Status != ffi::LORAMAC_EVENT_INFO_STATUS_OK {
            return;
        }

        unsafe {
            ffi::printf(
                c"receive data: rssi = %d, snr = %d, datarate = %d\r\n".as_ptr(),
                mcps_indirection.Rssi as i32,
                mcps_indirection.Snr as i32,
                mcps_indirection.RxDatarate as i32,
            );
        }

        // FramePending → schedule uplink ASAP
        if mcps_indirection.FramePending != 0 {
            self.on_tx_next_packet_timer_event();
        }

        if mcps_indirection.BufferSize != 0 {
            unsafe {
                ffi::printf(c"Received: ".as_ptr());
                for i in 0..(mcps_indirection.BufferSize as usize) {
                    let b = *mcps_indirection.Buffer.add(i);
                    ffi::printf(c"%x ".as_ptr(), b as i32);
                }
                ffi::printf(c"\r\n".as_ptr());
            }
        } else {
            unsafe { ffi::printf(c"\r\n".as_ptr()) };
        }
    }

    /// MAC layer management confirm callback (e.g. after join attempt)
    fn mlme_confirm(&mut self, mlme_confirm: &ffi::MlmeConfirm_t) {
        if mlme_confirm.MlmeRequest == ffi::MLME_JOIN {
            if mlme_confirm.Status == ffi::LORAMAC_EVENT_INFO_STATUS_OK {
                unsafe { ffi::printf(c"Joined\r\n".as_ptr()) };
                self.device_state = DeviceState::Send;
            } else {
                unsafe { ffi::printf(c"Join failed\r\n".as_ptr()) };

                let mut mlme_req: ffi::MlmeReq_t = Default::default();
                mlme_req.Type = ffi::MLME_JOIN;
                mlme_req.Req.Join.DevEui = self.dev_eui.as_mut_ptr();
                mlme_req.Req.Join.AppEui = self.app_eui.as_mut_ptr();
                mlme_req.Req.Join.AppKey = self.app_key.as_mut_ptr();
                mlme_req.Req.Join.NbTrials = 8;

                let st = unsafe { ffi::LoRaMacMlmeRequest(&mut mlme_req) };
                if st == ffi::LORAMAC_STATUS_OK {
                    self.device_state = DeviceState::Sleep;
                } else {
                    self.device_state = DeviceState::Cycle;
                }
            }
        } else if mlme_confirm.MlmeRequest == ffi::MLME_LINK_CHECK
            && mlme_confirm.Status == ffi::LORAMAC_EVENT_INFO_STATUS_OK
        {
            // Check DemodMargin, NbGateways...
        }

        self.next_tx = true;
    }

    /// MAC layer management indication callback (e.g. schedule uplink)
    fn mlme_indication(&mut self, mlme_indication: &ffi::MlmeIndication_t) {
        if mlme_indication.MlmeIndication == ffi::MLME_SCHEDULE_UPLINK {
            self.on_tx_next_packet_timer_event();
        }
    }

    /// update LoRaWAN device parameters (e.g. channels mask, class)
    fn lwan_dev_params_update(&mut self) {
        let mut mib_req: ffi::MibRequestConfirm_t = Default::default();

        // Same mask as C code
        let mut channels_mask_temp: [u16; 6] = [0; 6];
        channels_mask_temp[0] = 0x00FF;

        mib_req.Type = ffi::MIB_CHANNELS_DEFAULT_MASK;
        unsafe {
            mib_req.Param.ChannelsDefaultMask = channels_mask_temp.as_mut_ptr();
            ffi::LoRaMacMibSetRequestConfirm(&mut mib_req);
        }

        mib_req.Type = ffi::MIB_CHANNELS_MASK;
        unsafe {
            mib_req.Param.ChannelsMask = channels_mask_temp.as_mut_ptr();
            ffi::LoRaMacMibSetRequestConfirm(&mut mib_req);
        }

        mib_req.Type = ffi::MIB_DEVICE_CLASS;
        unsafe {
            mib_req.Param.Class = ffi::CLASS_C;
            ffi::LoRaMacMibSetRequestConfirm(&mut mib_req);
        }
    }
}

/// global application state
static mut APP: App = App::new();

/// get battery level in 0..254 (254 = 100%, 0 = dead, 255 = unable to measure)
pub extern "C" fn board_get_battery_level() -> u8 {
    0
}

/// MAC layer confirm
pub extern "C" fn mcps_confirm(mcps_confirm: *mut ffi::McpsConfirm_t) {
    if mcps_confirm.is_null() {
        return;
    }
    unsafe { APP.mcps_confirm(&*mcps_confirm) }
}

/// MAC layer indication (e.g. received data)
pub extern "C" fn mcps_indication(mcps_indication: *mut ffi::McpsIndication_t) {
    if mcps_indication.is_null() {
        return;
    }
    unsafe { APP.mcps_indication(&*mcps_indication) }
}

/// MAC layer management confirm (e.g. join result)
pub extern "C" fn mlme_confirm(mlme_confirm: *mut ffi::MlmeConfirm_t) {
    if mlme_confirm.is_null() {
        return;
    }
    unsafe { APP.mlme_confirm(&*mlme_confirm) }
}

/// MAC layer management indication (e.g. schedule uplink)
pub extern "C" fn mlme_indication(mlme_indication: *mut ffi::MlmeIndication_t) {
    if mlme_indication.is_null() {
        return;
    }
    unsafe { APP.mlme_indication(&*mlme_indication) }
}

/// called when it's time to send next packet
pub extern "C" fn on_tx_next_packet_timer_event() {
    unsafe { APP.on_tx_next_packet_timer_event() }
}

/// application start
pub fn app_start() -> ! {
    unsafe { ffi::printf(c"ClassC app start\r\n".as_ptr()) };

    unsafe { APP.device_state = DeviceState::Init };

    loop {
        // Read state once to avoid borrowing issues in match
        let state = unsafe { APP.device_state };

        match state {
            DeviceState::Init => unsafe {
                // Setup primitives/callbacks
                let mut primitives: ffi::LoRaMacPrimitives_t = Default::default();
                primitives.MacMcpsConfirm = Some(mcps_confirm);
                primitives.MacMcpsIndication = Some(mcps_indication);
                primitives.MacMlmeConfirm = Some(mlme_confirm);
                primitives.MacMlmeIndication = Some(mlme_indication);

                let mut callbacks: ffi::LoRaMacCallback_t = Default::default();
                callbacks.GetBatteryLevel = Some(board_get_battery_level);

                ffi::LoRaMacInitialization(&mut primitives, &mut callbacks, ACTIVE_REGION);

                ffi::TimerInit(
                    &mut APP.tx_next_packet_timer,
                    Some(on_tx_next_packet_timer_event),
                );

                // ADR
                let mut mib_req: ffi::MibRequestConfirm_t = Default::default();
                mib_req.Type = ffi::MIB_ADR;
                mib_req.Param.AdrEnable = LORAWAN_ADR_ON;
                ffi::LoRaMacMibSetRequestConfirm(&mut mib_req);

                // Public network
                mib_req.Type = ffi::MIB_PUBLIC_NETWORK;
                mib_req.Param.EnablePublicNetwork = true; // Commissioning.h LORAWAN_PUBLIC_NETWORK
                ffi::LoRaMacMibSetRequestConfirm(&mut mib_req);

                APP.lwan_dev_params_update();

                APP.device_state = DeviceState::Join;
            },

            DeviceState::Join => unsafe {
                let mut mlme_req: ffi::MlmeReq_t = Default::default();

                mlme_req.Type = ffi::MLME_JOIN;
                mlme_req.Req.Join.DevEui = APP.dev_eui.as_mut_ptr();
                mlme_req.Req.Join.AppEui = APP.app_eui.as_mut_ptr();
                mlme_req.Req.Join.AppKey = APP.app_key.as_mut_ptr();
                mlme_req.Req.Join.NbTrials = 8;

                if ffi::LoRaMacMlmeRequest(&mut mlme_req) == ffi::LORAMAC_STATUS_OK {
                    APP.device_state = DeviceState::Sleep;
                } else {
                    APP.device_state = DeviceState::Cycle;
                }
            },

            DeviceState::Send => unsafe {
                if APP.next_tx {
                    APP.prepare_tx_frame(APP.app_port);
                    APP.next_tx = APP.send_frame();
                }

                APP.tx_duty_cycle_time =
                    APP_TX_DUTYCYCLE + ffi::randr(0, APP_TX_DUTYCYCLE_RND as i32) as u32;
                APP.device_state = DeviceState::Cycle;
            },

            DeviceState::Cycle => unsafe {
                APP.device_state = DeviceState::Sleep;
                ffi::TimerSetValue(&mut APP.tx_next_packet_timer, APP.tx_duty_cycle_time);
                ffi::TimerStart(&mut APP.tx_next_packet_timer);
            },

            DeviceState::Sleep => unsafe {
                // Process Radio IRQ
                (ffi::Radio.IrqProcess.unwrap())(); // assume ffi wraps `Radio.IrqProcess()`
            },
        }
    }
}
