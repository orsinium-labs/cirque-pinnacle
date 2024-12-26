pub struct Relative;

pub struct Absolute {
    invert_x: bool,
    invert_y: bool,
}

pub trait Mode {
    #[must_use]
    fn build(&self, feed_config1: u8) -> u8;
}

impl Mode for Absolute {
    fn build(&self, feed_config1: u8) -> u8 {
        feed_config1 | (u8::from(self.invert_y) << 7 | u8::from(self.invert_x) << 6 | 1 << 1)
    }
}

impl Mode for Relative {
    fn build(&self, feed_config1: u8) -> u8 {
        feed_config1 & !(1 << 1)
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

#[derive(Copy, Clone, Debug)]
pub struct RelativeData {
    pub x: i16,
    pub y: i16,
    pub button_flags: u8,
    pub wheel: i8,
}
