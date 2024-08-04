/// Enum for display orientation
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default)]
pub enum DisplayOrientation {
    /// Landscape mode
    #[default]
    NoRotation,

    /// Portrait mode
    Rotate90,

    /// Landscape (flipped) mode
    Rotate180,

    /// Portrait (flipped) mode
    Rotate270,
}

#[repr(C)]
#[derive(Clone, Default, Debug)]
/**
DisplayMode represents one display mode of monitor. It contains resolution, refresh-rate and orientation.
The resolution contains width and height of display for their default orientation.

For example, a 1920 x 1080 monitor will have width 1920 and height 1080 irrespective of the orientation of
the monitor.
 */
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DisplayMode {
    /// width of the given display in pixels
    pub width: u32,
    /// height of the given display in pixels
    pub height: u32,

    /// orientation of the display
    pub orientation: DisplayOrientation,

    /// refresh-rate is usually represented as a fraction. refresh_num is numerator of that fraction
    pub refresh_num: u32,
    /// refresh_den is denominator of refresh-rate fraction.
    pub refresh_den: u32,

    /// this determines if the display is using 8bit or 16bit output mode. (10 bit is
    /// represented as 16 bit in windows)
    pub hdr: bool,
}
