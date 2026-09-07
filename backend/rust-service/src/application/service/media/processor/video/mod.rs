use std::sync::Arc;

use crate::application::service::{
    errors::media_service::MediaProcessorError, media::{
        model::File, processor::types::VideoPostProcessorType, storage::PersistentStore,
    },
};
pub mod hwaccel;
pub mod video;

pub struct VideoProcessor {
    gpu_accel: bool, // TODO: implement GPU acceleration for video processing
}

impl VideoProcessor {
    pub fn new(gpu_accel: bool) -> Self {
        Self { gpu_accel: gpu_accel }
    }

    pub async fn get_thumbnail(
        &self,
        file: &File,
        time: u32,
        format: &str,
    ) -> Result<Vec<u8>, MediaProcessorError> {
        let output = video::extract_thumbnail(
            &file.file_full_path().to_string_lossy(),
            format,
            time
        )
        .await?;
        Ok(output)
    }

    pub async fn run_post(
        &self,
        file: &File,
        
        target_path: &String,
        persistent: &Arc<dyn PersistentStore>,

        processes: VideoPostProcessorType,
        pool: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<(), MediaProcessorError> {
        let job_dir_path = file.file_full_directory();

        let source_path = file.file_full_path();

        tracing::info!(
            "Running post processing for video file: {} with processes: {:?}",
            source_path.display(),
            processes
        );

        if !job_dir_path.exists() || !source_path.exists() {
            return Err(MediaProcessorError::ProcessingFailed);
        }

        match processes {
            VideoPostProcessorType::HLS { segment_time } => {
                let output_dir: std::path::PathBuf = job_dir_path.join("hls");
                video::process_video_hls(file.id().unwrap(),segment_time as f32, output_dir, source_path.clone(), self.gpu_accel, target_path, persistent, pool).await.map_err(|e| {
                    tracing::error!("HLS processing failed: {:?}", e);
                    e
                })?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
