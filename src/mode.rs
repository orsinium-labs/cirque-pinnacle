pub struct Relative;

pub struct Absolute {
    invert_x: bool,
    invert_y: bool,
}

pub trait Build {
    type Data;
    fn build(&self, feed_config1: &mut u8);
}

impl Build for Absolute {
    type Data = AbsoluteData;
    fn build(&self, feed_config1: &mut u8) {
        *feed_config1 |= (self.invert_y as u8) << 7 | (self.invert_x as u8) << 6 | 1 << 1;
    }
}

impl Build for Relative {
    type Data = RelativeData;
    fn build(&self, feed_config1: &mut u8) {
        *feed_config1 &= !(1 << 1);
    }
}

pub trait TouchpadData {}

impl TouchpadData for AbsoluteData {}

impl TouchpadData for RelativeData {}

#[derive(Copy, Clone)]
pub struct AbsoluteData {
    pub x: u16,
    pub y: u16,
    pub z: u8,
    pub button_flags: u8,
}

#[derive(Copy, Clone)]
pub struct RelativeData {
    pub x: i16,
    pub y: i16,
    pub button_flags: u8,
    pub wheel: i8,
}
