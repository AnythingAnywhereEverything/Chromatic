use std::{path::Path, process::Stdio, sync::Arc};

use tokio::process::Command;

use crate::application::service::{
    errors::MediaServiceError,
    media::{
        storage::MediaStorage,
    },
};

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

pub async fn get_available_hardware_accels() -> Result<Vec<String>, MediaServiceError> {
    let output = Command::new("ffmpeg")
        .args(&["-hide_banner", "-hwaccels"])
        .output()
        .await?;

    if !output.status.success() {
        return Err(MediaServiceError::ProcessingFailed);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    lines.next(); // Skip the first line which is "Hardware acceleration methods:"
    let available_accels: Vec<String> = lines.map(|line| line.trim().to_string()).collect();

    Ok(available_accels)
}


pub async fn get_available_video_encoders_for_accel(accel: &str) -> Result<Vec<String>, MediaServiceError> {
    let output = Command::new("ffmpeg")
        .args(&["-hide_banner", "-encoders"])
        .output()
        .await?;

    if !output.status.success() {
        return Err(MediaServiceError::ProcessingFailed);
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


async fn get_video_height(path: &Path) -> Result<i32, MediaServiceError> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=height",
            "-of", "csv=p=0",
        ])
        .arg(path)
        .output()
        .await?;

    let height = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<i32>()
        .map_err(|_| MediaServiceError::ProcessingFailed)?;

    Ok(height)
}

async fn get_video_width(path: &Path) -> Result<i32, MediaServiceError> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=width",
            "-of", "csv=p=0",
        ])
        .arg(path)
        .output()
        .await?;

    let width = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<i32>()
        .map_err(|_| MediaServiceError::ProcessingFailed)?;

    Ok(width)
}

pub async fn get_video_duration(path: &str) -> Result<f32, MediaServiceError> {
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
        .map_err(|_| MediaServiceError::ProcessingFailed)?;

    tracing::debug!("Extracted video duration: {} seconds", duration);

    Ok(duration)
}

async fn get_video_dimensions(path: &Path) -> Result<(i32, i32), MediaServiceError> {
    let height = get_video_height(path).await?;
    let width = get_video_width(path).await?;
    Ok((width, height))
}

async fn get_viable_resolutions(src_width: i32, src_height: i32) -> Vec<Resolution> {
    let shortest_side = src_width.min(src_height) as u32;
    let mut resolutions = Vec::new();
    
    let accepted_resolutions = crate::constant::AVAILABLE_RESOLUTIONS;

    for res in accepted_resolutions {
        if res <= shortest_side {
            let side = if src_width < src_height {
                ResolutionSide::Width
            } else {
                ResolutionSide::Height
            };
            resolutions.push(Resolution { length: res, side });
        }
    }

    // check if res contains the original resolution, if not, add it to the list
    // we add a buffer of 100 to the shortest side to account for any rounding errors or slight differences in resolution
    if !resolutions.iter().any(|r| r.length == shortest_side && r.length + 100 >= shortest_side) {
        let side = if src_width < src_height {
            ResolutionSide::Width
        } else {
            ResolutionSide::Height
        };
        resolutions.push(Resolution { length: shortest_side, side });
    }

    resolutions
}

pub async fn strip_metadata(input_path: String, storage: Arc<dyn MediaStorage>) -> Result<(), MediaServiceError> {
    let temp_source_input = storage.temp_full_path(&input_path)?;

    // create temp file name for output
    let temp_output = storage.temp_full_path(&format!("{}_stripped", input_path))?;

    // Use ffmpeg to strip metadata
    let status = Command::new("ffmpeg")
        .args(&[
            "-i", &temp_source_input.to_string_lossy(),
            "-map_metadata", "-1",
            "-c:v", "copy",
            "-c:a", "copy",
            &temp_output.to_string_lossy(),
        ])
        .status()
        .await?;

    // Replace the original file with the stripped version
    storage.move_file(&temp_output, &temp_source_input).await?;

    if !status.success() {
        return Err(MediaServiceError::MetadataStripFailed);
    }

    Ok(())
}

/// Generates a thumbnail from the video at `input_path` and saves it to `output_path`.
pub async fn generate_video_thumbnail(input_path: &str, codec: &str, second: u32, storage: Arc<dyn MediaStorage>) -> Result<Vec<u8>, MediaServiceError> {
    let temp_source_input = storage.temp_full_path(&input_path)?;
    
    // Use ffmpeg to generate thumbnail
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", &temp_source_input.to_string_lossy(),
            "-ss", second.to_string().as_str(),
            "-vframes", "1",
            "-f", "image2pipe",
            "-vcodec", codec,
            "-",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await?;

    if !output.status.success() {
        return Err(MediaServiceError::ThumbnailGenerationFailed);
    }

    Ok(output.stdout)
}

pub async fn process_video_trim(
    input_path: String,
    start_time: f32,
    end_time: f32,
    output_path: String,
    storage: Arc<dyn MediaStorage>,
) -> Result<(), MediaServiceError> {
    let temp_source_input = storage.temp_full_path(&input_path)?;
    let temp_output = storage.temp_full_path(&output_path)?;

    // Use ffmpeg to trim the video
    let status = Command::new("ffmpeg")
        .args(&[
            "-i", &temp_source_input.to_string_lossy(),
            "-ss", &start_time.to_string(),
            "-to", &end_time.to_string(),
            "-c", "copy",
            &temp_output.to_string_lossy(),
        ])
        .status()
        .await?;

    if !status.success() {
        return Err(MediaServiceError::VideoTrimFailed);
    }

    // Move the trimmed video to the original input path
    storage.move_file(&temp_output, &temp_source_input).await?;

    Ok(())
}


pub async fn process_video_hls(
    segment_duration: u32,
    // job directory path for temporary processing
    job_dir_path: String,
    source_path: String,
    storage: Arc<dyn MediaStorage>,
) -> Result<(), MediaServiceError> {
    // get the full path of the input video from temp storage
    let temp_source_input = storage.temp_full_path(&source_path)?;

    // Create a temporary directory for the HLS job
    let full_job_dir = storage.temp_full_path(&job_dir_path)?;
    tokio::fs::create_dir_all(&full_job_dir).await?;

    // Get the original video dimensions to determine viable resolutions for HLS
    let (width, height) = get_video_dimensions(&temp_source_input).await?;

    // mutatable vectors to hold `ffmpeg` filter and map commands, as well as variant information
    let mut filters = Vec::new();
    let mut var_map = Vec::new();
    let mut index = 0;

    // Determine viable resolutions based on the original video dimensions
    let resolutions = get_viable_resolutions(width, height).await;

    tracing::debug!(
        "Original video dimensions: {}x{}, viable resolutions: {:?}",
        width,
        height,
        resolutions
    );

    // Iterate through the viable resolutions and create HLS variants for each resolution that is less than or equal to the original video height.
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
            // Add the variant stream mapping for the current resolution
            // v means video stream, a means audio stream, and index is the variant index
            var_map.push(format!("v:{},a:{}", index, index));

            index += 1;
        }
    }

    // If no resolutions were added, we should at least add the original resolution as a fallback
    if index == 0 {
        filters.push(format!("[0:v]scale=w=-2:h={}[v0];", height));
        var_map.push("v:0,a:0".to_string());
    }

    // Build the filter_complex string for ffmpeg
    let filter_complex = filters.join(" ");

    // Prepare the ffmpeg command to generate HLS segments and playlists
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-i")
        .arg(&temp_source_input)
        .arg("-filter_complex")
        .arg(&filter_complex);

    for i in 0..index {
        cmd.arg("-map")
            .arg(format!("[v{}]", i))
            .arg("-map")
            .arg("a?");
    }
    
    cmd.args([
        "-f", "hls",
        "-hls_time", &segment_duration.to_string(),
        "-hls_playlist_type", "vod",
        "-hls_segment_filename",
    ])
    .arg(full_job_dir.join("v%v_seg_%03d.ts"))
    .args([
        "-master_pl_name", "master.m3u8",
        "-var_stream_map",
        &var_map.join(" "),
    ])
    .arg(full_job_dir.join("v%v.m3u8"));

    // Execute the ffmpeg command and wait for it to finish
    let output = cmd.output().await?;

    // If the ffmpeg command failed, clean up the temporary job directory and return an error
    if !output.status.success() {
        tracing::error!(
            "ffmpeg failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        return Err(MediaServiceError::ProcessingFailed);
    }

    Ok(())
}