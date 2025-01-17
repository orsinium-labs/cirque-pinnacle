use crate::*;
use core::marker::PhantomData;
use embedded_hal::spi::{Operation, SpiDevice};

pub struct Touchpad<S: SpiDevice<u8>, M: Mode> {
    spi: S,
    phantom_: PhantomData<M>,
}

/// Pinnacle has four power modes - Active (touch detected), Idle (no touch),
/// Low Power/ Sleep (lower power after ~ 5 seconds of inactivity)
/// and Shutdown/Standby (no data reported).
#[derive(Copy, Clone, Debug)]
pub enum PowerMode {
    /// By default, Pinnacle toggles between Active and Idle mode. Pinnacle is in
    /// Active mode when a touch is detected (that is, a finger or stylus is present
    /// and is moving or tapping on the trackpad). The measurement system is active
    /// and data packets are being created and then sent to the host. Active mode
    /// begins as soon as a touch is detected. Idle mode is entered when the finger
    /// has been removed and there are no data packets to be sent. While in Idle mode,
    /// Pinnacle wakes every 10 milliseconds to check for a touch.
    Active,

    /// Enabling sleep mode will cause Pinnacle to go into a low power mode
    /// (around 50 μA) within 5 seconds of no touch detection. While in sleep mode,
    /// Pinnacle will wake within 300 ms to report any detection of a finger/stylus.
    Sleep,

    /// Shutdown/Standby mode is a very low power mode
    /// and Pinnacle does not track touch in this mode.
    Shutdown,
}

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

    /// When a touch is detected, Pinnacle loads X and Y position data into the
    /// position registers and asserts the `SW_DR` flag, which also triggers
    /// the `HW_DR` signal. While the finger/stylus is present, the position
    /// registers are updated every 10 ms and `SW_DR` and `HW_DR` are asserted.
    pub fn status(&mut self) -> Result<Status, S::Error> {
        let status = self.read(STATUS1_ADDR)?;
        Ok(Status {
            command_complete: status & 0b1000 != 0,
            data_ready: status & 0b0100 != 0,
        })
    }

    /// Forces Pinnacle to re-calibrate.
    ///
    /// If the touchpad is reporting touches when no fingers are on the pad
    /// then calibration (compensation) is wrong. Calling this function
    /// will fix the problem.
    ///
    /// **IMPORTANT:** after calling this function, wait until [`Touchpad::calibrated`]
    /// is true and then [`Touchpad::enable_feed`]. Until then, the device will not
    /// produce any data and you should not touch the device.
    pub fn calibrate(&mut self, c: &Calibration) -> Result<(), S::Error> {
        let mut data = 1;
        if c.background_comp {
            data |= 0x02;
        }
        if c.nerd_comp {
            data |= 0x04;
        }
        if c.track_error_comp {
            data |= 0x08;
        }
        if c.tap_comp {
            data |= 0x10;
        }
        if c.palm_error_comp {
            data |= 0x20;
        }
        if !c.calibration_matrix {
            data |= 0x40;
        }
        if c.force_precalibration_noise_check {
            data |= 0x80;
        }
        self.disable_feed()?;
        self.write(CAL_CONFIG1_ADDR, data)
    }

    /// Check if the touchpad is calibrated.
    pub fn calibrated(&mut self) -> Result<bool, S::Error> {
        let config = self.read(CAL_CONFIG1_ADDR)?;
        Ok((config & 1) != 0)
    }

    /// Clear Command Complete and Software Data Ready flags simultaneously.
    pub fn clear_flags(&mut self) -> Result<(), S::Error> {
        self.write_with_delay(STATUS1_ADDR, 0x00)
    }

    /// Number of samples generated per second.
    pub fn sample_rate(&mut self) -> Result<u8, S::Error> {
        self.read(SAMPLE_RATE_ADDR)
    }

    /// Set the number of samples generated per second.
    pub fn set_sample_rate(&mut self, sample_rate: u8) -> Result<(), S::Error> {
        self.write(SAMPLE_RATE_ADDR, sample_rate)
    }

    /// During Z-idle (no touch detected) and when in absolute data mode,
    /// Pinnacle will continue to send empty packets (both X and Y data set to 0x00)
    /// every 10 ms. The number of empty packets to be sent can be set using
    /// [`Touchpad::set_z_idle`]. The default value is 0x1E (30 decimal).
    /// When set to zero (0), this register prevents any empty packets from being sent,
    /// and the position registers will contain the last sensed location until
    /// a new finger presence is detected.
    ///
    /// The Z-Idle count can be a helpful design tool. For example, tap-frequency
    /// can be determined by counting the number of Z-idle packets reported
    /// between a finger lifting off and touching back down
    /// (cutting short the stream of Z-idle packets).
    pub fn z_idle(&mut self) -> Result<u8, S::Error> {
        self.read(Z_IDLE_ADDR)
    }

    /// Set the number of empty packets sent during Z-idle.
    pub fn set_z_idle(&mut self, z_idle: u8) -> Result<(), S::Error> {
        self.write(Z_IDLE_ADDR, z_idle)
    }

    /// Contains the pen Z_On threshold.
    pub fn z_scaler(&mut self) -> Result<u8, S::Error> {
        self.read(Z_SCALER_ADDR)
    }

    /// Set the pen Z_On threshold.
    pub fn set_z_scaler(&mut self, z_scaler: u8) -> Result<(), S::Error> {
        self.write(Z_SCALER_ADDR, z_scaler)
    }

    pub fn sleep_timer(&mut self) -> Result<u8, S::Error> {
        self.read(SLEEP_TIMER_ADDR)
    }

    pub fn set_sleep_timer(&mut self, sleep_timer: u8) -> Result<(), S::Error> {
        self.write(SLEEP_TIMER_ADDR, sleep_timer)
    }

    pub fn sleep_interval(&mut self) -> Result<u8, S::Error> {
        self.read(SLEEP_INTERVAL_ADDR)
    }

    pub fn set_sleep_interval(&mut self, sleep_interval: u8) -> Result<(), S::Error> {
        self.write(SLEEP_INTERVAL_ADDR, sleep_interval)
    }

    pub fn disable_feed(&mut self) -> Result<(), S::Error> {
        let config = self.read(FEED_CONFIG1_ADDR)?;
        let config = config & !0x01;
        self.write(FEED_CONFIG1_ADDR, config)
    }

    pub fn enable_feed(&mut self) -> Result<(), S::Error> {
        let config = self.read(FEED_CONFIG1_ADDR)?;
        let config = config | 0x01;
        self.write(FEED_CONFIG1_ADDR, config)
    }

    /// Get the current power mode.
    pub fn power_mode(&mut self) -> Result<PowerMode, S::Error> {
        let mode = self.read(SYS_CONFIG1_ADDR)?;
        if mode & 0b10 != 0 {
            return Ok(PowerMode::Shutdown);
        }
        if mode & 0b100 != 0 {
            return Ok(PowerMode::Sleep);
        }
        Ok(PowerMode::Active)
    }

    /// Set the power mode.
    pub fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), S::Error> {
        let mode = match mode {
            PowerMode::Sleep => 0b100,
            PowerMode::Shutdown => 0b10,
            PowerMode::Active => 0b0,
        };
        self.write(SYS_CONFIG1_ADDR, mode)
    }

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
        let buf = [addr, data];
        self.spi.write(&buf)
    }

    fn write_with_delay(&mut self, addr: u8, data: u8) -> Result<(), S::Error> {
        const US: u32 = 1000; // microseconds in nanosecond
        let addr = WRITE_BITS | (addr & ADDR_MASK);
        let buf = [addr, data];
        self.spi.transaction(&mut [
            Operation::Write(&buf),
            // wait "Inter-Message Transfer Delay (required by slave)"
            Operation::DelayNs(50 * US),
        ])
    }
}

impl<S: SpiDevice<u8>> Touchpad<S, Absolute> {
    pub fn read_absolute(&mut self) -> Result<AbsoluteData, S::Error> {
        let data = self.read_multi::<6>(PACKET_BYTE_0_ADDR)?;
        let data = AbsoluteData {
            x: u16::from(data[2]) | (u16::from(data[4] & 0b_1111) << 8),
            // The official example takes 4 bits from data[4] but we take only 3.
            // That's because the docs say the max value for Y is 1535,
            // which is `0b101_1111_1111` in binary. This leaves 3 bits for
            // the left byte. If we take 4th byte, it is, on practice, is always 1
            // (at least on the touchpads I have) which gives Y value alwy over 2048,
            // even if no touch detected.
            y: u16::from(data[3]) | (u16::from(data[4] & 0b_0111_0000) << 4),
            z: data[5] & 0b_0011_1111,
            button_flags: data[0] & 0b_0011_1111,
        };
        self.clear_flags()?;
        Ok(data)
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
        let data = RelativeData {
            x,
            y,
            buttons: Buttons {
                primary: data[0] & 0b001 != 0,
                secondary: data[0] & 0b010 != 0,
                auxiliary: data[0] & 0b100 != 0,
            },
            #[expect(clippy::cast_possible_wrap)]
            wheel: data[3] as i8,
        };
        self.clear_flags()?;
        Ok(data)
    }
}
