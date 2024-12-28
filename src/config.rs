use crate::*;
use embedded_hal::spi::SpiDevice;

#[expect(clippy::struct_excessive_bools)]
pub struct Calibration {
    pub background_comp: bool,
    pub nerd_comp: bool,
    pub track_error_comp: bool,
    pub tap_comp: bool,
    pub palm_error_comp: bool,
    pub calibration_matrix: bool,
    pub force_precalibration_noise_check: bool,
}

impl Default for Calibration {
    fn default() -> Self {
        Self {
            background_comp: false,
            nerd_comp: false,
            track_error_comp: false,
            tap_comp: false,
            palm_error_comp: false,
            calibration_matrix: true,
            force_precalibration_noise_check: true,
        }
    }
}

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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            x: true,
            y: true,
            filter: true,
        }
    }
}

impl Config {
    pub fn init<M, S>(self, mode: &M, spi: S) -> Result<Touchpad<S, M>, S::Error>
    where
        S: SpiDevice<u8>,
        M: Mode,
    {
        let mut pinnacle = Touchpad::new(spi);
        pinnacle.clear_flags()?;
        pinnacle.set_power_mode(PowerMode::Active)?;
        let feed_config2 = mode.build2();
        pinnacle.write(FEED_CONFIG2_ADDR, feed_config2)?;

        let feed_config1 = 1
            | u8::from(!self.y) << 4
            | u8::from(!self.x) << 3
            | u8::from(!self.filter) << 2
            | mode.build1();
        pinnacle.write(FEED_CONFIG1_ADDR, feed_config1)?;
        Ok(pinnacle)
    }
}
