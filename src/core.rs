use crate::constants::*;
use core::marker::PhantomData;
use embedded_hal::delay::DelayNs;
use embedded_hal::spi::*;

pub struct PinnacleTouchpad<S, D> {
    spi: S,
    phantom_: PhantomData<D>,
}

mod private {
    pub trait Sealed {}
}

pub struct Config {
    pub x: bool,
    pub y: bool,
    pub filter: bool,
    pub swap_x_y: bool,
    pub glide_extend: bool,
    pub scroll: bool,
    pub secondary_tap: bool,
    pub all_taps: bool,
    pub intellimouse: bool,
    pub calibrate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            x: true,
            y: true,
            filter: true,
            swap_x_y: true,
            glide_extend: false,
            scroll: false,
            secondary_tap: false,
            all_taps: false,
            intellimouse: false,
            calibrate: false,
        }
    }
}

pub trait Build: private::Sealed {
    type Data: private::Sealed;
    fn build(&self, feed_config1: &mut u8);
}

impl private::Sealed for Absolute {}
impl Build for Absolute {
    type Data = AbsoluteData;
    fn build(&self, feed_config1: &mut u8) {
        *feed_config1 |= (self.invert_y as u8) << 7 | (self.invert_x as u8) << 6 | 1 << 1;
    }
}

impl private::Sealed for Relative {}
impl Build for Relative {
    type Data = RelativeData;
    fn build(&self, feed_config1: &mut u8) {
        *feed_config1 &= !(1 << 1);
    }
}

pub fn new<S, D, Delay>(
    spi: S,
    config: Config,
    delay: &mut Delay,
    data: D,
) -> Result<PinnacleTouchpad<S, <D as Build>::Data>, S::Error>
where
    S: SpiDevice<u8>,
    D: Build,
    <D as Build>::Data: TouchpadData,
    Delay: DelayNs,
{
    let mut pinnacle = PinnacleTouchpad::new(spi);
    pinnacle.write(STATUS1_ADDR, 0x00)?; // SW_CC
    delay.delay_us(50);
    let feed_config2 = (config.swap_x_y as u8) << 7
        | (!config.glide_extend as u8) << 4
        | (!config.scroll as u8) << 4
        | (!config.secondary_tap as u8) << 2
        | (!config.all_taps as u8) << 1
        | (config.intellimouse as u8);
    pinnacle.write(SYS_CONFIG1_ADDR, 0x00)?;
    // pinnacle.write(FEED_CONFIG2_ADDR, 0x1F)?;
    // pinnacle.write(FEED_CONFIG1_ADDR, 0x03)?;
    pinnacle.write(FEED_CONFIG2_ADDR, feed_config2)?;
    if config.calibrate {
        let calibrate_config = 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1;
        pinnacle.write(CAL_CONFIG1_ADDR, calibrate_config)?;
    }

    let mut feed_config1 =
        1 | (!config.y as u8) << 4 | (!config.x as u8) << 3 | (!config.filter as u8) << 2;
    data.build(&mut feed_config1);
    pinnacle.write(FEED_CONFIG1_ADDR, feed_config1)?;
    Ok(pinnacle)
}

impl<S, D> PinnacleTouchpad<S, D>
where
    D: TouchpadData,
{
    fn new(spi: S) -> Self {
        Self {
            spi,
            phantom_: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum SampleRate {
    OneHundred,
    Eighty,
    Sixty,
    Forty,
    Twenty,
    Ten,
}

impl<S, D> PinnacleTouchpad<S, D>
where
    S: SpiDevice<u8>,
    D: TouchpadData,
{
    pub fn clear_flags(&mut self) -> Result<(), S::Error> {
        self.write(STATUS1_ADDR, 0x00)
    }

    pub fn product_id(&mut self) -> Result<u8, S::Error> {
        self.read(PRODUCT_ID_ADDR)
    }

    pub fn firmware_id(&mut self) -> Result<u8, S::Error> {
        self.read(FIRMWARE_ID_ADDR)
    }

    pub fn firmware_version(&mut self) -> Result<u8, S::Error> {
        self.read(FIRMWARE_VERSION_ADDR)
    }

    pub fn status(&mut self) -> Result<u8, S::Error> {
        self.read(STATUS1_ADDR)
    }

    /*
    100 Sample/Second 64H
    80  Sample/Second 50H
    60  Sample/Second 3CH
    40  Sample/Second 28H
    20  Sample/Second 14H
    10  Sample/Second 0Ah
     */

    pub fn sample_rate(&mut self) -> Result<SampleRate, S::Error> {
        todo!()
    }

    pub fn set_sample_rate(&mut self, _sample_rate: SampleRate) -> Result<(), S::Error> {
        todo!()
    }

    pub fn z_idle(&mut self) -> Result<u8, S::Error> {
        self.read(Z_IDLE_ADDR)
    }

    pub fn set_z_idle(&mut self, z_idle: u8) -> Result<(), S::Error> {
        self.write(Z_IDLE_ADDR, z_idle)
    }

    pub fn z_scaler(&mut self) -> Result<u8, S::Error> {
        self.read(Z_SCALER_ADDR)
    }

    pub fn set_z_scaler(&mut self, z_scaler: u8) -> Result<(), S::Error> {
        self.write(Z_SCALER_ADDR, z_scaler)
    }

    // Read a byte from `addr`.
    fn read(&mut self, addr: u8) -> Result<u8, S::Error> {
        let addr = READ_BITS | (addr & ADDR_MASK);
        let mut buf = [addr, READ_FILL, READ_FILL, READ_FILL];
        self.spi.transfer_in_place(&mut buf).unwrap();
        Ok(buf[3])
    }

    fn read_multi<const N: usize>(&mut self, addr: u8) -> Result<[u8; N], S::Error> {
        let addr = READ_BITS | (addr & ADDR_MASK);
        let mut buf = [READ_CONTINUE; N];
        buf[N - 1] = READ_FILL;
        let mut addr_buf = [addr, READ_CONTINUE, READ_CONTINUE];
        self.spi
            .transaction(&mut [
                Operation::TransferInPlace(&mut addr_buf),
                Operation::TransferInPlace(&mut buf),
            ])
            .unwrap();
        Ok(buf)
    }

    fn write(&mut self, addr: u8, data: u8) -> Result<(), S::Error> {
        let addr = WRITE_BITS | (addr & ADDR_MASK);
        let mut buf = [addr, data];
        self.spi.transfer_in_place(&mut buf).unwrap();
        Ok(())
    }
}
pub struct Relative;
pub struct Absolute {
    invert_x: bool,
    invert_y: bool,
}

impl private::Sealed for RelativeData {}
impl private::Sealed for AbsoluteData {}

pub trait TouchpadData: private::Sealed {}
impl TouchpadData for AbsoluteData {}
impl TouchpadData for RelativeData {}

#[derive(Copy, Clone)]
pub struct AbsoluteData {
    pub x: u16,
    pub y: u16,
    pub z: u8,
    pub button_flags: u8,
}

impl<S> PinnacleTouchpad<S, AbsoluteData>
where
    S: SpiDevice<u8>,
{
    pub fn read_absolute(&mut self) -> Result<AbsoluteData, S::Error> {
        let data = self.read_multi::<6>(PACKET_BYTE_0_ADDR)?;
        Ok(AbsoluteData {
            x: data[2] as u16 | (((data[4] & 0x0F) as u16) << 8),
            y: data[3] as u16 | (((data[4] & 0xF0) as u16) << 4),
            z: data[5] & 0x3F,
            button_flags: data[0] & 0x3F,
        })
    }
}

#[derive(Copy, Clone)]
pub struct RelativeData {
    pub x: i16,
    pub y: i16,
    pub button_flags: u8,
    pub wheel: i8,
}

impl<S> PinnacleTouchpad<S, RelativeData>
where
    S: SpiDevice<u8>,
{
    pub fn read_relative(&mut self) -> Result<RelativeData, S::Error> {
        let data = self.read_multi::<4>(PACKET_BYTE_0_ADDR)?;
        let mut x = data[1] as i16;
        let mut y = data[2] as i16;
        if (data[0] & 0x10) > 0 {
            x -= 256;
        }
        if (data[0] & 0x20) > 0 {
            y -= 256;
        }
        Ok(RelativeData {
            x,
            y,
            button_flags: data[0] & 0x07,
            wheel: data[3] as i8,
        })
    }
}
