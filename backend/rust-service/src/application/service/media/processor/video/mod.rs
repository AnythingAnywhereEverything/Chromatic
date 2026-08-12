use crate::application::service::{
    errors::MediaServiceError,
    media::{
        processor::types::VideoPostProcessorType, types::file::MultipartFile,
    },
};
pub mod hwaccel;
pub mod video;

pub struct VideoProcessor {
    _gpu_accel: bool, // TODO: implement GPU acceleration for video processing
}

impl VideoProcessor {
    pub fn new(gpu_accel: bool) -> Self {
        Self { _gpu_accel: gpu_accel }
    }

    pub async fn get_thumbnail(
        &self,
        file: &MultipartFile,
        time: u32,
        format: &str,
    ) -> Result<Vec<u8>, MediaServiceError> {
        let output = video::extract_thumbnail(
            &file.get_full_path(),
            format,
            time
        )
        .await?;
        Ok(output)
    }

    pub async fn run_post(
        &self,
        file: &MultipartFile,
        processes: VideoPostProcessorType,
    ) -> Result<(), MediaServiceError> {
        let job_dir_path = file.get_job().get_dir().unwrap();

        let source_path = file.get_job().get_source_file().unwrap();

        if !job_dir_path.exists() || !source_path.exists() {
            return Err(MediaServiceError::ProcessingFailed);
        }

        match processes {
            VideoPostProcessorType::HLS { segment_time } => {
                let output_dir = job_dir_path.join("hls");
                video::process_video_hls(segment_time as f32, output_dir, source_path.clone()).await?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
