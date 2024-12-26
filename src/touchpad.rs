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

impl<S: SpiDevice<u8>, M: Mode> Touchpad<S, M> {
    pub(crate) fn new(spi: S) -> Self {
        Self {
            spi,
            phantom_: PhantomData,
        }
    }

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

    pub(crate) fn write(&mut self, addr: u8, data: u8) -> Result<(), S::Error> {
        let addr = WRITE_BITS | (addr & ADDR_MASK);
        let mut buf = [addr, data];
        self.spi.transfer_in_place(&mut buf).unwrap();
        Ok(())
    }
}

impl<S: SpiDevice<u8>> Touchpad<S, Absolute> {
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

impl<S: SpiDevice<u8>> Touchpad<S, Relative> {
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
