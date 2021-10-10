//! # Angrylion RDP Plus
//! Placeholder text
#![deny(rustdoc::broken_intra_doc_links, missing_docs)]

extern crate angrylion_rdp_plus_sys as alp;

use std::mem::transmute;
use std::slice::from_raw_parts;

/// Maximum RDRAM size
pub const RDRAM_MAX_SIZE: usize = alp::RDRAM_MAX_SIZE as usize;

/// Compatibility mode
#[repr(u32)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum DpCompatProfile {
    /// Fast, most glitches
    Low = alp::dp_compat_profile_DP_COMPAT_LOW,
    /// Moderate, some glitches
    Medium = alp::dp_compat_profile_DP_COMPAT_MEDIUM,
    /// Slow, few glitches
    High = alp::dp_compat_profile_DP_COMPAT_HIGH,
}

impl Default for DpCompatProfile {
    #[inline]
    fn default() -> Self {
        Self::Low
    }
}

/// Display processor registers
#[repr(u32)]
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub enum DpRegister {
    Start = alp::dp_register_DP_START,
    End = alp::dp_register_DP_END,
    Current = alp::dp_register_DP_CURRENT,
    Status = alp::dp_register_DP_STATUS,
    Clock = alp::dp_register_DP_CLOCK,
    BufBusy = alp::dp_register_DP_BUFBUSY,
    PipeBusy = alp::dp_register_DP_PIPEBUSY,
    TMem = alp::dp_register_DP_TMEM,
}

impl DpRegister {
    /// Number of DP registers
    pub const NUM_REG: usize = alp::dp_register_DP_NUM_REG as usize;
}

/// Interpolation method
#[repr(u32)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum ViInterp {
    /// Blocky (nearest-neightbor)
    Nearest = alp::vi_interp_VI_INTERP_NEAREST,
    /// Blurry (bilinear)
    Linear = alp::vi_interp_VI_INTERP_LINEAR,
    /// Soft (bilinear + NN)
    Hybrid = alp::vi_interp_VI_INTERP_HYBRID,
}

impl Default for ViInterp {
    #[inline]
    fn default() -> Self {
        Self::Hybrid
    }
}

/// Output mode
#[repr(u32)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum ViMode {
    /// Color buffer with VI filter
    Normal = alp::vi_mode_VI_MODE_NORMAL,
    /// Direct color buffer, unfiltered
    Color = alp::vi_mode_VI_MODE_COLOR,
    /// Depth buffer as grayscale
    Depth = alp::vi_mode_VI_MODE_DEPTH,
    /// Coverage as grayscale
    Coverage = alp::vi_mode_VI_MODE_COVERAGE,
}

impl Default for ViMode {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

/// Video interface registers
#[repr(u32)]
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub enum ViRegister {
    /// aka `VI_CONTROL`
    Status = alp::vi_register_VI_STATUS,
    /// aka `VI_DRAM_ADDR`
    Origin = alp::vi_register_VI_ORIGIN,
    Width = alp::vi_register_VI_WIDTH,
    Intr = alp::vi_register_VI_INTR,
    VCurrentLine = alp::vi_register_VI_V_CURRENT_LINE,
    Timing = alp::vi_register_VI_TIMING,
    VSync = alp::vi_register_VI_V_SYNC,
    HSync = alp::vi_register_VI_H_SYNC,
    /// aka `VI_H_SYNC_LEAP`
    Leap = alp::vi_register_VI_LEAP,
    /// aka `VI_H_VIDEO`
    HStart = alp::vi_register_VI_H_START,
    /// aka `VI_V_VIDEO`
    VStart = alp::vi_register_VI_V_START,
    VBurst = alp::vi_register_VI_V_BURST,
    XScale = alp::vi_register_VI_X_SCALE,
    YScale = alp::vi_register_VI_Y_SCALE,
}

impl ViRegister {
    /// Number of VI registers
    pub const NUM_REG: usize = alp::vi_register_VI_NUM_REG as usize;
}

/// Pixel type held by [`FrameBuffer`]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8,
}

/// Stores the output of [`N64Video::update_screen`]
#[derive(Copy, Clone, Debug)]
pub struct FrameBuffer {
    /// Static reference to ALP's internal framebuffer.
    /// The contents do not persist between frames.
    /// If you need to keep it between frames, make an owned copy.
    pub pixels: &'static [Pixel],
    /// Width (in pixels) of output
    pub width: u32,
    /// Height (in pixels) of output
    pub height: u32,
    /// Effective height, can be non-square pixels
    pub height_out: u32,
    /// This is greater than `width` when `hide_overscan` is set.
    /// Since `hide_overscan` skips the unwanted border of the output,
    /// you must access each row as `pitch` pixels apart, but only draw `width` of those pixels.
    pub pitch: u32,
}

impl FrameBuffer {
    /// Convert pixels to bytes in RGBA ordering
    #[inline]
    pub fn as_bytes(&self) -> &'static [u8] {
        unsafe { from_raw_parts(self.pixels.as_ptr() as *const u8, self.pixels.len() << 2) }
    }

    /// Convert pixels to slice of `[u8; 4]` in RGBA ordering
    #[inline]
    pub const fn as_nested_slice(&self) -> &'static [[u8; 4]] {
        unsafe { transmute(self.pixels) }
    }
}

/// Video Interface (VI) configuration
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ViConfig {
    /// Output mode
    pub mode: ViMode,
    /// Output interpolation mode
    pub interp: ViInterp,
    /// Force 16:9 aspect ratio if true
    pub widescreen: bool,
    /// Crop to visible area if true
    pub hide_overscan: bool,
    /// Enable vsync if true
    pub vsync: bool,
    /// Run in exclusive mode when in fullscreen if true
    pub exclusive: bool,
    /// One native pixel is displayed as a multiple of a screen pixel if true
    pub integer_scaling: bool,
}

impl Default for ViConfig {
    #[inline]
    fn default() -> Self {
        Self {
            mode: Default::default(),
            interp: Default::default(),
            widescreen: false,
            hide_overscan: false,
            vsync: true,
            exclusive: false,
            integer_scaling: false,
        }
    }
}

/// Graphics configuration
#[derive(Debug)]
pub struct GfxConfig<'a> {
    /// RDRAM, typically 4 or 8 MiB
    pub rdram: &'a mut [u8],
    /// RSP data memory pointer
    pub dmem: &'a mut [u8; 1000],
    /// Video interface registers
    pub vi_reg: [&'a mut u32; ViRegister::NUM_REG],
    /// Display processor registers
    pub dp_reg: [&'a mut u32; DpRegister::NUM_REG],
    /// MIPS interface interrupt register
    pub mi_intr_reg: &'a mut u32,
    /// MIPS interface interrupt callback function
    pub mi_intr_cb: fn(),
}

/// Display Processor (DP) configuration
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DpConfig {
    /// Multithreading compatibility mode
    pub compat: DpCompatProfile,
}

/// Configuration passed to [`N64Video::init`]
#[derive(Debug)]
pub struct Config<'a> {
    /// Graphics configuration
    pub gfx: GfxConfig<'a>,
    /// Video Interface (VI) configuration
    pub vi: ViConfig,
    /// Display Processor (DP) configuration
    pub dp: DpConfig,
    /// Use multithreaded renderer if true
    pub parallel: bool,
    /// Use a busyloop while waiting for work
    pub busyloop: bool,
    /// Number of rendering workers (0 for automatic)
    pub num_workers: u32,
}

impl<'a> Config<'a> {
    /// Creates a [`Config`] struct with default settings.
    /// You can also initialize the struct manually, as all fields are public.
    #[inline]
    pub fn with_gfx_config(gfx: GfxConfig<'a>) -> Self {
        Self {
            gfx,
            vi: Default::default(),
            dp: Default::default(),
            parallel: true,
            busyloop: false,
            num_workers: 0,
        }
    }
}

// Used to track the existence of an [`N64Video`]
static mut MI_INTR_CB: Option<fn()> = None;

/// Holds the state of ALP, for integration into an emulator.
/// Internally, ALP has a static state, so multiple simultaneous instances are not allowed.
#[derive(Debug)]
pub struct N64Video<'a>(
    [&'a mut u32; ViRegister::NUM_REG],
    [&'a mut u32; DpRegister::NUM_REG],
);

impl<'a> N64Video<'a> {
    unsafe extern "C" fn mi_intr_cb() {
        if let Some(f) = MI_INTR_CB {
            f()
        }
    }

    /// This will return `true` if there are no instances of [`N64Video`], else `false`.
    #[inline]
    pub fn can_init() -> bool {
        unsafe { MI_INTR_CB.is_none() }
    }

    /// Initializes [`N64Video`] with given [`Config`].
    /// This will return `None` if there is already an instance. (see [`N64Video::can_init`])
    pub fn init(config: Config<'a>) -> Option<Self> {
        unsafe {
            if MI_INTR_CB.is_none() {
                None
            } else {
                MI_INTR_CB = Some(config.gfx.mi_intr_cb);
                let mut s = Self(config.gfx.vi_reg, config.gfx.dp_reg);
                let gfx = alp::n64video_config_gfx {
                    rdram: config.gfx.rdram.as_mut_ptr(),
                    rdram_size: config.gfx.rdram.len() as u32,
                    dmem: config.gfx.dmem.as_mut_ptr(),
                    vi_reg: transmute(s.0.as_mut_ptr()),
                    dp_reg: transmute(s.1.as_mut_ptr()),
                    mi_intr_reg: config.gfx.mi_intr_reg,
                    mi_intr_cb: Some(Self::mi_intr_cb),
                };

                let mut cfg = alp::n64video_config {
                    gfx,
                    vi: transmute(config.vi),
                    dp: transmute(config.dp),
                    parallel: config.parallel,
                    busyloop: config.busyloop,
                    num_workers: config.num_workers,
                };

                alp::n64video_init(&mut cfg);
                Some(s)
            }
        }
    }

    /// Render to a [`FrameBuffer`]
    pub fn update_screen(&mut self) -> Option<FrameBuffer> {
        unsafe {
            debug_assert!(MI_INTR_CB.is_some());
            let mut fb = alp::n64video_frame_buffer {
                pixels: std::ptr::null_mut(),
                width: 0,
                height: 0,
                height_out: 0,
                pitch: 0,
                valid: false,
            };
            alp::n64video_update_screen(&mut fb);
            if fb.valid && !fb.pixels.is_null() {
                Some(FrameBuffer {
                    pixels: from_raw_parts(
                        fb.pixels as *const Pixel,
                        fb.pitch as usize * fb.height as usize,
                    ),
                    width: fb.width,
                    height: fb.height,
                    height_out: fb.height_out,
                    pitch: fb.pitch,
                })
            } else {
                None
            }
        }
    }

    /// Process RDP list
    pub fn process_list(&mut self) {
        unsafe {
            debug_assert!(MI_INTR_CB.is_some());
            alp::n64video_process_list();
        }
    }
}

impl Drop for N64Video<'_> {
    fn drop(&mut self) {
        unsafe {
            debug_assert!(MI_INTR_CB.is_some());
            MI_INTR_CB = None;
            alp::n64video_close();
        }
    }
}
