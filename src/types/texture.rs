/// Describes a texture's basic properties.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TextureDesc {
    pub height: u32,
    pub width: u32,
    pub format: ColorFormat,
}

/// enumeration of color formats. this is mainly used to convert color formats
/// from different libraries into a common format.
///
/// you can convert between DXGI_FORMAT and this format using into.
///
/// For example:
///
/// ```
///     let format_dxgi: DXGI_FORMAT = ColorFormat::ARGBUNorm.into();
///     let format: ColorFormat = DXGI_FORMAT_R8G8B8A8_UNORM.into();
/// ```
///
/// when using this in your own project, feel free to implement From and Into
/// traits that convert from other packages like nvenc or intel quick sync.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub enum ColorFormat {
    #[default]
    Unknown,

    // regular formats
    /// Packed 8bit per pixel ARGB unsigned normalized int format
    ARGB8UNorm,

    /// Packed 8bit per pixel ABGR unsigned normalized int format
    ABGR8UNorm,

    /// planar 8bit per pixel YUV 4:4:4 format
    YUV444,

    /// packed 8bit per pixel AYUV 4:4:4 format with alpha channel
    AYUV,

    /// planar 8bit per pixel YUV 4:2:0 format u,v planes have half height and half width of Y plane
    YUV420,

    /// semi planar 8bit per pixel YUV 4:2:0. Y followed by interleaved u,v plane.
    NV12,

    // 10 bit options
    /// packed 16 bits per pixel ARGB float format.
    ARGB16Float,

    /// packed 10 bits per channel for R,G,B channels and 2 bits for alpha channel. total 32 bits per pixel
    ARGB10UNorm,

    /// packed 10 bits per channel for YUV and 2 bits for alpha channel. YUV 4:4:4 format
    Y410,

    /// planar 16bit per pixel YUV 4:4:4 format. (only 10 significant bits will be used)
    YUV444_10bit,

    /// 16 bit Semi-Planar YUV. Y plane followed by interleaved UV plane . Each pixel of size 2 bytes. Most Significant 10 bits contain pixel data.
    /// this format is also called P010
    YUV420_10bit,
}
