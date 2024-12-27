use crate::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::spi::SpiDevice;

#[expect(clippy::struct_excessive_bools)]
pub struct Config {
    /// Set to false to disable X.
    ///
    /// Disabling the X-axis will not allow regular tracking and is not recommended
    /// for typical applications.
    pub x: bool,

    /// Set to false to disable Y.
    ///
    /// Disabling the Y-axis will not allow regular tracking and is not recommended
    /// for typical applications.
    pub y: bool,

    /// Set to false to disable filter.
    ///
    /// The Filter disable bit controls whether the filtering algorithm is applied
    /// to generated data. By default the hardware filters are enabled.
    /// Cirque does not recommend disabling hardware filtering.
    pub filter: bool,

    pub calibrate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            x: true,
            y: true,
            filter: true,
            calibrate: false,
        }
    }
}

impl Config {
    pub fn init<M, S, D>(self, mode: &M, spi: S, delay: &mut D) -> Result<Touchpad<S, M>, S::Error>
    where
        S: SpiDevice<u8>,
        M: Mode,
        D: DelayNs,
    {
        let mut pinnacle = Touchpad::new(spi);
        pinnacle.write(STATUS1_ADDR, 0x00)?; // SW_CC
        delay.delay_us(50);
        let feed_config2 = mode.build2();
        pinnacle.write(SYS_CONFIG1_ADDR, 0x00)?;
        pinnacle.write(FEED_CONFIG2_ADDR, feed_config2)?;
        if self.calibrate {
            let calibrate_config = 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1;
            pinnacle.write(CAL_CONFIG1_ADDR, calibrate_config)?;
        }

        let feed_config1 = 1
            | u8::from(!self.y) << 4
            | u8::from(!self.x) << 3
            | u8::from(!self.filter) << 2
            | mode.build1();
        pinnacle.write(FEED_CONFIG1_ADDR, feed_config1)?;
        Ok(pinnacle)
    }
}
