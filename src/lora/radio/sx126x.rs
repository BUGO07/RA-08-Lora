use crate::{
    lora::driver::sx1262_board::{
        sx126x_ant_sw_off, sx126x_ant_sw_on, sx126x_lorac_init, sx126x_read_buffer,
        sx126x_read_command, sx126x_read_register, sx126x_read_registers, sx126x_reset,
        sx126x_wait_on_busy, sx126x_wakeup, sx126x_write_buffer, sx126x_write_command,
        sx126x_write_registers,
    },
    peripherals::delay::delay_ms,
};

/// Radio complete Wake-up Time with margin for temperature compensation (ms)
pub const RADIO_WAKEUP_TIME: usize = 3;

/// Compensation delay for SetAutoTx/Rx functions in 15.625 microseconds
pub const AUTO_RX_TX_OFFSET: usize = 2;

/// LFSR initial value to compute IBM type CRC
pub const CRC_IBM_SEED: u16 = 0xFFFF;

/// LFSR initial value to compute CCIT type CRC
pub const CRC_CCITT_SEED: u16 = 0x1D0F;

/// Polynomial used to compute IBM CRC
pub const CRC_POLYNOMIAL_IBM: u16 = 0x8005;

/// Polynomial used to compute CCIT CRC
pub const CRC_POLYNOMIAL_CCITT: u16 = 0x1021;

/// The address of the register holding the first byte defining the CRC seed
pub const REG_LR_CRCSEEDBASEADDR: u16 = 0x06BC;

/// The address of the register holding the first byte defining the CRC polynomial
pub const REG_LR_CRCPOLYBASEADDR: u16 = 0x06BE;

/// The address of the register holding the first byte defining the whitening seed
pub const REG_LR_WHITSEEDBASEADDR_MSB: u16 = 0x06B8;
pub const REG_LR_WHITSEEDBASEADDR_LSB: u16 = 0x06B9;

/// The address of the register holding the packet configuration
pub const REG_LR_PACKETPARAMS: u16 = 0x0704;

/// The address of the register holding the payload size
pub const REG_LR_PAYLOADLENGTH: u16 = 0x0702;

/// The addresses of the registers holding SyncWords values
pub const REG_LR_SYNCWORDBASEADDRESS: u16 = 0x06C0;

/// The addresses of the register holding LoRa Modem SyncWord value
pub const REG_LR_SYNCWORD: u16 = 0x0740;

/// Syncword for Private LoRa networks
pub const LORA_MAC_PRIVATE_SYNCWORD: u16 = 0x1424;

/// Syncword for Public LoRa networks
pub const LORA_MAC_PUBLIC_SYNCWORD: u16 = 0x3444;

/// The address of the register giving a 4 bytes random number
pub const RANDOM_NUMBER_GENERATORBASEADDR: u16 = 0x0819;

/// The address of the register holding RX Gain value (0x94: power saving, 0x96: rx boosted)
pub const REG_RX_GAIN: u16 = 0x08AC;

/// Change the value on the device internal trimming capacitor
pub const REG_XTA_TRIM: u16 = 0x0911;

/// Set the current max value in the over current protection
pub const REG_OCP: u16 = 0x08E7;

/// Structure describing the radio status
#[derive(Debug, Clone, Copy)]
pub struct RadioStatus {
    pub value: u8,
}

impl RadioStatus {
    pub fn reserved(&self) -> u8 {
        self.value & 0x01
    }

    /// Command status
    pub fn cmd_status(&self) -> u8 {
        (self.value >> 1) & 0x07
    }

    /// Chip mode
    pub fn chip_mode(&self) -> u8 {
        (self.value >> 4) & 0x07
    }

    /// Flag for CPU radio busy
    pub fn cpu_busy(&self) -> bool {
        (self.value >> 7) & 0x01 != 0
    }
}

/// Error codes for callback functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IrqErrorCode {
    HeaderError = 0x01,
    SyncwordError = 0x02,
    CrcError = 0x04,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IrqPblSyncHeaderCode {
    PblDetect = 0x01,
    SyncwordValid = 0x02,
    HeaderValid = 0x04,
}

/// Represents the operating mode the radio is actually running
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioOperatingModes {
    /// The radio is in sleep mode
    Sleep = 0x00,
    /// The radio is in standby mode with RC oscillator
    StdbyRc = 0x01,
    /// The radio is in standby mode with XOSC oscillator
    StdbyXosc = 0x02,
    /// The radio is in frequency synthesis mode
    Fs = 0x03,
    /// The radio is in transmit mode
    Tx = 0x04,
    /// The radio is in receive mode
    Rx = 0x05,
    /// The radio is in receive duty cycle mode
    RxDc = 0x06,
    /// The radio is in channel activity detection mode
    Cad = 0x07,
}

/// Declares the oscillator in use while in standby mode
///
/// Using the STDBY_RC standby mode allows reducing the energy consumption.
/// STDBY_XOSC should be used for time critical applications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioStandbyModes {
    StdbyRc = 0x00,
    StdbyXosc = 0x01,
}

/// Declares the power regulation used to power the device
///
/// This command allows the user to specify if DC-DC or LDO is used for power regulation.
/// Using only LDO implies that the Rx or Tx current is doubled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioRegulatorMode {
    /// Default
    UseLdo = 0x00,
    UseDcdc = 0x01,
}

/// Represents the possible packet type (i.e. modem) used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioPacketTypes {
    Gfsk = 0x00,
    LoRa = 0x01,
    None = 0x0F,
}

/// Represents the ramping time for power amplifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioRampTimes {
    Ramp10Us = 0x00,
    Ramp20Us = 0x01,
    Ramp40Us = 0x02,
    Ramp80Us = 0x03,
    Ramp200Us = 0x04,
    Ramp800Us = 0x05,
    Ramp1700Us = 0x06,
    Ramp3400Us = 0x07,
}

/// Represents the number of symbols to be used for channel activity detection operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioLoRaCadSymbols {
    CadOn01Symbol = 0x00,
    CadOn02Symbol = 0x01,
    CadOn04Symbol = 0x02,
    CadOn08Symbol = 0x03,
    CadOn16Symbol = 0x04,
}

/// Represents the Channel Activity Detection actions after the CAD operation is finished
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioCadExitModes {
    CadOnly = 0x00,
    CadRx = 0x01,
    CadLbt = 0x10,
}

/// Represents the modulation shaping parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioModShapings {
    Off = 0x00,
    GBt03 = 0x08,
    GBt05 = 0x09,
    GBt07 = 0x0A,
    GBt1 = 0x0B,
}

/// Represents the receiver bandwidth
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioRxBandwidth {
    Bw4800 = 0x1F,
    Bw5800 = 0x17,
    Bw7300 = 0x0F,
    Bw9700 = 0x1E,
    Bw11700 = 0x16,
    Bw14600 = 0x0E,
    Bw19500 = 0x1D,
    Bw23400 = 0x15,
    Bw29300 = 0x0D,
    Bw39000 = 0x1C,
    Bw46900 = 0x14,
    Bw58600 = 0x0C,
    Bw78200 = 0x1B,
    Bw93800 = 0x13,
    Bw117300 = 0x0B,
    Bw156200 = 0x1A,
    Bw187200 = 0x12,
    Bw234300 = 0x0A,
    Bw312000 = 0x19,
    Bw373600 = 0x11,
    Bw467000 = 0x09,
}

/// Represents the possible spreading factor values in LoRa packet types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioLoRaSpreadingFactors {
    Sf5 = 0x05,
    Sf6 = 0x06,
    Sf7 = 0x07,
    Sf8 = 0x08,
    Sf9 = 0x09,
    Sf10 = 0x0A,
    Sf11 = 0x0B,
    Sf12 = 0x0C,
}

/// Represents the bandwidth values for LoRa packet type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioLoRaBandwidths {
    Bw500 = 6,
    Bw250 = 5,
    Bw125 = 4,
    Bw062 = 3,
    Bw041 = 10,
    Bw031 = 2,
    Bw020 = 9,
    Bw015 = 1,
    Bw010 = 8,
    Bw007 = 0,
}

/// Represents the coding rate values for LoRa packet type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioLoRaCodingRates {
    Cr4_5 = 0x01,
    Cr4_6 = 0x02,
    Cr4_7 = 0x03,
    Cr4_8 = 0x04,
}

/// Represents the preamble length used to detect the packet on Rx side
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioPreambleDetection {
    /// Preamble detection length off
    Off = 0x00,
    /// Preamble detection length 8 bits
    Detect08Bits = 0x04,
    /// Preamble detection length 16 bits
    Detect16Bits = 0x05,
    /// Preamble detection length 24 bits
    Detect24Bits = 0x06,
    /// Preamble detection length 32 bits
    Detect32Bits = 0x07,
}

/// Represents the possible combinations of SyncWord correlators activated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioAddressComp {
    /// No correlator turned on, i.e. do not search for SyncWord
    FiltOff = 0x00,
    FiltNode = 0x01,
    FiltNodeBroad = 0x02,
}

/// Radio GFSK packet length mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioPacketLengthModes {
    /// The packet is known on both sides, no header included in the packet
    FixedLength = 0x00,
    /// The packet is on variable size, header included
    VariableLength = 0x01,
}

/// Represents the CRC length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioCrcTypes {
    /// No CRC in use
    Off = 0x01,
    OneBytes = 0x00,
    TwoBytes = 0x02,
    OneBytesInv = 0x04,
    TwoBytesInv = 0x06,
    TwoBytesIbm = 0xF1,
    TwoBytesCcit = 0xF2,
}

/// Radio whitening mode activated or deactivated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioDcFree {
    Off = 0x00,
    Whitening = 0x01,
}

/// Holds the Radio lengths mode for the LoRa packet type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioLoRaPacketLengthsMode {
    /// The packet is on variable size, header included (Explicit)
    VariableLength = 0x00,
    /// The packet is known on both sides, no header included in the packet (Implicit)
    FixedLength = 0x01,
}

/// Represents the CRC mode for LoRa packet type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioLoRaCrcModes {
    /// CRC activated
    On = 0x01,
    /// CRC not used
    Off = 0x00,
}

/// Represents the IQ mode for LoRa packet type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioLoRaIQModes {
    Normal = 0x00,
    Inverted = 0x01,
}

/// Represents the voltage used to control the TCXO on/off from DIO3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioTcxoCtrlVoltage {
    Ctrl1_6V = 0x00,
    Ctrl1_7V = 0x01,
    Ctrl1_8V = 0x02,
    Ctrl2_2V = 0x03,
    Ctrl2_4V = 0x04,
    Ctrl2_7V = 0x05,
    Ctrl3_0V = 0x06,
    Ctrl3_3V = 0x07,
}

/// Represents the interruption masks available for the radio
///
/// Note that not all these interruptions are available for all packet types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RadioIrqMasks {
    None = 0x0000,
    TxDone = 0x0001,
    RxDone = 0x0002,
    PreambleDetected = 0x0004,
    SyncwordValid = 0x0008,
    HeaderValid = 0x0010,
    HeaderError = 0x0020,
    CrcError = 0x0040,
    CadDone = 0x0080,
    CadActivityDetected = 0x0100,
    RxTxTimeout = 0x0200,
    All = 0xFFFF,
}

/// Represents all possible opcodes understood by the radio
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RadioCommands {
    GetStatus = 0xC0,
    WriteRegister = 0x0D,
    ReadRegister = 0x1D,
    WriteBuffer = 0x0E,
    ReadBuffer = 0x1E,
    SetSleep = 0x84,
    SetStandby = 0x80,
    SetFs = 0xC1,
    SetTx = 0x83,
    SetRx = 0x82,
    SetRxDutyCycle = 0x94,
    SetCad = 0xC5,
    SetTxContinuousWave = 0xD1,
    SetTxContinuousPreamble = 0xD2,
    SetPacketType = 0x8A,
    GetPacketType = 0x11,
    SetRfFrequency = 0x86,
    SetTxParams = 0x8E,
    SetPaConfig = 0x95,
    SetCadParams = 0x88,
    SetBufferBaseAddress = 0x8F,
    SetModulationParams = 0x8B,
    SetPacketParams = 0x8C,
    GetRxBufferStatus = 0x13,
    GetPacketStatus = 0x14,
    GetRssiInst = 0x15,
    GetStats = 0x10,
    ResetStats = 0x00,
    CfgDioIrq = 0x08,
    GetIrqStatus = 0x12,
    ClrIrqStatus = 0x02,
    Calibrate = 0x89,
    CalibrateImage = 0x98,
    SetRegulatorMode = 0x96,
    GetError = 0x17,
    ClrError = 0x07,
    SetTcxoMode = 0x97,
    SetTxFallbackMode = 0x93,
    SetRfSwitchMode = 0x9D,
    SetStopRxTimerOnPreamble = 0x9F,
    SetLoRaSymbTimeout = 0xA0,
}

/// GFSK modulation parameters
#[derive(Debug, Clone, Copy)]
pub struct GfskModulationParams {
    pub bit_rate: usize,
    pub fdev: usize,
    pub modulation_shaping: RadioModShapings,
    pub bandwidth: u8,
}

/// LoRa modulation parameters
#[derive(Debug, Clone, Copy)]
pub struct LoRaModulationParams {
    /// Spreading Factor for the LoRa modulation
    pub spreading_factor: RadioLoRaSpreadingFactors,
    /// Bandwidth for the LoRa modulation
    pub bandwidth: RadioLoRaBandwidths,
    /// Coding rate for the LoRa modulation
    pub coding_rate: RadioLoRaCodingRates,
    /// Indicates if the modem uses the low datarate optimization
    pub low_datarate_optimize: u8,
}

/// The type describing the modulation parameters for every packet type
#[derive(Debug, Clone, Copy)]
pub enum ModulationParams {
    Gfsk(GfskModulationParams),
    LoRa(LoRaModulationParams),
}

/// GFSK packet parameters
#[derive(Debug, Clone, Copy)]
pub struct GfskPacketParams {
    /// The preamble Tx length for GFSK packet type in bits
    pub preamble_length: u16,
    /// The preamble Rx length minimal for GFSK packet type
    pub preamble_min_detect: RadioPreambleDetection,
    /// The synchronization word length for GFSK packet type
    pub sync_word_length: u8,
    /// Activated SyncWord correlators
    pub addr_comp: RadioAddressComp,
    /// If the header is explicit, it will be transmitted in the GFSK packet.
    /// If the header is implicit, it will not be transmitted.
    pub header_type: RadioPacketLengthModes,
    /// Size of the payload in the GFSK packet
    pub payload_length: u8,
    /// Size of the CRC block in the GFSK packet
    pub crc_length: RadioCrcTypes,
    pub dc_free: RadioDcFree,
}

/// LoRa packet parameters
#[derive(Debug, Clone, Copy)]
pub struct LoRaPacketParams {
    /// The preamble length is the number of LoRa symbols in the preamble
    pub preamble_length: u16,
    /// If the header is explicit, it will be transmitted in the LoRa packet.
    /// If the header is implicit, it will not be transmitted.
    pub header_type: RadioLoRaPacketLengthsMode,
    /// Size of the payload in the LoRa packet
    pub payload_length: u8,
    /// Size of CRC block in LoRa packet
    pub crc_mode: RadioLoRaCrcModes,
    /// Allows to swap IQ for LoRa packet
    pub invert_iq: RadioLoRaIQModes,
}

/// The type describing the packet parameters for every packet type
#[derive(Debug, Clone, Copy)]
pub enum PacketParams {
    Gfsk(GfskPacketParams),
    LoRa(LoRaPacketParams),
}

/// GFSK packet status
#[derive(Debug, Clone, Copy)]
pub struct GfskPacketStatus {
    pub rx_status: u8,
    /// The averaged RSSI
    pub rssi_avg: i8,
    /// The RSSI measured on last packet
    pub rssi_sync: i8,
    pub freq_error: usize,
}

/// LoRa packet status
#[derive(Debug, Clone, Copy)]
pub struct LoRaPacketStatus {
    /// The RSSI of the last packet
    pub rssi_pkt: i8,
    /// The SNR of the last packet
    pub snr_pkt: i8,
    pub signal_rssi_pkt: i8,
    pub freq_error: usize,
}

/// Represents the packet status for every packet type
#[derive(Debug, Clone, Copy)]
pub enum PacketStatus {
    Gfsk(GfskPacketStatus),
    LoRa(LoRaPacketStatus),
}

/// Represents the Rx internal counters values when GFSK or LoRa packet type is used
#[derive(Debug, Clone, Copy)]
pub struct RxCounter {
    pub packet_type: RadioPacketTypes,
    pub packet_received: u16,
    pub crc_ok: u16,
    pub length_error: u16,
}

/// Represents a calibration configuration
#[derive(Default, Debug, Clone, Copy)]
pub struct CalibrationParams {
    pub value: u8,
}

impl CalibrationParams {
    /// Calibrate RC64K clock
    pub fn rc64k_enable(&self) -> bool {
        self.value & (1 << 0) != 0
    }

    /// Calibrate RC13M clock
    pub fn rc13m_enable(&self) -> bool {
        self.value & (1 << 1) != 0
    }

    /// Calibrate PLL
    pub fn pll_enable(&self) -> bool {
        self.value & (1 << 2) != 0
    }

    /// Calibrate ADC Pulse
    pub fn adc_pulse_enable(&self) -> bool {
        self.value & (1 << 3) != 0
    }

    /// Calibrate ADC bulkN
    pub fn adc_bulk_n_enable(&self) -> bool {
        self.value & (1 << 4) != 0
    }

    /// Calibrate ADC bulkP
    pub fn adc_bulk_p_enable(&self) -> bool {
        self.value & (1 << 5) != 0
    }

    pub fn img_enable(&self) -> bool {
        self.value & (1 << 6) != 0
    }

    pub fn set_rc64k_enable(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 0;
        } else {
            self.value &= !(1 << 0);
        }
        self
    }

    pub fn set_rc13m_enable(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 1;
        } else {
            self.value &= !(1 << 1);
        }
        self
    }

    pub fn set_pll_enable(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 2;
        } else {
            self.value &= !(1 << 2);
        }
        self
    }

    pub fn set_adc_pulse_enable(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 3;
        } else {
            self.value &= !(1 << 3);
        }
        self
    }

    pub fn set_adc_bulk_n_enable(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 4;
        } else {
            self.value &= !(1 << 4);
        }
        self
    }

    pub fn set_adc_bulk_p_enable(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 5;
        } else {
            self.value &= !(1 << 5);
        }
        self
    }

    pub fn set_img_enable(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 6;
        } else {
            self.value &= !(1 << 6);
        }
        self
    }
}

/// Represents a sleep mode configuration
#[derive(Default, Debug, Clone, Copy)]
pub struct SleepParams {
    pub value: u8,
}

impl SleepParams {
    /// Get out of sleep mode if wakeup signal received from RTC
    pub fn wake_up_rtc(&self) -> bool {
        self.value & (1 << 0) != 0
    }

    pub fn reset(&self) -> bool {
        self.value & (1 << 1) != 0
    }

    pub fn warm_start(&self) -> bool {
        self.value & (1 << 2) != 0
    }

    pub fn set_wake_up_rtc(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 0;
        } else {
            self.value &= !(1 << 0);
        }
        self
    }

    pub fn set_reset(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 1;
        } else {
            self.value &= !(1 << 1);
        }
        self
    }

    pub fn set_warm_start(mut self, enable: bool) -> Self {
        if enable {
            self.value |= 1 << 2;
        } else {
            self.value &= !(1 << 2);
        }
        self
    }
}

/// Represents the possible radio system error states
#[derive(Debug, Clone, Copy)]
pub struct RadioError {
    pub value: u16,
}

impl RadioError {
    /// RC 64kHz oscillator calibration failed
    pub fn rc64k_calib(&self) -> bool {
        self.value & (1 << 0) != 0
    }

    /// RC 13MHz oscillator calibration failed
    pub fn rc13m_calib(&self) -> bool {
        self.value & (1 << 1) != 0
    }

    /// PLL calibration failed
    pub fn pll_calib(&self) -> bool {
        self.value & (1 << 2) != 0
    }

    /// ADC calibration failed
    pub fn adc_calib(&self) -> bool {
        self.value & (1 << 3) != 0
    }

    /// Image calibration failed
    pub fn img_calib(&self) -> bool {
        self.value & (1 << 4) != 0
    }

    /// XOSC oscillator failed to start
    pub fn xosc_start(&self) -> bool {
        self.value & (1 << 5) != 0
    }

    /// PLL lock failed
    pub fn pll_lock(&self) -> bool {
        self.value & (1 << 6) != 0
    }

    /// Buck converter failed to start
    pub fn buck_start(&self) -> bool {
        self.value & (1 << 7) != 0
    }

    /// PA ramp failed
    pub fn pa_ramp(&self) -> bool {
        self.value & (1 << 8) != 0
    }
}

pub struct Sx126x {
    pub packet_params: PacketParams,
    pub packet_status: PacketStatus,
    pub modulation_params: ModulationParams,
}

type VoidFn = fn();
// type DioIrqHandler = VoidFn;

pub const XTAL_FREQ: f64 = 32000000.0;
pub const FREQ_DIV: f64 = 2u64.pow(25) as f64;
pub const FREQ_STEP: f64 = XTAL_FREQ / FREQ_DIV;
pub const RX_BUFFER_SIZE: usize = 256;

pub struct Sx126xCallbacks {
    pub tx_done: Option<VoidFn>,
    pub rx_done: Option<VoidFn>,
    pub rx_preamble_detect: Option<VoidFn>,
    pub rx_sync_word_done: Option<VoidFn>,
    pub rx_header_done: Option<fn(is_ok: bool)>,
    pub tx_timeout: Option<VoidFn>,
    pub rx_timeout: Option<VoidFn>,
    pub rx_error: Option<IrqErrorCode>,
    pub cad_done: Option<fn(cad_flag: bool)>,
}

pub struct RadioRegister {
    pub addr: u16,
    pub value: u8,
}

pub static mut OPERATING_MODE: RadioOperatingModes = RadioOperatingModes::Sleep;
pub static mut RADIO_PACKET_TYPE: RadioPacketTypes = RadioPacketTypes::None;
pub static mut FREQUENCY_ERROR: usize = 0;
pub static mut IMAGE_CALIBRATED: bool = false;

pub fn sx126x_init() {
    sx126x_lorac_init();

    sx126x_reset();

    sx126x_wakeup();

    sx126x_set_standby(RadioStandbyModes::StdbyRc);

    // #ifdef CONFIG_LORA_USE_TCXO
    //     CalibrationParams_t calibParam;

    //     SX126xSetDio3AsTcxoCtrl( TCXO_CTRL_1_7V, SX126xGetBoardTcxoWakeupTime( ) << 6 ); // convert from ms to SX126x time base
    //     calibParam.Value = 0x7F;
    //     SX126xCalibrate( calibParam );
    // #endif

    sx126x_set_dio_2_as_rf_switch_ctrl(true);
    unsafe { OPERATING_MODE = RadioOperatingModes::StdbyRc };
}

pub fn sx126x_get_operating_mode() -> RadioOperatingModes {
    unsafe { OPERATING_MODE }
}

pub fn sx126x_set_operating_mode(mode: RadioOperatingModes) {
    unsafe { OPERATING_MODE = mode };
}

pub fn sx126x_check_device_ready() {
    match sx126x_get_operating_mode() {
        RadioOperatingModes::Sleep | RadioOperatingModes::RxDc => {
            sx126x_wakeup();
            sx126x_ant_sw_on();
        }
        RadioOperatingModes::Rx => {
            sx126x_ant_sw_on();
        }
        _ => {
            sx126x_ant_sw_off();
        }
    }

    sx126x_wait_on_busy();
}

pub fn sx126x_set_payload(payload: &[u8]) {
    sx126x_write_buffer(0x00, payload);
}

pub enum Sx126xError {
    PayloadBufferTooSmall,
}

pub fn sx126x_get_payload(buffer: &mut [u8], max_size: u8) -> Result<u8, Sx126xError> {
    let mut offset = 0;
    let length = get_rx_buffer_status(buffer.len() as u8, &mut offset);
    if length > max_size {
        return Err(Sx126xError::PayloadBufferTooSmall);
    }
    sx126x_read_buffer(offset, &mut buffer[..length as usize]);

    Ok(length)
}

pub fn sx126x_send_payload(payload: &[u8], timeout: usize) {
    sx126x_set_payload(payload);
    sx126x_set_tx(timeout);
}

pub fn sx126x_set_sync_word(sync_word: &[u8]) {
    sx126x_write_registers(REG_LR_SYNCWORDBASEADDRESS, sync_word);
}

pub fn sx126x_set_crc_seed(seed: u16) {
    let seed_bytes = seed.to_be_bytes();
    if matches!(sx126x_get_packet_type(), RadioPacketTypes::Gfsk) {
        sx126x_write_registers(REG_LR_CRCSEEDBASEADDR, &seed_bytes);
    }
}

pub fn sx126x_set_crc_polynomial(polynomial: u16) {
    let polynomial_bytes = polynomial.to_be_bytes();
    if matches!(sx126x_get_packet_type(), RadioPacketTypes::Gfsk) {
        sx126x_write_registers(REG_LR_CRCPOLYBASEADDR, &polynomial_bytes);
    }
}

pub fn sx126x_set_whitening_seed(seed: u16) {
    if matches!(sx126x_get_packet_type(), RadioPacketTypes::Gfsk) {
        let mut reg_value = sx126x_read_register(REG_LR_WHITSEEDBASEADDR_MSB) & 0xFE;
        reg_value |= (seed >> 8) as u8 & 0x01;
        sx126x_write_registers(REG_LR_WHITSEEDBASEADDR_MSB, &[reg_value]); // ony 1 bit
        sx126x_write_registers(REG_LR_WHITSEEDBASEADDR_LSB, &[seed as u8]);
    }
}

pub fn sx126x_get_random() -> usize {
    let mut random_bytes = [0u8; 4];

    sx126x_set_rx(0);

    delay_ms(1);

    sx126x_read_registers(RANDOM_NUMBER_GENERATORBASEADDR, &mut random_bytes);

    sx126x_set_standby(RadioStandbyModes::StdbyRc);

    usize::from_be_bytes(random_bytes)
}

pub fn sx126x_set_sleep(sleep_config: SleepParams) {
    sx126x_ant_sw_off();
    sx126x_write_command(RadioCommands::SetSleep, &[sleep_config.value]);
    unsafe { OPERATING_MODE = RadioOperatingModes::Sleep };
}

pub fn sx126x_set_standby(standby_config: RadioStandbyModes) {
    sx126x_write_command(RadioCommands::SetStandby, &[standby_config as u8]);
    unsafe {
        if standby_config == RadioStandbyModes::StdbyRc {
            OPERATING_MODE = RadioOperatingModes::StdbyRc;
        } else {
            OPERATING_MODE = RadioOperatingModes::StdbyXosc;
        }
    }
}

pub fn sx126x_set_fs() {
    sx126x_write_command(RadioCommands::SetFs, &[]);
    unsafe { OPERATING_MODE = RadioOperatingModes::Fs };
}

pub fn sx126x_set_tx(timeout: usize) {
    unsafe { OPERATING_MODE = RadioOperatingModes::Tx };

    let buf = [
        ((timeout >> 16) & 0xFF) as u8,
        ((timeout >> 8) & 0xFF) as u8,
        (timeout & 0xFF) as u8,
    ];
    sx126x_write_command(RadioCommands::SetTx, &buf);
}

pub fn sx126x_set_rx(timeout: usize) {
    unsafe { OPERATING_MODE = RadioOperatingModes::Rx };

    let buf = [
        ((timeout >> 16) & 0xFF) as u8,
        ((timeout >> 8) & 0xFF) as u8,
        (timeout & 0xFF) as u8,
    ];
    sx126x_write_command(RadioCommands::SetRx, &buf);
}

pub fn sx126x_set_rx_boosted(timeout: usize) {
    unsafe { OPERATING_MODE = RadioOperatingModes::Rx };

    sx126x_write_registers(REG_RX_GAIN, &[0x96]); // max LNA gain, increase current by ~2mA for around ~3dB in sensitivity

    let buf = [
        ((timeout >> 16) & 0xFF) as u8,
        ((timeout >> 8) & 0xFF) as u8,
        (timeout & 0xFF) as u8,
    ];
    sx126x_write_command(RadioCommands::SetRx, &buf);
}

pub fn sx126x_set_rx_duty_cycle(rx_time: usize, sleep_time: usize) {
    let buf = [
        ((rx_time >> 16) & 0xFF) as u8,
        ((rx_time >> 8) & 0xFF) as u8,
        (rx_time & 0xFF) as u8,
        ((sleep_time >> 16) & 0xFF) as u8,
        ((sleep_time >> 8) & 0xFF) as u8,
        (sleep_time & 0xFF) as u8,
    ];
    sx126x_write_command(RadioCommands::SetRxDutyCycle, &buf);
    unsafe { OPERATING_MODE = RadioOperatingModes::RxDc };
}

pub fn sx126x_set_cad() {
    sx126x_write_command(RadioCommands::SetCad, &[]);
    unsafe { OPERATING_MODE = RadioOperatingModes::Cad };
}

pub fn sx126x_set_tx_continuous_wave() {
    sx126x_write_command(RadioCommands::SetTxContinuousWave, &[]);
}

pub fn sx126x_set_tx_infinite_preamble() {
    sx126x_write_command(RadioCommands::SetTxContinuousPreamble, &[]);
}

pub fn sx126x_set_stop_rx_timer_on_preamble_detect(enable: bool) {
    sx126x_write_command(RadioCommands::SetStopRxTimerOnPreamble, &[enable as u8]);
}

pub fn sx126x_set_lora_symb_num_timeout(symb_num: u8) {
    sx126x_write_command(RadioCommands::SetLoRaSymbTimeout, &[symb_num]);
}

pub fn sx126x_set_regulator_mode(mode: RadioRegulatorMode) {
    sx126x_write_command(RadioCommands::SetRegulatorMode, &[mode as u8]);
}

pub fn sx126x_calibrate(calib_param: CalibrationParams) {
    sx126x_write_command(RadioCommands::Calibrate, &[calib_param.value]);
}

pub fn sx126x_calibrate_image(freq: usize) {
    let cal_freq = if freq > 900_000_000 {
        [0xE1, 0xE9]
    } else if freq > 850_000_000 {
        [0xD7, 0xD8]
    } else if freq > 770_000_000 {
        [0xC1, 0xC5]
    } else if freq > 460_000_000 {
        [0x75, 0x81]
    // } else if freq > 425_000_000 {
    //     [0x6B, 0x6F]
    } else {
        [0x6B, 0x6F] // default fallback
    };
    sx126x_write_command(RadioCommands::CalibrateImage, &cal_freq);
}

pub fn sx126x_set_pa_config(pa_duty_cycle: u8, hp_max: u8, device_sel: u8, pa_lut: u8) {
    let buf = [pa_duty_cycle, hp_max, device_sel, pa_lut];
    sx126x_write_command(RadioCommands::SetPaConfig, &buf);
}

pub fn sx126x_set_rx_tx_fallback_mode(fallback_mode: u8) {
    sx126x_write_command(RadioCommands::SetTxFallbackMode, &[fallback_mode]);
}

pub fn sx126x_set_dio_irq_params(irq_mask: u16, dio1_mask: u16, dio2_mask: u16, dio3_mask: u16) {
    let buf = [
        ((irq_mask >> 8) & 0x00FF) as u8,
        (irq_mask & 0x00FF) as u8,
        ((dio1_mask >> 8) & 0x00FF) as u8,
        (dio1_mask & 0x00FF) as u8,
        ((dio2_mask >> 8) & 0x00FF) as u8,
        (dio2_mask & 0x00FF) as u8,
        ((dio3_mask >> 8) & 0x00FF) as u8,
        (dio3_mask & 0x00FF) as u8,
    ];
    sx126x_write_command(RadioCommands::CfgDioIrq, &buf);
}

pub fn sx126x_get_irq_status() -> u16 {
    let mut irq_status = [0u8; 2];
    sx126x_read_command(RadioCommands::GetIrqStatus, &mut irq_status);
    ((irq_status[0] as u16) << 8) | irq_status[1] as u16
}

pub fn sx126x_set_dio_2_as_rf_switch_ctrl(enable: bool) {
    sx126x_write_command(RadioCommands::SetRfSwitchMode, &[enable as u8]);
}

pub fn sx126x_set_dio_3_as_tcxo_ctrl(tcxo_voltage: RadioTcxoCtrlVoltage, timeout: usize) {
    let buf = [
        tcxo_voltage as u8 & 0x07,
        ((timeout >> 16) & 0xFF) as u8,
        ((timeout >> 8) & 0xFF) as u8,
        (timeout & 0xFF) as u8,
    ];
    sx126x_write_command(RadioCommands::SetTcxoMode, &buf);
}

pub fn sx126x_set_rf_frequency(frequency: usize) {
    unsafe {
        if !IMAGE_CALIBRATED {
            sx126x_calibrate_image(frequency);
            IMAGE_CALIBRATED = true;
        }
    }

    let freq = (frequency as f64 / FREQ_STEP) as usize;
    let buf = [
        ((freq >> 24) & 0xFF) as u8,
        ((freq >> 16) & 0xFF) as u8,
        ((freq >> 8) & 0xFF) as u8,
        (freq & 0xFF) as u8,
    ];
    sx126x_write_command(RadioCommands::SetRfFrequency, &buf);
}

pub fn sx126x_set_packet_type(packet_type: RadioPacketTypes) {
    unsafe { RADIO_PACKET_TYPE = packet_type };
    sx126x_write_command(RadioCommands::SetPacketType, &[packet_type as u8]);
}

pub fn sx126x_get_packet_type() -> RadioPacketTypes {
    unsafe { RADIO_PACKET_TYPE }
}

pub fn sx126x_set_tx_params(power: i8, ramp_time: RadioRampTimes) {
    // For SX1262:
    // WORKAROUND - Better Resistance of the SX1262 Tx to Antenna Mismatch
    let reg_val = sx126x_read_register(0x08D8) | (0x0F << 1);
    sx126x_write_registers(0x08D8, &[reg_val]);

    sx126x_set_pa_config(0x04, 0x07, 0x00, 0x01);

    let power = power.clamp(-3, 22);

    sx126x_write_registers(REG_OCP, &[0x38]); // current max 160mA for the whole device

    let buf = [power as u8, ramp_time as u8];
    sx126x_write_command(RadioCommands::SetTxParams, &buf);
}

pub fn sx126x_set_modulation_params(modulation_params: &ModulationParams) {
    match modulation_params {
        ModulationParams::Gfsk(params) => {
            if !matches!(sx126x_get_packet_type(), RadioPacketTypes::Gfsk) {
                sx126x_set_packet_type(RadioPacketTypes::Gfsk);
            }
            let temp_val = (32.0 * (XTAL_FREQ / params.bit_rate as f64)) as usize;
            let fdev_val = (params.fdev as f64 / FREQ_STEP) as usize;
            let buf = [
                ((temp_val >> 16) & 0xFF) as u8,
                ((temp_val >> 8) & 0xFF) as u8,
                (temp_val & 0xFF) as u8,
                params.modulation_shaping as u8,
                params.bandwidth,
                ((fdev_val >> 16) & 0xFF) as u8,
                ((fdev_val >> 8) & 0xFF) as u8,
                (fdev_val & 0xFF) as u8,
            ];
            sx126x_write_command(RadioCommands::SetModulationParams, &buf);
        }
        ModulationParams::LoRa(params) => {
            if !matches!(sx126x_get_packet_type(), RadioPacketTypes::LoRa) {
                sx126x_set_packet_type(RadioPacketTypes::LoRa);
            }
            let buf = [
                params.spreading_factor as u8,
                params.bandwidth as u8,
                params.coding_rate as u8,
                params.low_datarate_optimize,
            ];
            sx126x_write_command(RadioCommands::SetModulationParams, &buf);
        }
    }
}

pub fn sx126x_set_packet_params(packet_params: &PacketParams) {
    match packet_params {
        PacketParams::Gfsk(params) => {
            if !matches!(sx126x_get_packet_type(), RadioPacketTypes::Gfsk) {
                sx126x_set_packet_type(RadioPacketTypes::Gfsk);
            }

            let crc_val = match params.crc_length {
                RadioCrcTypes::TwoBytesIbm => {
                    sx126x_set_crc_seed(CRC_IBM_SEED);
                    sx126x_set_crc_polynomial(CRC_POLYNOMIAL_IBM);
                    RadioCrcTypes::TwoBytes as u8
                }
                RadioCrcTypes::TwoBytesCcit => {
                    sx126x_set_crc_seed(CRC_CCITT_SEED);
                    sx126x_set_crc_polynomial(CRC_POLYNOMIAL_CCITT);
                    RadioCrcTypes::TwoBytesInv as u8
                }
                other => other as u8,
            };

            let buf = [
                ((params.preamble_length >> 8) & 0xFF) as u8,
                (params.preamble_length & 0xFF) as u8,
                params.preamble_min_detect as u8,
                params.sync_word_length,
                params.addr_comp as u8,
                params.header_type as u8,
                params.payload_length,
                crc_val,
                params.dc_free as u8,
            ];
            sx126x_write_command(RadioCommands::SetPacketParams, &buf);
        }
        PacketParams::LoRa(params) => {
            if !matches!(sx126x_get_packet_type(), RadioPacketTypes::LoRa) {
                sx126x_set_packet_type(RadioPacketTypes::LoRa);
            }

            let buf = [
                ((params.preamble_length >> 8) & 0xFF) as u8,
                (params.preamble_length & 0xFF) as u8,
                params.header_type as u8,
                params.payload_length,
                params.crc_mode as u8,
                params.invert_iq as u8,
            ];
            sx126x_write_command(RadioCommands::SetPacketParams, &buf);
        }
    }
}

pub fn sx126x_set_cad_params(
    cad_symbol_num: RadioLoRaCadSymbols,
    cad_det_peak: u8,
    cad_det_min: u8,
    cad_exit_mode: RadioCadExitModes,
    cad_timeout: usize,
) {
    let buf = [
        cad_symbol_num as u8,
        cad_det_peak,
        cad_det_min,
        cad_exit_mode as u8,
        ((cad_timeout >> 16) & 0xFF) as u8,
        ((cad_timeout >> 8) & 0xFF) as u8,
        (cad_timeout & 0xFF) as u8,
    ];
    sx126x_write_command(RadioCommands::SetCadParams, &buf[..5]);
    unsafe { OPERATING_MODE = RadioOperatingModes::Cad };
}

pub fn sx126x_set_buffer_base_address(tx_base_address: u8, rx_base_address: u8) {
    let buf = [tx_base_address, rx_base_address];
    sx126x_write_command(RadioCommands::SetBufferBaseAddress, &buf);
}

pub fn sx126x_get_status() -> RadioStatus {
    let mut stat = [0u8; 1];
    sx126x_read_command(RadioCommands::GetStatus, &mut stat);
    RadioStatus { value: stat[0] }
}

pub fn sx126x_get_rssi_inst() -> i8 {
    let mut buf = [0u8; 1];
    sx126x_read_command(RadioCommands::GetRssiInst, &mut buf);
    -(buf[0] as i8 >> 1)
}

// change?
pub fn get_rx_buffer_status(_payload_length: u8, rx_start_buffer_pointer: &mut u8) -> u8 {
    let mut status = [0u8; 2];
    sx126x_read_command(RadioCommands::GetRxBufferStatus, &mut status);

    // In case of LoRa fixed header, the payload_length is obtained by reading
    // the register REG_LR_PAYLOADLENGTH
    let length = if matches!(sx126x_get_packet_type(), RadioPacketTypes::LoRa)
        && (sx126x_read_register(REG_LR_PACKETPARAMS) >> 7 == 1)
    {
        sx126x_read_register(REG_LR_PAYLOADLENGTH)
    } else {
        status[0]
    };

    *rx_start_buffer_pointer = status[1];
    length
}

pub fn sx126x_get_packet_status() -> PacketStatus {
    let mut status = [0u8; 3];
    sx126x_read_command(RadioCommands::GetPacketStatus, &mut status);

    match sx126x_get_packet_type() {
        RadioPacketTypes::Gfsk => PacketStatus::Gfsk(GfskPacketStatus {
            rx_status: status[0],
            rssi_sync: (-(status[1] as i32 >> 1)) as i8,
            rssi_avg: (-(status[2] as i32 >> 1)) as i8,
            freq_error: 0,
        }),
        RadioPacketTypes::LoRa => {
            let snr_pkt = if status[1] < 128 {
                (status[1] >> 2) as i8
            } else {
                ((status[1] as i16 - 256) >> 2) as i8
            };
            PacketStatus::LoRa(LoRaPacketStatus {
                rssi_pkt: (-(status[0] as i32 >> 1)) as i8,
                snr_pkt,
                signal_rssi_pkt: (-(status[2] as i32 >> 1)) as i8,
                freq_error: unsafe { FREQUENCY_ERROR },
            })
        }
        RadioPacketTypes::None => PacketStatus::LoRa(LoRaPacketStatus {
            rssi_pkt: 0,
            snr_pkt: 0,
            signal_rssi_pkt: 0,
            freq_error: 0,
        }),
    }
}

pub fn sx126x_get_device_errors() -> RadioError {
    let mut buf = [0u8; 2];
    sx126x_read_command(RadioCommands::GetError, &mut buf);
    RadioError {
        value: ((buf[0] as u16) << 8) | buf[1] as u16,
    }
}

pub fn sx126x_clear_device_errors() {
    let buf = [0x00u8, 0x00];
    sx126x_write_command(RadioCommands::ClrError, &buf);
}

pub fn sx126x_clear_irq_status(irq: u16) {
    let buf = [((irq >> 8) & 0x00FF) as u8, (irq & 0x00FF) as u8];
    sx126x_write_command(RadioCommands::ClrIrqStatus, &buf);
}
