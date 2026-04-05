use std::{path::Path, sync::Arc};

use tokio::process::Command;

use crate::application::service::{
    errors::MediaServiceError,
    media::{
        storage::MediaStorage,
        types::{VideoManifest, VideoVariant},
    },
};

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

pub async fn process_video_hls(
    input_path: String,
    output_path: String,
    storage: Arc<dyn MediaStorage>,
) -> Result<VideoManifest, MediaServiceError> {
    let temp_source_input = storage.temp_full_path(&input_path)?;

    let temp_job_rel = storage.new_temp_relative_path("video_hls_job")?;
    let temp_job_dir = storage.temp_full_path(&temp_job_rel)?;

    let input_ext = std::path::Path::new(&input_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("mp4");

    let temp_input = temp_job_dir.join(format!("source.{}", input_ext));
    let temp_output_dir = temp_job_dir.join("hls");

    tokio::fs::create_dir_all(&temp_job_dir).await?;
    tokio::fs::create_dir_all(&temp_output_dir).await?;

    tokio::fs::copy(&temp_source_input, &temp_input).await?;

    let height = get_video_height(&temp_input).await?;

    let mut filters = Vec::new();
    let mut maps = Vec::new();
    let mut var_map = Vec::new();
    let mut variants = Vec::new();

    let mut index = 0;
    let resolutions = [1080, 720, 480];

    for &h in &resolutions {
        if height >= h {
            filters.push(format!("[0:v]scale=w=-2:h={}[v{}];", h, index));
            maps.push(format!("-map [v{}]", index));
            maps.push("-map a?".to_string());
            var_map.push(format!("v:{},a:{}", index, index));

            variants.push(VideoVariant {
                resolution: h,
                playlist: format!("{}/v{}.m3u8", output_path, index),
            });

            index += 1;
        }
    }

    if index == 0 {
        filters.push(format!("[0:v]scale=w=-2:h={}[v0];", height));
        maps.push("-map [v0]".to_string());
        maps.push("-map a?".to_string());
        var_map.push("v:0,a:0".to_string());

        variants.push(VideoVariant {
            resolution: height,
            playlist: format!("{}/v0.m3u8", output_path),
        });
    }

    let filter_complex = filters.join(" ");

    let mut cmd = Command::new("ffmpeg");

    cmd.arg("-i")
        .arg(&temp_input)
        .arg("-filter_complex")
        .arg(&filter_complex);

    for m in maps {
        cmd.arg(m);
    }

    cmd.args([
        "-f", "hls",
        "-hls_time", "6",
        "-hls_playlist_type", "vod",
        "-hls_segment_filename",
    ])
    .arg(temp_output_dir.join("v%v_seg_%03d.ts"))
    .args([
        "-master_pl_name", "master.m3u8",
        "-var_stream_map",
        &var_map.join(" "),
    ])
    .arg(temp_output_dir.join("v%v.m3u8"))
    .stdout(std::process::Stdio::null())
    .stderr(std::process::Stdio::null());

    let status = cmd.spawn()?.wait().await?;

    if !status.success() {
        let _ = tokio::fs::remove_dir_all(&temp_job_dir).await;
        return Err(MediaServiceError::ProcessingFailed);
    }

    let mut entries = tokio::fs::read_dir(&temp_output_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        let data = tokio::fs::read(&file_path).await?;
        let filename = file_path
            .file_name()
            .ok_or(MediaServiceError::ProcessingFailed)?
            .to_string_lossy()
            .to_string();

        let relative = format!("{}/{}", output_path, filename);
        storage.save(&relative, &data).await?;
    }

    let _ = tokio::fs::remove_dir_all(&temp_job_dir).await;

    Ok(VideoManifest {
        master: format!("{}/master.m3u8", output_path),
        variants,
    })
}