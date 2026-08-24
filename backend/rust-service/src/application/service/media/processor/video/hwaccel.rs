use tokio::process::Command;
use crate::application::service::errors::media_service::MediaProcessorError;
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

pub async fn get_available_hwaccels() -> Result<Vec<HardwareAccel>, MediaProcessorError> {
    let hwaccels = Command::new("ffmpeg")
        .args(["-hide_banner", "-hwaccels"])
        .output()
        .await?;

    if !hwaccels.status.success() {
        return Err(MediaProcessorError::ProcessingFailed);
    }

    let encoders = Command::new("ffmpeg")
        .args(["-hide_banner", "-encoders"])
        .output()
        .await?;

    if !encoders.status.success() {
        return Err(MediaProcessorError::ProcessingFailed);
    }

    let hwaccels_stdout = String::from_utf8_lossy(&hwaccels.stdout);
    let hwaccels_stderr = String::from_utf8_lossy(&hwaccels.stderr);

    let encoders_stdout = String::from_utf8_lossy(&encoders.stdout);
    let encoders_stderr = String::from_utf8_lossy(&encoders.stderr);

    // * FFmpeg can emit capability information through either stream.
    let hwaccels_output = format!(
        "{}\n{}",
        hwaccels_stdout,
        hwaccels_stderr
    );

    let encoders_output = format!(
        "{}\n{}",
        encoders_stdout,
        encoders_stderr
    );

    let has_hwaccel = |name: &str| {
        hwaccels_output
            .lines()
            .any(|line| line.trim() == name)
    };

    let has_encoder = |name: &str| {
        encoders_output
            .lines()
            .any(|line| {
                line.split_whitespace()
                    .nth(1)
                    .is_some_and(|encoder| encoder == name)
            })
    };

    tracing::info!(
        "Available hardware accelerations: {:?}",
        hwaccels_output
    );

    let mut available = Vec::new();

    // * NVENC requires CUDA support and the actual NVENC encoder.
    if has_hwaccel("cuda") && has_encoder("h264_nvenc") {
        available.push(HardwareAccel::Nvenc);
    }

    if has_hwaccel("qsv") && has_encoder("h264_qsv") {
        available.push(HardwareAccel::Qsv);
    }

    if has_encoder("h264_amf") {
        available.push(HardwareAccel::Amf);
    }

    if has_hwaccel("videotoolbox")
        && has_encoder("h264_videotoolbox")
    {
        available.push(HardwareAccel::Videotoolbox);
    }

    if has_hwaccel("vaapi") && has_encoder("h264_vaapi") {
        available.push(HardwareAccel::Vaapi);
    }

    if has_hwaccel("drm") && has_encoder("h264_v4l2m2m") {
        available.push(HardwareAccel::V4l2m2m);
    }

    if has_hwaccel("vdpau") {
        available.push(HardwareAccel::Vdpau);
    }

    if has_hwaccel("opencl") {
        available.push(HardwareAccel::Opencl);
    }

    tracing::info!(
        "Detected hardware acceleration: {:?}",
        available
    );

    Ok(available)
}

pub fn select_best_hardware_accel(available: &[HardwareAccel]) -> HardwareAccel {
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
    pub fn supports_resolution(
        self,
        width: u32,
        height: u32,
    ) -> bool {
        match self {
            // * NVENC H.264 has GPU/driver-dependent minimum dimensions.
            // * Use 145 as the conservative minimum for automatic selection.
            HardwareAccel::Nvenc => {
                width >= 145 && height >= 145
            }

            HardwareAccel::Qsv
            | HardwareAccel::Amf
            | HardwareAccel::V4l2m2m
            | HardwareAccel::Vaapi
            | HardwareAccel::Videotoolbox => {
                true
            }

            HardwareAccel::Cuda
            | HardwareAccel::Vdpau
            | HardwareAccel::Opencl
            | HardwareAccel::Auto
            | HardwareAccel::Software => {
                true
            }
        }
    }

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
            HardwareAccel::Cuda | HardwareAccel::Nvenc => &["-hwaccel", "cuda"],

            HardwareAccel::Qsv => &["-hwaccel", "qsv"],

            HardwareAccel::Vaapi => &["-hwaccel", "vaapi"],

            HardwareAccel::Videotoolbox => &["-hwaccel", "videotoolbox"],

            // * AMD AMF encoding doesn't require an -hwaccel argument.
            HardwareAccel::Amf => &[],

            HardwareAccel::V4l2m2m => &["-hwaccel", "drm"],

            HardwareAccel::Vdpau => &["-hwaccel", "vdpau"],

            HardwareAccel::Opencl | HardwareAccel::Auto | HardwareAccel::Software => &[],
        }
    }
}
