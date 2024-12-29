use crate::*;
use embedded_hal::spi::SpiDevice;

#[expect(clippy::struct_excessive_bools)]
pub struct Relative {
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

    /// Swap X & Y
    pub swap_x_y: bool,

    /// Enable GlideExtend®.
    ///
    /// GlideExtend is Cirque's patented motion extender technology that allows
    /// the user to continue the drag function when an edge is reached, by lifting
    /// and repositioning the finger.
    pub glide_extend: bool,

    /// Enable scroll.
    pub scroll: bool,

    /// Enable secondary tap.
    ///
    /// Secondary Taps allows a tap in the upper right corner (standard orientation)
    /// to simulate an activation of the secondary button.
    pub secondary_tap: bool,

    /// If false, disables all taps.
    ///
    /// Disabling all taps disables secondary taps, even if secondary tap
    /// is explicitly enabled.
    pub taps: bool,

    /// Enable Intellimouse.
    ///
    /// Intellimouse enabled will change Pinnacle's relative data packet to four bytes
    /// rather than three. The fourth byte (PacketByte_3) will report scroll data
    /// (referred to as wheel count).
    pub intellimouse: bool,
}

#[derive(Default)]
#[expect(clippy::struct_excessive_bools)]
pub struct Absolute {
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

    /// X data Invert.
    ///
    /// Y and X data count invert is only available when in absolute mode.
    invert_x: bool,

    /// Y data Invert.
    ///
    /// Y and X data count invert is only available when in absolute mode.
    invert_y: bool,
}

impl Relative {
    pub fn init<S: SpiDevice<u8>>(&self, spi: S) -> Result<Touchpad<S, Self>, S::Error> {
        let config1 =
            1 | u8::from(!self.y) << 4 | u8::from(!self.x) << 3 | u8::from(!self.filter) << 2;
        let config2 = u8::from(self.swap_x_y) << 7
            | u8::from(!self.glide_extend) << 4
            | u8::from(!self.scroll) << 3
            | u8::from(!self.secondary_tap) << 2
            | u8::from(!self.taps) << 1
            | u8::from(self.intellimouse);
        init(spi, config1, config2)
    }
}

impl Absolute {
    pub fn init<S: SpiDevice<u8>>(&self, spi: S) -> Result<Touchpad<S, Self>, S::Error> {
        let config1 = 1
            | u8::from(self.invert_y) << 7
            | u8::from(self.invert_x) << 6
            | u8::from(!self.y) << 4
            | u8::from(!self.x) << 3
            | u8::from(!self.filter) << 2
            | 1 << 1;
        let config2 = 0b11111; // disable all relative mode features;
        init(spi, config1, config2)
    }
}

pub trait Mode {}
impl Mode for Relative {}
impl Mode for Absolute {}

fn init<M, S>(spi: S, config1: u8, config2: u8) -> Result<Touchpad<S, M>, S::Error>
where
    S: SpiDevice<u8>,
    M: Mode,
{
    let mut pinnacle = Touchpad::new(spi);
    pinnacle.clear_flags()?;
    pinnacle.set_power_mode(PowerMode::Active)?;
    pinnacle.write(FEED_CONFIG2_ADDR, config2)?;
    pinnacle.write(FEED_CONFIG1_ADDR, config1)?;
    Ok(pinnacle)
}

#[derive(Copy, Clone, Debug)]
pub struct AbsoluteData {
    pub x: u16,
    pub y: u16,
    pub z: u8,
    pub button_flags: u8,
}

impl AbsoluteData {
    pub const X_MIN: u16 = 0;
    pub const Y_MIN: u16 = 0;
    pub const X_MAX: u16 = 2047;
    pub const Y_MAX: u16 = 1535;

    #[must_use]
    pub const fn touched(&self) -> bool {
        self.z != 0
    }

    /// Represent X as f32 on the range from -1.0 to +1.0.
    #[must_use]
    pub fn x_f32(&self) -> f32 {
        f32::from(self.x * 2) / f32::from(Self::X_MAX) - 1.0
    }

    /// Represent Y as f32 on the range from -1.0 to +1.0.
    #[must_use]
    pub fn y_f32(&self) -> f32 {
        f32::from(self.y * 2) / f32::from(Self::Y_MAX) - 1.0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RelativeData {
    pub x: i16,
    pub y: i16,
    pub wheel: i8,
    pub buttons: Buttons,
}

#[derive(Copy, Clone, Debug)]
pub struct Buttons {
    /// BTN Primary (or tap).
    pub primary: bool,
    /// BTN Secondary (or top right corner tap).
    pub secondary: bool,
    /// BTN Auxiliary.
    pub auxiliary: bool,
}
