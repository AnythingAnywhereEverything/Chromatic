use std::{
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
};

use tokio::process::Command;

use crate::application::{
    repository::media::{
        self,
        row::{MediaHls, MediaHlsPlaylist},
    },
    service::{
        errors::media_service::MediaProcessorError,
        media::{
            model::File,
            processor::video::hwaccel::{self, HardwareAccel},
            storage::PersistentStore,
        },
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

fn get_segment_duration(base_duration: f32, resolution: &Resolution) -> f32 {
    // * Higher resolutions get shorter segments to keep individual fragments smaller.
    match resolution.length {
        2160.. => base_duration.min(2.0),
        1440.. => base_duration.min(3.0),
        1080.. => base_duration.min(4.0),
        720.. => base_duration.min(5.0),
        _ => base_duration,
    }
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

pub async fn get_video_duration(path: &Path) -> Result<f32, MediaProcessorError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path.to_str().unwrap(),
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
    id: i64,

    segment_duration: f32,
    job_dir_path: PathBuf,
    source_path: PathBuf,
    gpu_accel: bool,

    target_path: &String,
    persistent: &Arc<dyn PersistentStore>,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), MediaProcessorError> {
    tokio::fs::create_dir_all(&job_dir_path).await?;

    let (width, height) = get_video_dimensions(&source_path).await?;
    let resolutions = get_viable_resolutions(width, height).await;
    let has_audio = has_audio_stream(&source_path).await?;

    tracing::debug!(
        "Original video dimensions: {}x{}, viable resolutions: {:?}",
        width,
        height,
        resolutions
    );

    let encoder = if gpu_accel {
        let available = hwaccel::get_available_hwaccels().await?;
        let selected = hwaccel::select_best_hardware_accel(&available);

        selected
    } else {
        HardwareAccel::Software
    };

    let mut variants = Vec::new();

    for (index, res) in resolutions.iter().enumerate() {
        let resolution_relative_path = format!("{}/hls/{}", target_path, res.length);
        let playlist_name = format!("{}/v{}.m3u8", res.length, index);

        let job_resolution_dir = job_dir_path.join(res.length.to_string());
        let segment_pattern = job_dir_path.join(format!("{}/v{}_seg_%03d.ts", res.length, index));
        let playlist_path = job_dir_path.join(&playlist_name);

        let variant_segment_duration = get_segment_duration(segment_duration, res);

        tokio::fs::create_dir_all(&job_resolution_dir).await?;

        let (variant_width, variant_height) = match res.side {
            ResolutionSide::Width => {
                let variant_width = res.length;
                let variant_height =
                    ((height as f64 / width as f64) * variant_width as f64).round() as u32;

                // * Keep dimensions even for H.264.
                (variant_width, variant_height & !1)
            }

            ResolutionSide::Height => {
                let variant_height = res.length;
                let variant_width =
                    ((width as f64 / height as f64) * variant_height as f64).round() as u32;

                // * Keep dimensions even for H.264.
                (variant_width & !1, variant_height)
            }
        };

        let scale = format!("scale=w={}:h={}", variant_width, variant_height);

        // * Hardware encoding is selected per variant because some GPUs
        // * cannot encode very small resolutions.
        let variant_encoder = if encoder.supports_resolution(variant_width, variant_height) {
            encoder
        } else {
            HardwareAccel::Software
        };

        tracing::info!(
            "Processing HLS variant {}: {}x{} using {:?}",
            index,
            variant_width,
            variant_height,
            variant_encoder
        );

        let mut cmd = Command::new("ffmpeg");

        // * Hardware acceleration arguments must come before the input.
        cmd.args(variant_encoder.hwaccel_args());

        cmd.arg("-i")
            .arg(&source_path)
            .arg("-vf")
            .arg(&scale)
            .arg("-map")
            .arg("0:v:0");

        if has_audio {
            cmd.arg("-map").arg("0:a:0");
        }

        cmd.arg("-c:v");

        if let Some(video_encoder) = variant_encoder.encoder() {
            cmd.arg(video_encoder);
        } else {
            cmd.arg("libx264");
        }

        for arg in variant_encoder.quality_args() {
            cmd.arg(arg);
        }

        if has_audio {
            cmd.arg("-c:a").arg("aac");
        }

        cmd.arg("-f")
            .arg("hls")
            .arg("-hls_time")
            .arg(variant_segment_duration.to_string())
            .arg("-hls_playlist_type")
            .arg("vod")
            .arg("-hls_segment_filename")
            .arg(&segment_pattern)
            .arg(&playlist_path);

        let output = cmd.output().await?;

        if !output.status.success() {
            tracing::error!(
                "ffmpeg failed for {}{}:\n{}",
                res.length,
                match res.side {
                    ResolutionSide::Width => "w",
                    ResolutionSide::Height => "h",
                },
                String::from_utf8_lossy(&output.stderr)
            );

            return Err(MediaProcessorError::ProcessingFailed);
        }

        // move the generated HLS variant files to the target path

        let metadata = probe_hls_variant(&playlist_path).await?;

        tracing::debug!("HLS variant {} metadata: {:?}", index, metadata);

        let mut tx = pool.begin().await?;

        let playlist = MediaHlsPlaylist {
            media_id: id,
            resolution: res.length.to_string(),
            segment_count: metadata.segment_count,
            segment_duration: metadata.segment_duration,
            playlist_storage_key: format!("{}/hls/{}", target_path, playlist_name),
            ..Default::default()
        };

        media::create::media_hls_playlist_create(&mut tx, &playlist).await?;

        tx.commit().await?;

        persistent
            .put_dir(
                &job_dir_path.join(res.length.to_string()),
                &resolution_relative_path,
            )
            .await?;
        variants.push((index, playlist_name, metadata));
    }

    // * All encoders have finished and every variant has been probed.
    let mut master = String::from("#EXTM3U\n#EXT-X-VERSION:3\n");

    for (_index, playlist_name, metadata) in &variants {
        master.push_str(&format!(
            "#EXT-X-STREAM-INF:BANDWIDTH={},AVERAGE-BANDWIDTH={},RESOLUTION={}x{},CODECS=\"{}\"\n",
            metadata.bandwidth,
            metadata.average_bandwidth,
            metadata.width,
            metadata.height,
            metadata.codecs,
        ));

        master.push_str(&format!("{}\n\n", playlist_name));
    }

    let mut tx = pool.begin().await?;

    media::create::media_hls_create(
        &mut tx,
        &MediaHls {
            media_id: id,
            master_playlist: format!("{}/hls/master.m3u8", target_path),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        },
    )
    .await?;

    tx.commit().await?;

    tokio::fs::write(job_dir_path.join("master.m3u8"), master).await?;

    Ok(())
}

#[derive(Debug, Clone)]
struct HlsVariantMetadata {
    width: u32,
    height: u32,
    bandwidth: u64,
    average_bandwidth: u64,
    codecs: String,
    segment_count: i32,
    segment_duration: f32,
}

async fn probe_hls_variant(
    playlist_path: &PathBuf,
) -> Result<HlsVariantMetadata, MediaProcessorError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height,codec_name,profile,level",
            "-of",
            "default=noprint_wrappers=1",
        ])
        .arg(playlist_path)
        .output()
        .await?;

    if !output.status.success() {
        tracing::error!(
            "ffprobe failed for {}:\n{}",
            playlist_path.display(),
            String::from_utf8_lossy(&output.stderr)
        );

        return Err(MediaProcessorError::ProcessingFailed);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut width = None;
    let mut height = None;
    let mut codec_name = None;
    let mut profile = None;
    let mut level = None;

    for line in stdout.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        match key {
            "width" => width = value.parse::<u32>().ok(),
            "height" => height = value.parse::<u32>().ok(),
            "codec_name" => codec_name = Some(value.to_string()),
            "profile" => profile = Some(value.to_string()),
            "level" => level = value.parse::<u32>().ok(),
            _ => {}
        }
    }

    let width = width.ok_or(MediaProcessorError::ProcessingFailed)?;
    let height = height.ok_or(MediaProcessorError::ProcessingFailed)?;
    let codec_name = codec_name.ok_or(MediaProcessorError::ProcessingFailed)?;

    let video_codec = match codec_name.as_str() {
        "h264" => {
            let profile_idc = match profile.as_deref() {
                Some("Baseline") => "42",
                Some("Main") => "4D",
                Some("High") => "64",
                Some("High 10") => "6E",
                Some("High 4:2:2") => "7A",
                Some("High 4:4:4 Predictive") => "F4",
                _ => "64",
            };

            let level = level.unwrap_or(31);
            let level_hex = format!("{:02X}", level);

            format!("avc1.{}00{}", profile_idc, level_hex)
        }

        "hevc" => "hvc1".to_string(),
        "av1" => "av01".to_string(),
        "vp9" => "vp09".to_string(),
        other => other.to_string(),
    };

    // * Read the generated HLS playlist and calculate actual segment metadata.
    let playlist = tokio::fs::read_to_string(playlist_path).await?;

    let mut total_bytes = 0u64;
    let mut total_duration = 0f64;
    let mut peak_bandwidth = 0u64;
    let mut segment_count = 0i32;

    let mut current_duration = None;

    let playlist_dir = playlist_path
        .parent()
        .ok_or(MediaProcessorError::ProcessingFailed)?;

    for line in playlist.lines() {
        if let Some(duration) = line.strip_prefix("#EXTINF:") {
            let duration = duration
                .split(',')
                .next()
                .and_then(|value| value.parse::<f64>().ok());

            current_duration = duration;
            continue;
        }

        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let segment_path = playlist_dir.join(line.trim());
        let metadata = tokio::fs::metadata(&segment_path).await?;
        let bytes = metadata.len();

        if let Some(duration) = current_duration.take() {
            if duration > 0.0 {
                let bandwidth = ((bytes as f64 * 8.0) / duration) as u64;

                peak_bandwidth = peak_bandwidth.max(bandwidth);
                total_bytes += bytes;
                total_duration += duration;
                segment_count += 1;
            }
        }
    }

    let average_bandwidth = if total_duration > 0.0 {
        ((total_bytes as f64 * 8.0) / total_duration) as u64
    } else {
        peak_bandwidth
    };

    let segment_duration = if segment_count > 0 {
        (total_duration / segment_count as f64) as f32
    } else {
        0.0
    };

    // * HLS BANDWIDTH represents the peak segment bitrate.
    let bandwidth = peak_bandwidth.max(average_bandwidth);

    // * Add AAC-LC when the generated variant contains audio.
    let codecs = if has_audio_stream(playlist_path).await? {
        format!("{},mp4a.40.2", video_codec)
    } else {
        video_codec
    };

    Ok(HlsVariantMetadata {
        width,
        height,
        bandwidth,
        average_bandwidth,
        codecs,
        segment_count,
        segment_duration,
    })
}