use crate::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::spi::*;

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

pub fn new<S, D, Delay>(
    spi: S,
    config: Config,
    delay: &mut Delay,
    data: D,
) -> Result<Touchpad<S, <D as Build>::Data>, S::Error>
where
    S: SpiDevice<u8>,
    D: Build,
    <D as Build>::Data: TouchpadData,
    Delay: DelayNs,
{
    let mut pinnacle = Touchpad::new(spi);
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
