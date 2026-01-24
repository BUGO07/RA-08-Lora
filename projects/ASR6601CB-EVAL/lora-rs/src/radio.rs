use crate::ffi::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum States_t {
    LowPower,
    Rx,
    RxTimeout,
    RxError,
    Tx,
    TxTimeout,
}

pub static mut STATE: States_t = States_t::LowPower;

pub static mut BUFFER_SIZE: u16 = 5;
pub static mut BUFFER: [u8; 5] = [0; 5];

pub static mut RSSI_VALUE: i8 = 0;
pub static mut SNR_VALUE: i8 = 0;

const RF_FREQUENCY: u32 = 470000000;
const TX_OUTPUT_POWER: i8 = 14;

pub static mut EVENTS: RadioEvents_t = RadioEvents_t {
    TxDone: None,
    RxDone: None,
    TxTimeout: None,
    RxTimeout: None,
    RxError: None,
    FhssChangeChannel: None,
    CadDone: None,
};

pub extern "C" fn on_tx_done() {
    unsafe {
        (Radio.Sleep.unwrap())();
        STATE = States_t::Tx;
    }
}

pub extern "C" fn on_rx_done(payload: *mut u8, size: u16, rssi: i16, snr: i8) {
    unsafe {
        (Radio.Sleep.unwrap())();
        BUFFER_SIZE = size;

        core::ptr::copy_nonoverlapping(payload, BUFFER.as_mut_ptr(), BUFFER_SIZE as usize);

        RSSI_VALUE = rssi as i8;
        SNR_VALUE = snr;
        STATE = States_t::Rx;
    }
}

pub extern "C" fn on_tx_timeout() {
    unsafe {
        (Radio.Sleep.unwrap())();
        STATE = States_t::TxTimeout;
    }
}

pub extern "C" fn on_rx_timeout() {
    unsafe {
        printf(c"on_rx_timeout\r\n".as_ptr());
        (Radio.Sleep.unwrap())();
        STATE = States_t::RxTimeout;
    }
}

pub extern "C" fn on_rx_error() {
    unsafe {
        (Radio.Sleep.unwrap())();
        STATE = States_t::RxError;
    }
}

pub fn init() {
    unsafe {
        EVENTS.TxDone = Some(on_tx_done);
        EVENTS.RxDone = Some(on_rx_done);
        EVENTS.TxTimeout = Some(on_tx_timeout);
        EVENTS.RxTimeout = Some(on_rx_timeout);
        EVENTS.RxError = Some(on_rx_error);

        (Radio.Init.unwrap())(&mut EVENTS);
        (Radio.SetChannel.unwrap())(RF_FREQUENCY);

        (Radio.SetTxConfig.unwrap())(
            MODEM_LORA,
            TX_OUTPUT_POWER,
            0,
            0,
            7,
            1,
            8,
            false,
            true,
            false,
            0,
            false,
            3000,
        );

        (Radio.SetRxConfig.unwrap())(
            MODEM_LORA, 0, 7, 1, 0, 8, 0, false, 0, true, false, 0, false, true,
        );

        (Radio.Rx.unwrap())(crate::RX_TIMEOUT_VALUE);
    }
}
