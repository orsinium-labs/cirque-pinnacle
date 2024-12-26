pub(crate) const WRITE_BITS: u8 = 0b_1000_0000;
pub(crate) const READ_BITS: u8 = 0b_1010_0000;
pub(crate) const ADDR_MASK: u8 = 0b_0001_1111;
pub(crate) const READ_FILL: u8 = 0xFB;
pub(crate) const READ_CONTINUE: u8 = 0xFC;

pub(crate) const FIRMWARE_ID_ADDR: u8 = 0x00;
pub(crate) const FIRMWARE_VERSION_ADDR: u8 = 0x01;
pub(crate) const STATUS1_ADDR: u8 = 0x02;
pub(crate) const SYS_CONFIG1_ADDR: u8 = 0x03;
pub(crate) const FEED_CONFIG1_ADDR: u8 = 0x04;
pub(crate) const FEED_CONFIG2_ADDR: u8 = 0x05;
pub(crate) const CAL_CONFIG1_ADDR: u8 = 0x07;
// pub(crate) const PS2_AUX_CTRL_ADDR: u8 = 0x08;
// pub(crate) const SAMPLE_RATE_ADDR: u8 = 0x09;
pub(crate) const Z_IDLE_ADDR: u8 = 0x0A;
pub(crate) const Z_SCALER_ADDR: u8 = 0x0B;
// pub(crate) const SLEEP_INTERVAL_ADDR: u8 = 0x0C;
// pub(crate) const SLEEP_TIMER_ADDR: u8 = 0x0D;
pub(crate) const PACKET_BYTE_0_ADDR: u8 = 0x12;
// pub(crate) const PACKET_BYTE_1_ADDR: u8 = 0x13;
// pub(crate) const PACKET_BYTE_2_ADDR: u8 = 0x14;
// pub(crate) const PACKET_BYTE_3_ADDR: u8 = 0x15;
// pub(crate) const PACKET_BYTE_4_ADDR: u8 = 0x16;
// pub(crate) const PACKET_BYTE_5_ADDR: u8 = 0x17;
pub(crate) const PRODUCT_ID_ADDR: u8 = 0x1F;
