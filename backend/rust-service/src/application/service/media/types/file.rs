
use std::{io::Read, path::PathBuf, sync::Arc};
use serde::{Serialize, Deserialize};

use rs_vips::VipsImage;

use crate::application::service::{errors::MediaServiceError, media::{storage::{MediaStorage, MediaStorageContainer}, types::media_options::MediaCategory, utils::get_mime_and_extension}};

#[derive(Serialize, Deserialize)]
pub struct MultipartFile {
    #[serde(skip)]
    storage_container: MediaStorageContainer,

    id: i64,
    path: PathBuf,
    relative_path: String,
    name: String, // can be overridden by user, if not provided, will be generated from the path
    extension: String,
    category: MediaCategory,
    thumbhash: Option<String>,
    destination: String,
    mime: String,
    size: usize,
    job : MultipartJob,
    meta_data: Option<MultipartMeta>,
}

impl std::fmt::Debug for MultipartFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultipartFile")
            .field("id", &self.id)
            .field("path", &self.path)
            .field("relative_path", &self.relative_path)
            .field("name", &self.name)
            .field("extension", &self.extension)
            .field("category", &self.category)
            .field("thumbhash", &self.thumbhash)
            .field("destination", &self.destination)
            .field("mime", &self.mime)
            .field("size", &self.size)
            .field("job", &self.job)
            .field("meta_data", &self.meta_data)
            .finish()
    }
}

impl Clone for MultipartFile {
    fn clone(&self) -> Self {
        Self {
            storage_container: MediaStorageContainer { storage: Arc::clone(&self.storage_container.storage) },
            id: self.id,
            path: self.path.clone(),
            relative_path: self.relative_path.clone(),
            name: self.name.clone(),
            extension: self.extension.clone(),
            category: self.category.clone(),
            thumbhash: self.thumbhash.clone(),
            destination: self.destination.clone(),
            mime: self.mime.clone(),
            size: self.size,
            job: self.job.clone(),
            meta_data: self.meta_data.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultipartJob {
    dir: Option<PathBuf>,
    relative_dir: Option<String>,
    source_file: Option<PathBuf>,
    source_relative_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultipartMeta {
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}

impl MultipartFile {
    pub fn new(storage: Arc<dyn MediaStorage>, path: PathBuf, relative_path: String, name: String, extension: String, category: MediaCategory, size: usize, mime: String) -> Self {
        Self {
            storage_container: MediaStorageContainer { storage },
            id: 0, // will be set when saved to the database
            path,
            relative_path,
            name: name,
            extension,
            category,
            thumbhash: None,
            size,
            mime,
            destination: String::new(),
            job: MultipartJob {
                dir: None,
                relative_dir: None,
                source_file: None,
                source_relative_path: None,
            },
            meta_data: None,
        }
    }

    pub fn get_extra_meta(&self) -> Result<MultipartMeta, MediaServiceError> {
        match self.category {
            MediaCategory::Image =>  {
                let image = VipsImage::new_from_file(self.path.to_string_lossy().to_string())?;
                return Ok(MultipartMeta {
                    width: Some(image.get_width()),
                    height: Some(image.get_height()),
                    duration: None,
                })
            }
            MediaCategory::Video => {
                let output = std::process::Command::new("ffprobe")
                    .args(&[
                        "-v",
                        "error",
                        "-select_streams",
                        "v:0",
                        "-show_entries",
                        "stream=width,height,duration",
                        "-of",
                        "default=noprint_wrappers=1:nokey=1",
                        self.path.to_str().unwrap(),
                    ])
                    .output()
                    .expect("Failed to execute ffprobe");

                let output_str = String::from_utf8_lossy(&output.stdout);
                let mut lines = output_str.lines();

                let width = lines.next().and_then(|w| w.parse::<i32>().ok());
                let height = lines.next().and_then(|h| h.parse::<i32>().ok());
                let duration = lines.next().and_then(|d| d.parse::<f32>().ok());

                return Ok(MultipartMeta {
                    width,
                    height,
                    duration,
                });
            }
            _ => return Ok(MultipartMeta {
                width: None,
                height: None,
                duration: None,
            }),
        }
    }

    pub fn get_relative_destination(&self) -> String {
        format!("{}/{}", self.destination, self.get_full_name())
    }

    pub fn is_existing(&self, storage: &Arc<dyn MediaStorage>) -> bool {
        storage.temp_full_path(&self.relative_path).exists()
    }

    

    pub async fn del_replace(&mut self, storage: &Arc<dyn MediaStorage>, new_path: PathBuf, new_relative_path: String) -> Result<(), MediaServiceError> {
        //delete old file
        storage.delete_temp(&self.relative_path).await;
        self.path = new_path;
        self.relative_path = new_relative_path;
        Ok(())
    }

    pub async fn replace(&mut self, new_path: PathBuf, new_relative_path: String) -> Result<(), MediaServiceError> {
        self.path = new_path;
        self.relative_path = new_relative_path;
        Ok(())
    }

    pub fn rename_file(&mut self, storage: &Arc<dyn MediaStorage>, new_name: String) {
        let new_relative_path = self.relative_path.replace(
            &self.get_full_name(),
            &format!("{}.{}", new_name, self.extension),
        );

        std::fs::rename(
            storage.temp_full_path(&self.relative_path),
            storage.temp_full_path(&new_relative_path),
        )
        .unwrap();

        self.name = new_name;
        self.relative_path = new_relative_path;
        self.path = storage.temp_full_path(&self.relative_path);
    }

    pub fn get_hash(&self) -> Result<String, std::io::Error> {
        use sha2::{Digest, Sha256};
        use std::io::Read;

        let mut file = std::fs::File::open(&self.path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 64 * 1024];

        loop {
            let read = file.read(&mut buffer)?;

            if read == 0 {
                break;
            }

            hasher.update(&buffer[..read]);
        }

        Ok(hex::encode(hasher.finalize()))
    }

    pub fn get_full_path(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_extension(&mut self, extension: String) {
        self.extension = extension;
    }
}

// * ---------------------------------
// *  Implementing Others
// * ---------------------------------

impl MultipartFile {
    pub fn build_relative_file_destination(&self) -> String {
        format!("{}/{}", self.destination, self.get_full_name())
    }

    pub fn revalidate(&mut self, storage: &Arc<dyn MediaStorage>) -> Result<(), MediaServiceError> {
        let full_path = storage.temp_full_path(&self.relative_path);
        
        // read 8kb file as bytes
        let mut file = std::fs::File::open(&full_path)?;
        let mut buffer = [0u8; 8 * 1024];
        let read_bytes = file.read(&mut buffer)?;

        let (mime, extension) = get_mime_and_extension(&buffer[..read_bytes])?;

        let file_size = std::fs::metadata(&full_path)?.len() as usize;

        self.mime = mime;
        self.extension = extension;
        self.size = file_size;
        Ok(())
    }

    pub fn generate_hash(&self) -> Result<String, MediaServiceError> {
        use sha2::{Digest, Sha256};
        use std::io::Read;

        let mut file = std::fs::File::open(&self.path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 64 * 1024];

        loop {
            let read = file.read(&mut buffer)?;

            if read == 0 {
                break;
            }

            hasher.update(&buffer[..read]);
        }

        Ok(hex::encode(hasher.finalize()))
    }
}


// * File managements

impl MultipartFile {

    pub fn move_to_path(&mut self, dst_path: &PathBuf, dst_relative_path: String) -> Result<(), MediaServiceError> {
        std::fs::rename(&self.path, dst_path)?;
        self.path = dst_path.clone();
        self.relative_path = dst_relative_path;
        Ok(())
    }

    pub fn copy_to_path(&self, dst_path: &PathBuf, dst_relative_path: String) -> Result<MultipartFile, MediaServiceError> {
        std::fs::copy(&self.path, dst_path)?;
        let new_file = MultipartFile::new(
            self.storage_container.storage.clone(),
            dst_path.clone(),
            dst_relative_path,
            self.name.clone(),
            self.extension.clone(),
            self.category.clone(),
            self.size,
            self.mime.clone(),
        );
        Ok(new_file)
    }

    pub fn rename(&mut self, new_name: &str) -> Result<(), MediaServiceError> {
        let new_relative_path = self.relative_path.replace(
            &self.get_full_name(),
            &format!("{}.{}", new_name, self.extension),
        );

        std::fs::rename(
            &self.path,
            self.path.with_file_name(format!("{}.{}", new_name, self.extension)),
        )?;

        self.name = new_name.to_string();
        self.relative_path = new_relative_path;
        self.path = self.path.with_file_name(format!("{}.{}", new_name, self.extension));
        Ok(())
    }

    pub fn rename_extension(&mut self, new_extension: &str) -> Result<(), MediaServiceError> {
        let new_relative_path = self.relative_path.replace(
            &self.get_full_name(),
            &format!("{}.{}", self.name, new_extension),
        );

        std::fs::rename(
            &self.path,
            self.path.with_extension(new_extension),
        )?;

        self.extension = new_extension.to_string();
        self.relative_path = new_relative_path;
        self.path = self.path.with_extension(new_extension);
        Ok(())
    }

    pub fn rename_full(&mut self, new_name_full: &str) -> Result<(), MediaServiceError> {
        let parts: Vec<&str> = new_name_full.rsplitn(2, '.').collect();
        let (new_name, new_extension) = if parts.len() == 2 {
            (parts[1], parts[0])
        } else {
            (new_name_full, "")
        };

        let new_relative_path = self.relative_path.replace(
            &self.get_full_name(),
            &format!("{}.{}", new_name, new_extension),
        );

        std::fs::rename(
            &self.path,
            self.path.with_file_name(format!("{}.{}", new_name, new_extension)),
        )?;

        self.name = new_name.to_string();
        self.extension = new_extension.to_string();
        self.relative_path = new_relative_path;
        self.path = self.path.with_file_name(format!("{}.{}", new_name, new_extension));
        Ok(())
    }
}

// * ---------------------------------
// *  Implementing SETTER
// * ---------------------------------

impl MultipartFile {
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    pub fn set_file_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_file_extension(&mut self, extension: String) {
        self.extension = extension;
    }

    pub fn set_full_name_from_str(&mut self, full_name: &str) {
        let parts: Vec<&str> = full_name.rsplitn(2, '.').collect();
        if parts.len() == 2 {
            self.name = parts[1].to_string();
            self.extension = parts[0].to_string();
        } else {
            self.name = full_name.to_string();
            self.extension = String::new();
        }
    }

    pub fn set_thumbhash(&mut self, thumbhash: String) {
        self.thumbhash = Some(thumbhash);
    }

    pub fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    pub fn set_mime(&mut self, mime: String) {
        self.mime = mime;
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn set_job_dir(&mut self, job_dir: &PathBuf, job_relative: &str) {
        self.job.relative_dir = Some(job_relative.to_string());
        self.job.dir = Some(job_dir.clone());
    }

    pub fn set_job_source_file(&mut self, source_file: &PathBuf, source_relative_path: &str) {
        self.job.source_file = Some(source_file.clone());
        self.job.source_relative_path = Some(source_relative_path.to_string());
    }
}

// * ---------------------------------
// *  Implementing GETTER
// * ---------------------------------

impl MultipartMeta {
    pub fn get_width(&self) -> Option<i32> {
        self.width
    }

    pub fn get_height(&self) -> Option<i32> {
        self.height
    }

    pub fn get_duration(&self) -> Option<f32> {
        self.duration
    }
}

impl MultipartJob {
    pub fn get_dir(&self) -> Option<&PathBuf> {
        self.dir.as_ref()
    }

    pub fn get_relative_dir(&self) -> String {
        self.relative_dir.as_ref().cloned().unwrap_or_default()
    }

    pub fn get_source_file(&self) -> Option<&PathBuf> {
        self.source_file.as_ref()
    }

    pub fn get_source_relative_path(&self) -> String {
        self.source_relative_path.as_ref().cloned().unwrap_or_default()
    }
}

impl MultipartFile {
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_path_str(&self) -> &str {
        self.path.to_str().unwrap_or_default()
    }

    pub fn get_relative_path(&self) -> &str {
        &self.relative_path
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    
    pub fn get_extension(&self) -> &str {
        &self.extension
    }

    pub fn get_full_name(&self) -> String {
        format!("{}.{}", self.name, self.extension)
    }

    pub fn get_mime(&self) -> &str {
        &self.mime
    }
    

    pub fn get_category(&self) -> &MediaCategory {
        &self.category
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_thumbhash(&self) -> Option<&String> {
        self.thumbhash.as_ref()
    }

    pub fn get_destination(&self) -> &str {
        &self.destination
    }

    pub fn get_meta_data(&self) -> Option<&MultipartMeta> {
        self.meta_data.as_ref()
    }

    pub fn get_job(&self) -> &MultipartJob {
        &self.job
    }

    pub fn get(&self) -> &Self {
        self
    }
}