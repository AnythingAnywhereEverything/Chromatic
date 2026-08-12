#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareAccel {
    Auto,
    Software,

    Cuda,
    Nvenc,
    Qsv,
    V4l2m2m,
    Vaapi,
    Vdpau,
    Opencl,
    Amf,
    Videotoolbox,
}

pub fn select_best_hardware_accel(
    available: &[HardwareAccel],
) -> HardwareAccel {
    const PRIORITY: &[HardwareAccel] = &[
        HardwareAccel::Nvenc,
        HardwareAccel::Qsv,
        HardwareAccel::Amf,
        HardwareAccel::Videotoolbox,
        HardwareAccel::Vaapi,
        HardwareAccel::V4l2m2m,
    ];

    PRIORITY
        .iter()
        .copied()
        .find(|accel| available.contains(accel))
        .unwrap_or(HardwareAccel::Software)
}

impl HardwareAccel {
    pub fn encoder(self) -> Option<&'static str> {
        match self {
            HardwareAccel::Nvenc => Some("h264_nvenc"),
            HardwareAccel::Qsv => Some("h264_qsv"),
            HardwareAccel::Amf => Some("h264_amf"),
            HardwareAccel::V4l2m2m => Some("h264_v4l2m2m"),
            HardwareAccel::Vaapi => Some("h264_vaapi"),
            HardwareAccel::Videotoolbox => Some("h264_videotoolbox"),

            // * These are acceleration APIs, not necessarily the HLS encoder.
            HardwareAccel::Cuda => Some("h264_nvenc"),
            HardwareAccel::Vdpau => None,
            HardwareAccel::Opencl => None,

            HardwareAccel::Auto | HardwareAccel::Software => None,
        }
    }

    pub fn hwaccel_args(self) -> &'static [&'static str] {
        match self {
            HardwareAccel::Cuda | HardwareAccel::Nvenc => {
                &["-hwaccel", "cuda"]
            }

            HardwareAccel::Qsv => {
                &["-hwaccel", "qsv"]
            }

            HardwareAccel::Vaapi => {
                &["-hwaccel", "vaapi"]
            }

            HardwareAccel::Videotoolbox => {
                &["-hwaccel", "videotoolbox"]
            }

            // * AMD AMF encoding doesn't require an -hwaccel argument.
            HardwareAccel::Amf => &[],

            HardwareAccel::V4l2m2m => {
                &["-hwaccel", "drm"]
            }

            HardwareAccel::Vdpau => {
                &["-hwaccel", "vdpau"]
            }

            HardwareAccel::Opencl
            | HardwareAccel::Auto
            | HardwareAccel::Software => &[],
        }
    }
}