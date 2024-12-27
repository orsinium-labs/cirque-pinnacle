use crate::*;
use core::marker::PhantomData;
use embedded_hal::spi::{Operation, SpiDevice};

pub struct Touchpad<S: SpiDevice<u8>, M: Mode> {
    spi: S,
    phantom_: PhantomData<M>,
}

#[derive(Debug)]
pub enum SampleRate {
    S100,
    S80,
    S60,
    S40,
    S20,
    S10,
}

pub struct Status {
    /// Command Complete (`SW_CC`).
    ///
    /// Asserted after calibration, POR. Remains asserted until cleared by host.
    pub command_complete: bool,

    /// Software Data Ready (`SW_DR`).
    ///
    /// Asserted with new data. Remains asserted until cleared by host.
    pub data_ready: bool,
}

impl<S: SpiDevice<u8>, M: Mode> Touchpad<S, M> {
    pub(crate) const fn new(spi: S) -> Self {
        Self {
            spi,
            phantom_: PhantomData,
        }
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

    /// When a touch is detected, Pinnacle loads X and Y position data into the position registers and
    /// asserts the `SW_DR` flag (Bit [2] of Register 0x02, Status 1), which also triggers the `HW_DR`
    /// signal. While the finger/stylus is present, the position registers are updated every
    /// 10 ms and `SW_DR` and `HW_DR` are asserted.
    pub fn status(&mut self) -> Result<Status, S::Error> {
        let status = self.read(STATUS1_ADDR)?;
        Ok(Status {
            command_complete: status & 0b1000 != 0,
            data_ready: status & 0b0100 != 0,
        })
    }

    /// Clear Command Complete and Software Data Ready flags simultaneously.
    pub fn clear_flags(&mut self) -> Result<(), S::Error> {
        self.write(STATUS1_ADDR, 0x00)
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

    pub fn set_sample_rate(&mut self, _sample_rate: &SampleRate) -> Result<(), S::Error> {
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
        self.spi.transfer_in_place(&mut buf)?;
        Ok(buf[3])
    }

    fn read_multi<const N: usize>(&mut self, addr: u8) -> Result<[u8; N], S::Error> {
        let addr = READ_BITS | (addr & ADDR_MASK);
        let mut buf = [READ_CONTINUE; N];
        buf[N - 1] = READ_FILL;
        let mut addr_buf = [addr, READ_CONTINUE, READ_CONTINUE];
        self.spi.transaction(&mut [
            Operation::TransferInPlace(&mut addr_buf),
            Operation::TransferInPlace(&mut buf),
        ])?;
        Ok(buf)
    }

    pub(crate) fn write(&mut self, addr: u8, data: u8) -> Result<(), S::Error> {
        let addr = WRITE_BITS | (addr & ADDR_MASK);
        let mut buf = [addr, data];
        self.spi.transfer_in_place(&mut buf)?;
        Ok(())
    }
}

impl<S: SpiDevice<u8>> Touchpad<S, Absolute> {
    pub fn read_absolute(&mut self) -> Result<Option<AbsoluteData>, S::Error> {
        // let status =
        let data = self.read_multi::<6>(PACKET_BYTE_0_ADDR)?;
        let data = AbsoluteData {
            x: u16::from(data[2]) | (u16::from(data[4] & 0x0F) << 8),
            y: u16::from(data[3]) | (u16::from(data[4] & 0xF0) << 4),
            z: data[5] & 0x3F,
            button_flags: data[0] & 0x3F,
        };
        Ok(Some(data))
    }
}

impl<S: SpiDevice<u8>> Touchpad<S, Relative> {
    pub fn read_relative(&mut self) -> Result<RelativeData, S::Error> {
        let data = self.read_multi::<4>(PACKET_BYTE_0_ADDR)?;
        let mut x = i16::from(data[1]);
        let mut y = i16::from(data[2]);
        if (data[0] & 0x10) > 0 {
            x -= 256;
        }
        if (data[0] & 0x20) > 0 {
            y -= 256;
        }
        Ok(RelativeData {
            x,
            y,
            buttons: Buttons {
                primary: data[0] & 0b001 != 0,
                secondary: data[0] & 0b010 != 0,
                auxiliary: data[0] & 0b100 != 0,
            },
            #[expect(clippy::cast_possible_wrap)]
            wheel: data[3] as i8,
        })
    }
}
