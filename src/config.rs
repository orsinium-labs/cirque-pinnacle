use crate::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::spi::SpiDevice;

#[expect(clippy::struct_excessive_bools)]
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
        let feed_config2 = u8::from(self.swap_x_y) << 7
            | u8::from(!self.glide_extend) << 4
            | u8::from(!self.scroll) << 4
            | u8::from(!self.secondary_tap) << 2
            | u8::from(!self.all_taps) << 1
            | u8::from(self.intellimouse);
        pinnacle.write(SYS_CONFIG1_ADDR, 0x00)?;
        pinnacle.write(FEED_CONFIG2_ADDR, feed_config2)?;
        if self.calibrate {
            let calibrate_config = 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1;
            pinnacle.write(CAL_CONFIG1_ADDR, calibrate_config)?;
        }

        let feed_config1 =
            1 | u8::from(!self.y) << 4 | u8::from(!self.x) << 3 | u8::from(!self.filter) << 2;
        let feed_config1 = mode.build(feed_config1);
        pinnacle.write(FEED_CONFIG1_ADDR, feed_config1)?;
        Ok(pinnacle)
    }
}
