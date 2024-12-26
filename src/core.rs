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

pub struct PinnacleTouchpadBuilder<S, D> {
    spi: S,
    x: bool,
    y: bool,
    filter: bool,
    swap_x_y: bool,
    glide_extend: bool,
    scroll: bool,
    secondary_tap: bool,
    all_taps: bool,
    intellimouse: bool,
    calibrate: bool,
    data: D,
}

impl<S: SpiDevice<u8>> PinnacleTouchpadBuilder<S, ()> {
    pub fn new(spi: S) -> Self {
        Self {
            spi,
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
            data: (),
        }
    }

    pub fn relative_mode(self) -> PinnacleTouchpadBuilder<S, Relative> {
        PinnacleTouchpadBuilder {
            spi: self.spi,
            x: self.x,
            y: self.y,
            filter: self.filter,
            swap_x_y: self.swap_x_y,
            glide_extend: self.glide_extend,
            scroll: self.scroll,
            secondary_tap: self.secondary_tap,
            all_taps: self.all_taps,
            intellimouse: self.intellimouse,
            calibrate: self.calibrate,
            data: Relative,
        }
    }

    pub fn absolute_mode(self) -> PinnacleTouchpadBuilder<S, Absolute> {
        PinnacleTouchpadBuilder {
            spi: self.spi,
            x: self.x,
            y: self.y,
            filter: self.filter,
            swap_x_y: self.swap_x_y,
            glide_extend: self.glide_extend,
            scroll: self.scroll,
            secondary_tap: self.secondary_tap,
            all_taps: self.all_taps,
            intellimouse: self.intellimouse,
            calibrate: self.calibrate,
            data: Absolute {
                invert_x: false,
                invert_y: false,
            },
        }
    }
}

impl<S, D> PinnacleTouchpadBuilder<S, D> {
    pub fn enable_x(mut self) -> Self {
        self.x = true;
        self
    }
    pub fn disable_x(mut self) -> Self {
        self.x = false;
        self
    }
    pub fn enable_y(mut self) -> Self {
        self.y = true;
        self
    }
    pub fn disable_y(mut self) -> Self {
        self.y = false;
        self
    }
    pub fn enable_filter(mut self) -> Self {
        self.filter = true;
        self
    }
    pub fn disable_filter(mut self) -> Self {
        self.filter = false;
        self
    }
    pub fn swap_x_y(mut self, swap: bool) -> Self {
        self.swap_x_y = swap;
        self
    }
    pub fn enable_glide_extend(mut self) -> Self {
        self.glide_extend = true;
        self
    }
    pub fn disable_glide_extend(mut self) -> Self {
        self.glide_extend = false;
        self
    }
    pub fn enable_scroll(mut self) -> Self {
        self.scroll = true;
        self
    }
    pub fn disable_scroll(mut self) -> Self {
        self.scroll = false;
        self
    }
    pub fn enable_secondary_tap(mut self) -> Self {
        self.secondary_tap = true;
        self
    }
    pub fn disable_secondary_tap(mut self) -> Self {
        self.secondary_tap = false;
        self
    }
    pub fn enable_all_taps(mut self) -> Self {
        self.all_taps = true;
        self
    }
    pub fn disable_all_taps(mut self) -> Self {
        self.all_taps = false;
        self
    }
    pub fn enable_intellimouse(mut self) -> Self {
        self.intellimouse = true;
        self
    }
    pub fn disable_intellimouse(mut self) -> Self {
        self.intellimouse = false;
        self
    }
    pub fn calibrate(mut self) -> Self {
        self.calibrate = true;
        self
    }
}

impl<S> PinnacleTouchpadBuilder<S, Relative> {}

impl<S> PinnacleTouchpadBuilder<S, Absolute> {
    pub fn invert_x(mut self, invert: bool) -> Self {
        self.data.invert_x = invert;
        self
    }
}

impl<S> PinnacleTouchpadBuilder<S, Absolute> {
    pub fn invert_y(mut self, invert: bool) -> Self {
        self.data.invert_y = invert;
        self
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

impl<S, D> PinnacleTouchpadBuilder<S, D>
where
    S: SpiDevice<u8>,
    D: Build,
    <D as Build>::Data: TouchpadData,
{
    pub fn build(
        self,
        delay: &mut Delay,
    ) -> Result<PinnacleTouchpad<S, <D as Build>::Data>, S::Error> {
        let mut pinnacle = PinnacleTouchpad::new(self.spi);
        pinnacle.write(STATUS1_ADDR, 0x00)?; // SW_CC
        delay.delay(50.micros());
        let feed_config2 = (self.swap_x_y as u8) << 7
            | (!self.glide_extend as u8) << 4
            | (!self.scroll as u8) << 4
            | (!self.secondary_tap as u8) << 2
            | (!self.all_taps as u8) << 1
            | (self.intellimouse as u8);
        pinnacle.write(SYS_CONFIG1_ADDR, 0x00)?;
        // pinnacle.write(FEED_CONFIG2_ADDR, 0x1F)?;
        // pinnacle.write(FEED_CONFIG1_ADDR, 0x03)?;
        pinnacle.write(FEED_CONFIG2_ADDR, feed_config2)?;
        if self.calibrate {
            let calibrate_config = 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1;
            pinnacle.write(CAL_CONFIG1_ADDR, calibrate_config)?;
        }

        let mut feed_config1 =
            1 | (!self.y as u8) << 4 | (!self.x as u8) << 3 | (!self.filter as u8) << 2;
        self.data.build(&mut feed_config1);
        pinnacle.write(FEED_CONFIG1_ADDR, feed_config1)?;
        Ok(pinnacle)
    }
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
        let res = self.write(STATUS1_ADDR, 0x00);
        res
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

const WRITE_BITS: u8 = 0b_1000_0000;
const READ_BITS: u8 = 0b_1010_0000;
const ADDR_MASK: u8 = 0b_0001_1111;
const READ_FILL: u8 = 0xFB;
const READ_CONTINUE: u8 = 0xFC;

const FIRMWARE_ID_ADDR: u8 = 0x00;
const FIRMWARE_VERSION_ADDR: u8 = 0x01;
const STATUS1_ADDR: u8 = 0x02;
const SYS_CONFIG1_ADDR: u8 = 0x03;
const FEED_CONFIG1_ADDR: u8 = 0x04;
const FEED_CONFIG2_ADDR: u8 = 0x05;
const CAL_CONFIG1_ADDR: u8 = 0x07;
const PS2_AUX_CTRL_ADDR: u8 = 0x08;
const SAMPLE_RATE_ADDR: u8 = 0x09;
const Z_IDLE_ADDR: u8 = 0x0A;
const Z_SCALER_ADDR: u8 = 0x0B;
const SLEEP_INTERVAL_ADDR: u8 = 0x0C;
const SLEEP_TIMER_ADDR: u8 = 0x0D;
const PACKET_BYTE_0_ADDR: u8 = 0x12;
const PACKET_BYTE_1_ADDR: u8 = 0x13;
const PACKET_BYTE_2_ADDR: u8 = 0x14;
const PACKET_BYTE_3_ADDR: u8 = 0x15;
const PACKET_BYTE_4_ADDR: u8 = 0x16;
const PACKET_BYTE_5_ADDR: u8 = 0x17;
const PRODUCT_ID_ADDR: u8 = 0x1F;
