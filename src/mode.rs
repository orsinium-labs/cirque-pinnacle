#[expect(clippy::struct_excessive_bools)]
pub struct Relative {
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
pub struct Absolute {
    /// X data Invert.
    ///
    /// Y and X data count invert is only available when in absolute mode.
    invert_x: bool,

    /// Y data Invert.
    ///
    /// Y and X data count invert is only available when in absolute mode.
    invert_y: bool,
}

pub trait Mode {
    /// Modify FEED_CONFIG1 value.
    #[must_use]
    fn build1(&self) -> u8;

    /// Provide FEED_CONFIG2 value.
    #[must_use]
    fn build2(&self) -> u8;
}

impl Mode for Absolute {
    fn build1(&self) -> u8 {
        u8::from(self.invert_y) << 7 | u8::from(self.invert_x) << 6 | 1 << 1
    }

    fn build2(&self) -> u8 {
        0b11111 // disable all relative mode features
    }
}

impl Mode for Relative {
    fn build1(&self) -> u8 {
        0
    }

    fn build2(&self) -> u8 {
        u8::from(self.swap_x_y) << 7
            | u8::from(!self.glide_extend) << 4
            | u8::from(!self.scroll) << 3
            | u8::from(!self.secondary_tap) << 2
            | u8::from(!self.taps) << 1
            | u8::from(self.intellimouse)
    }
}

pub trait TouchpadData {}

impl TouchpadData for AbsoluteData {}

impl TouchpadData for RelativeData {}

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
