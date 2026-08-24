use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use tokio::process::Command;

use crate::application::service::{errors::media_service::MediaProcessorError, media::model::File};

#[derive(Debug, Clone, Copy)]
pub enum ResolutionSide {
    Width,
    Height,
}

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub length: u32,
    pub side: ResolutionSide,
}

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

pub async fn get_available_hardware_accels() -> Result<Vec<HardwareAccel>, MediaProcessorError> {
    let output = Command::new("ffmpeg")
        .args(&["-hide_banner", "-hwaccels"])
        .output()
        .await?;

    if !output.status.success() {
        return Err(MediaProcessorError::ProcessingFailed);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut accels = Vec::new();

    for line in stdout.lines() {
        let trimmed_line = line.trim();
        match trimmed_line {
            "nvenc" => accels.push(HardwareAccel::Nvenc),
            "qsv" => accels.push(HardwareAccel::Qsv),
            "v4l2m2m" => accels.push(HardwareAccel::V4l2m2m),
            "vaapi" => accels.push(HardwareAccel::Vaapi),
            "vdpau" => accels.push(HardwareAccel::Vdpau),
            "cuda" => accels.push(HardwareAccel::Cuda),
            "opencl" => accels.push(HardwareAccel::Opencl),
            "amf" => accels.push(HardwareAccel::Amf),
            "videotoolbox" => accels.push(HardwareAccel::Videotoolbox),
            _ => {}
        }
    }

    Ok(accels)
}

pub async fn get_available_video_encoders_for_accel(
    accel: &str,
) -> Result<Vec<String>, MediaProcessorError> {
    let output = Command::new("ffmpeg")
        .args(&["-hide_banner", "-encoders"])
        .output()
        .await?;

    if !output.status.success() {
        return Err(MediaProcessorError::ProcessingFailed);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut encoders = Vec::new();
    let accel_lower = accel.to_lowercase(); // Convert accel to lowercase for case-insensitive matching

    // Skip header lines until we find the actual encoder list.
    // Encoders start after the "------" line, or when lines begin with V, A, or S.
    let lines = stdout.lines().skip_while(|line| {
        !line.starts_with("V") && !line.starts_with("A") && !line.starts_with("S")
    });

    for line in lines {
        let trimmed_line = line.trim();
        // We're looking specifically for video encoders, which start with 'V'
        if trimmed_line.starts_with("V") {
            // Split the line to get the encoder name
            // Example line: "V....D hevc_nvenc NVIDIA NVENC hevc encoder (codec hevc)"
            let parts: Vec<&str> = trimmed_line.split_whitespace().collect();

            // Ensure there's at least a flag and an encoder name
            if parts.len() >= 2 {
                let encoder_name = parts[1]; // The encoder name is the second part

                // Check if the encoder name contains the hardware acceleration string
                // e.g., "hevc_nvenc" contains "nvenc"
                if encoder_name.to_lowercase().contains(&accel_lower) {
                    encoders.push(encoder_name.to_string());
                }
            }
        }
    }

    Ok(encoders)
}

async fn has_audio_stream(source_path: &PathBuf) -> Result<bool, MediaProcessorError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "a",
            "-show_entries",
            "stream=index",
            "-of",
            "csv=p=0",
        ])
        .arg(source_path)
        .output()
        .await?;

    if !output.status.success() {
        return Err(MediaProcessorError::ProcessingFailed);
    }

    Ok(!output.stdout.is_empty())
}

async fn get_video_height(path: &Path) -> Result<i32, MediaProcessorError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=height",
            "-of",
            "csv=p=0",
        ])
        .arg(path)
        .output()
        .await?;

    let height = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<i32>()
        .map_err(|_| MediaProcessorError::ProcessingFailed)?;

    Ok(height)
}

async fn get_video_width(path: &Path) -> Result<i32, MediaProcessorError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width",
            "-of",
            "csv=p=0",
        ])
        .arg(path)
        .output()
        .await?;

    let width = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<i32>()
        .map_err(|_| MediaProcessorError::ProcessingFailed)?;

    Ok(width)
}

pub async fn get_video_duration(path: &str) -> Result<f32, MediaProcessorError> {
    tracing::debug!("Getting video duration for path: {}", path);
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output()
        .await?;

    tracing::debug!(
        "ffprobe output for video duration: stdout: {}, stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let duration = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<f32>()
        .map_err(|_| MediaProcessorError::ProcessingFailed)?;

    tracing::debug!("Extracted video duration: {} seconds", duration);

    Ok(duration)
}

async fn get_video_dimensions(path: &Path) -> Result<(i32, i32), MediaProcessorError> {
    let height = get_video_height(path).await?;
    let width = get_video_width(path).await?;
    Ok((width, height))
}

async fn get_viable_resolutions(src_width: i32, src_height: i32) -> Vec<Resolution> {
    let mut resolutions = Vec::new();

    let is_vertical = src_height > src_width;

    let quality_dimension = if is_vertical {
        src_width as u32
    } else {
        src_height as u32
    };

    let accepted_resolutions = crate::constant::AVAILABLE_RESOLUTIONS;

    for res in accepted_resolutions {
        if res <= quality_dimension {
            resolutions.push(Resolution {
                length: res,
                side: if is_vertical {
                    ResolutionSide::Width
                } else {
                    ResolutionSide::Height
                },
            });
        }
    }

    tracing::debug!(
        "Source dimensions: {}x{}, quality dimension: {}, viable resolutions: {:?}",
        src_width,
        src_height,
        quality_dimension,
        resolutions
    );

    // * Preserve the source's native quality dimension when it isn't
    // * already represented by AVAILABLE_RESOLUTIONS.
    if !resolutions.iter().any(|r| r.length == quality_dimension) {
        resolutions.push(Resolution {
            length: quality_dimension,
            side: if is_vertical {
                ResolutionSide::Width
            } else {
                ResolutionSide::Height
            },
        });
    }

    resolutions
}

pub async fn strip_metadata(file: File) -> Result<(), MediaProcessorError> {
    let temp_source_input = file.file_full_path(); // Assuming the input_path is already a temporary path

    let temp_dir = file.file_full_directory(); // Get the directory of the input file
    // create temp file name for output
    let temp_output = temp_dir.join(format!("{}_stripped", file.file_name()));

    // Use ffmpeg to strip metadata
    let status = Command::new("ffmpeg")
        .args(&[
            "-i",
            &temp_source_input.to_string_lossy(),
            "-map_metadata",
            "-1",
            "-c:v",
            "copy",
            "-c:a",
            "copy",
            &temp_output.to_string_lossy(),
        ])
        .status()
        .await?;

    // Replace the original file with the stripped version
    tokio::fs::rename(&temp_output, &temp_source_input).await?;

    if !status.success() {
        return Err(MediaProcessorError::MetadataStripFailed);
    }

    Ok(())
}

/// Generates a thumbnail from the video at `input_path` and saves it to `output_path`.
pub async fn extract_thumbnail(
    input_path: &str,
    codec: &str,
    second: u32,
) -> Result<Vec<u8>, MediaProcessorError> {
    // Use ffmpeg to generate thumbnail

    tracing::debug!(
        "Extracting thumbnail from video: {}, codec: {}, second: {}",
        input_path,
        codec,
        second
    );

    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            input_path,
            "-ss",
            second.to_string().as_str(),
            "-vframes",
            "1",
            "-f",
            "image2pipe",
            "-vcodec",
            codec,
            "-",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await?;

    if !output.status.success() {
        return Err(MediaProcessorError::ThumbnailGenerationFailed);
    }

    Ok(output.stdout)
}

pub async fn process_video_trim(
    start_time: f32,
    end_time: f32,
    file: &File,
) -> Result<(), MediaProcessorError> {
    let temp_source_input = file.file_full_path();
    let temp_output = file
        .file_full_directory()
        .join(format!("{}_trimmed", file.file_name()));

    // Use ffmpeg to trim the video
    let status = Command::new("ffmpeg")
        .args(&[
            "-i",
            &temp_source_input.to_string_lossy(),
            "-ss",
            &start_time.to_string(),
            "-to",
            &end_time.to_string(),
            "-c",
            "copy",
            &temp_output.to_string_lossy(),
        ])
        .status()
        .await?;

    if !status.success() {
        return Err(MediaProcessorError::VideoTrimFailed);
    }

    // Move the trimmed video to the original input path
    tokio::fs::rename(&temp_output, &temp_source_input).await?;

    Ok(())
}

pub async fn process_video_hls(
    segment_duration: f32,

    // job directory path for temporary processing
    job_dir_path: PathBuf,
    source_path: PathBuf,
) -> Result<(), MediaProcessorError> {
    tokio::fs::create_dir_all(&job_dir_path).await?;

    // Get the original video dimensions to determine viable resolutions for HLS
    let (width, height) = get_video_dimensions(&source_path).await?;

    // Check whether the source contains an audio stream.
    let has_audio = has_audio_stream(&source_path).await?;

    tracing::debug!("HLS source: {}x{}, audio: {}", width, height, has_audio);

    // Mutable vectors to hold ffmpeg filter and variant information
    let mut filters = Vec::new();
    let mut var_map = Vec::new();

    // Determine viable resolutions based on the original video dimensions
    let resolutions = get_viable_resolutions(width, height).await;

    tracing::debug!(
        "Original video dimensions: {}x{}, viable resolutions: {:?}",
        width,
        height,
        resolutions
    );

    let mut index = 0;

    // Create one video variant for every viable resolution.
    for &res in &resolutions {
        if res.length <= height as u32 {
            match res.side {
                ResolutionSide::Width => {
                    filters.push(format!("[0:v]scale=w={}:h=-2[v{}];", res.length, index));
                }

                ResolutionSide::Height => {
                    filters.push(format!("[0:v]scale=w=-2:h={}[v{}];", res.length, index));
                }
            }

            // * When audio exists, each variant gets its corresponding audio stream.
            // * Without audio, the variant contains video only.
            if has_audio {
                var_map.push(format!("v:{},a:{}", index, index));
            } else {
                var_map.push(format!("v:{}", index));
            }

            index += 1;
        }
    }

    // If no resolutions were added, use the original resolution as a fallback.
    if index == 0 {
        filters.push(format!("[0:v]scale=w=-2:h={}[v0];", height));

        if has_audio {
            var_map.push("v:0,a:0".to_string());
        } else {
            var_map.push("v:0".to_string());
        }

        index = 1;
    }

    let filter_complex = filters.join(" ");

    let mut cmd = Command::new("ffmpeg");

    cmd.arg("-i")
        .arg(&source_path)
        .arg("-filter_complex")
        .arg(&filter_complex);

    // Map the generated video streams.
    for i in 0..index {
        cmd.arg("-map").arg(format!("[v{}]", i));

        // * Only map audio when the source actually has audio.
        if has_audio {
            cmd.arg("-map").arg("0:a:0");
        }
    }

    cmd.args([
        "-c:v",
        "libx264",
        "-c:a",
        "aac",
        "-f",
        "hls",
        "-hls_time",
        &segment_duration.to_string(),
        "-hls_playlist_type",
        "vod",
        "-hls_segment_filename",
    ])
    .arg(job_dir_path.join("v%v_seg_%03d.ts"))
    .args([
        "-master_pl_name",
        "master.m3u8",
        "-var_stream_map",
        &var_map.join(" "),
    ])
    .arg(job_dir_path.join("v%v.m3u8"));

    let output = cmd.output().await?;

    if !output.status.success() {
        tracing::error!(
            "ffmpeg failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        return Err(MediaProcessorError::ProcessingFailed);
    }

    Ok(())
}
