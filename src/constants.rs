pub const WRITE_BITS: u8 = 0b_1000_0000;
pub const READ_BITS: u8 = 0b_1010_0000;
pub const ADDR_MASK: u8 = 0b_0001_1111;
pub const READ_FILL: u8 = 0xFB;
pub const READ_CONTINUE: u8 = 0xFC;

/// Firmware ASIC ID.
pub const FIRMWARE_ID_ADDR: u8 = 0x00;

/// Firmware revision number.
pub const FIRMWARE_VERSION_ADDR: u8 = 0x01;

/// Contains status flags about the state of Pinnacle.
pub const STATUS1_ADDR: u8 = 0x02;

/// Contains system operation and configuration bits.
pub const SYS_CONFIG1_ADDR: u8 = 0x03;

/// Contains feed operation and configuration bits.
pub const FEED_CONFIG1_ADDR: u8 = 0x04;

/// Contains feed operation and configuration bits.
pub const FEED_CONFIG2_ADDR: u8 = 0x05;

/// Contains calibration configuration bits.
pub const CAL_CONFIG1_ADDR: u8 = 0x07;

// Contains Data register for PS/2 Aux Control.
// pub const PS2_AUX_CTRL_ADDR: u8 = 0x08;

/// Number of samples generated per second.
pub const SAMPLE_RATE_ADDR: u8 = 0x09;

/// Number of Z=0 packets sent when Z goes from >0 to 0.
pub const Z_IDLE_ADDR: u8 = 0x0A;

/// Contains the pen `Z_On` threshold.
pub const Z_SCALER_ADDR: u8 = 0x0B;

// Sleep Interval
pub const SLEEP_INTERVAL_ADDR: u8 = 0x0C;

// Sleep Timer
pub const SLEEP_TIMER_ADDR: u8 = 0x0D;

pub const PACKET_BYTE_0_ADDR: u8 = 0x12;

pub const PRODUCT_ID_ADDR: u8 = 0x1F;
