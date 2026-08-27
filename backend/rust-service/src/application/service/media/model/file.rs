use std::path::PathBuf;

use crate::application::service::{errors::media_service::FileError, media::inspector::MediaKind};
#[repr(u32)]
pub enum Flags {
    None = 0,
    // Fist bit indicates whether the file is animated (e.g., GIF, APNG).
    // bit as numerial = 1
    IsAnimated = 1 << 0,

    // Second bit indicates whether the file is an HLS video.
    // bit as numerial = 2
    IsHLS = 1 << 1,

    // For video file
    // bit as numerial = 4
    HasThumbnail = 1 << 2,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct File {
    id: Option<i64>,

    // Original file name and content type as provided by the user.
    original_name: Option<String>,
    original_content_type: Option<String>,

    // Detected content type.
    content_type: String,
    detected_extension: String,

    // Placeholder for the file such as thumbhash.
    placeholder: Option<String>,

    // Flags for the file, represented as a bitmask.
    flags: u32,

    // Size of the file in bytes.
    size: u64,

    // Working file name, directory, and path, in relative to the temp root directory.
    // File name without extension, e.g., "file" for "file.png".
    // essential for processing and storage operations.
    // * file_name: The name of the file without its extension.
    file_name: String,
    // * file_directory: The directory where the file is stored, relative to the temp root.
    current_extension: String,
    // * file_directory: The directory where the file is stored, relative to the temp root.
    file_directory: String,
    // * full_file_directory: The full path to the file's directory.
    full_file_directory: PathBuf,

    // * In case of HLS that does not have an original file
    // * We want to only save as a directory.
    // * from /path/{id}/file_that_does_not_exist.mp4 to just /path/{id}/
    // * This is a special case for HLS.
    // * Though the folder will have to provide t_{id}.webp for thumbnail
    // * and /hls/master.m3u8 for the HLS playlist.
    key_override: Option<String>,

    kind: MediaKind,

    width: Option<u32>,
    height: Option<u32>,
    duration: Option<f32>,

    is_deleted: bool,
}

impl File {
    pub fn new() -> Self {
        Self {
            id: None,
            original_name: None,
            original_content_type: None,
            content_type: String::new(),
            detected_extension: String::new(),
            current_extension: String::new(),
            placeholder: None,
            flags: Flags::None as u32,
            file_name: String::new(),
            file_directory: String::new(),
            size: 0,
            full_file_directory: PathBuf::new(),
            kind: MediaKind::Generic,
            key_override: None,
            width: None,
            height: None,
            duration: None,
            is_deleted: false,
        }
    }
}

/// Special feature
impl File {
    pub fn is_animated(&self) -> bool {
        (self.flags & (Flags::IsAnimated as u32)) != 0
    }

    pub fn set_animated(&mut self, animated: bool) {
        if animated {
            self.flags |= Flags::IsAnimated as u32;
        } else {
            self.flags &= !(Flags::IsAnimated as u32);
        }
    }

    pub fn is_hls(&self) -> bool {
        (self.flags & (Flags::IsHLS as u32)) != 0
    }

    pub fn set_hls(&mut self, hls: bool) {
        if hls {
            self.flags |= Flags::IsHLS as u32;
        } else {
            self.flags &= !(Flags::IsHLS as u32);
        }
    }

    pub fn has_thumbnail(&self) -> bool {
        (self.flags & (Flags::HasThumbnail as u32)) != 0
    }

    pub fn set_has_thumbnail(&mut self, has_thumbnail: bool) {
        if has_thumbnail {
            self.flags |= Flags::HasThumbnail as u32;
        } else {
            self.flags &= !(Flags::HasThumbnail as u32);
        }
    }

    pub fn hash(&self) -> Result<String, FileError> {
        use sha2::{Digest, Sha256};
        use std::io::Read;

        let mut file = std::fs::File::open(&self.file_full_path())?;
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

    pub fn delete(&mut self) -> Result<(), FileError> {
        let full_path = self.file_full_path();
        if full_path.exists() {
            std::fs::remove_file(full_path)?;
        }
        self.is_deleted = true;
        Ok(())
    }

    pub fn rename(&mut self, new_name: &str) -> Result<(), FileError> {
        // prepare to rename the file
        let old_full_path = self.file_full_path();
        let new_full_path = self
            .full_file_directory
            .join(format!("{}{}", new_name, self.current_extension));
        // rename the file on the filesystem
        std::fs::rename(&old_full_path, &new_full_path)?;
        // update the file name in the struct
        self.file_name = new_name.to_string();
        Ok(())
    }

    pub fn rename_with_extension(&mut self, name_with_ext: &str) -> Result<(), FileError> {
        // prepare to rename the file
        let old_full_path = self.file_full_path();
        let new_full_path = self.full_file_directory.join(name_with_ext);
        // rename the file on the filesystem
        std::fs::rename(&old_full_path, &new_full_path)?;
        // update the file name and extension in the struct
        let parts: Vec<&str> = name_with_ext.rsplitn(2, '.').collect();
        if parts.len() == 2 {
            self.current_extension = format!(".{}", parts[0]);
            self.file_name = parts[1].to_string();
        } else {
            self.file_name = name_with_ext.to_string();
            self.current_extension.clear();
        }
        Ok(())
    }

    pub fn move_to_directory(
        &mut self,
        new_directory: PathBuf,
        directory_key: &str,
    ) -> Result<(), FileError> {
        // prepare to move the file
        let old_full_path = self.file_full_path();
        let new_full_path =
            new_directory.join(format!("{}{}", self.file_name, self.current_extension));
        // create the new directory if it doesn't exist
        std::fs::create_dir_all(&new_directory)?;
        // move the file on the filesystem
        std::fs::rename(&old_full_path, &new_full_path)?;
        // update the file directory in the struct
        self.file_directory = directory_key.to_string();
        self.full_file_directory = new_directory;
        Ok(())
    }

    pub fn copy_to_directory(
        &mut self,
        new_directory: PathBuf,
        directory_key: &str,
    ) -> Result<File, FileError> {
        // prepare to copy the file
        let old_full_path = self.file_full_path();
        let new_full_path =
            new_directory.join(format!("{}{}", self.file_name, self.current_extension));
        // create the new directory if it doesn't exist
        std::fs::create_dir_all(&new_directory)?;
        // copy the file on the filesystem
        std::fs::copy(&old_full_path, &new_full_path)?;

        let mut new_file = self.clone();
        // update the file directory in the struct
        new_file.file_directory = directory_key.to_string();
        new_file.full_file_directory = new_directory;
        Ok(new_file)
    }

    pub fn prepare_new_extension(&mut self, new_extension: &str) -> PathBuf {
        let new_full_path = self
            .full_file_directory
            .join(format!("{}{}", self.file_name, new_extension));
        new_full_path
    }

    pub fn replace_from_file(&mut self, new_file_path: &PathBuf) -> Result<(), FileError> {
        // prepare to replace the file
        let old_full_path = self.file_full_path();
        // move the new file to the old file's location
        std::fs::rename(new_file_path, &old_full_path)?;
        // update the size and content type in the struct
        self.size = std::fs::metadata(&old_full_path)?.len();
        Ok(())
    }
}

/// Getters
impl File {

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn duration(&self) -> Option<f32> {
        self.duration
    }

    pub fn key_override(&self) -> Option<&str> {
        self.key_override.as_deref()
    }

    pub fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn kind(&self) -> &MediaKind {
        &self.kind
    }

    pub fn original_name(&self) -> Option<&str> {
        self.original_name.as_deref()
    }

    pub fn original_content_type(&self) -> Option<&str> {
        self.original_content_type.as_deref()
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn detected_extension(&self) -> &str {
        &self.detected_extension
    }

    pub fn current_extension(&self) -> &str {
        &self.current_extension
    }

    pub fn placeholder(&self) -> Option<&str> {
        self.placeholder.as_deref()
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_name_with_extension(&self) -> String {
        format!("{}{}", self.file_name, self.current_extension)
    }

    pub fn file_directory(&self) -> &str {
        &self.file_directory
    }

    pub fn file_path(&self) -> String {
        // construct the file path relative to the temp root directory
        format!(
            "{}/{}{}",
            self.file_directory, self.file_name, self.current_extension
        )
    }

    pub fn file_full_path(&self) -> PathBuf {
        // construct the full file path relative to the temp root directory
        let mut path = self.full_file_directory.clone();
        path.push(format!("{}{}", self.file_name, self.current_extension));
        path
    }

    pub fn file_full_directory(&self) -> &PathBuf {
        &self.full_file_directory
    }
}

/// Setters
impl File {
    pub fn set_key_override(&mut self, key_override: Option<String>) -> &mut Self {
        self.key_override = key_override;
        self
    }

    pub fn set_id(&mut self, id: i64) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn set_file_directory(&mut self, file_directory: String) -> &mut Self {
        self.file_directory = file_directory;
        self
    }

    pub fn set_full_file_directory(&mut self, full_file_directory: PathBuf) -> &mut Self {
        self.full_file_directory = full_file_directory;
        self
    }

    pub fn set_kind(&mut self, kind: MediaKind) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn set_detected_extension(&mut self, extension: String) -> &mut Self {
        self.detected_extension = extension;
        self
    }

    pub fn set_current_extension(&mut self, extension: String) -> Result<&mut Self, FileError> {
        // check if the extension got dot

        let extension = if extension.starts_with('.') {
            extension
        } else {
            format!(".{}", extension)
        };

        // if extension is empty, we just normally set it
        // if not empty, rename the file to have the new extension
        if !extension.is_empty()
            && !self.current_extension.is_empty()
            && extension != self.current_extension
        {
            tracing::info!(
                "Changing file extension from {} to {}",
                self.current_extension,
                extension
            );
            let old_full_path = self.file_full_path();
            let new_full_path = self
                .full_file_directory
                .join(format!("{}{}", self.file_name, extension));
            std::fs::rename(&old_full_path, &new_full_path)?;
        }
        self.current_extension = extension;
        Ok(self)
    }

    pub fn set_extension_unsafe(&mut self, extension: String) -> &mut Self {
        self.current_extension = extension;
        self
    }

    pub fn set_original_name(&mut self, name: Option<String>) -> &mut Self {
        self.original_name = name;
        self
    }

    pub fn set_original_content_type(&mut self, content_type: Option<String>) -> &mut Self {
        self.original_content_type = content_type;
        self
    }

    pub fn set_content_type(&mut self, content_type: String) -> &mut Self {
        self.content_type = content_type;
        self
    }

    pub fn set_placeholder(&mut self, placeholder: String) -> &mut Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn set_flags(&mut self, flags: u32) -> &mut Self {
        self.flags = flags;
        self
    }

    pub fn set_size(&mut self, size: u64) -> &mut Self {
        self.size = size;
        self
    }

    pub fn set_file_name(&mut self, file_name: String) -> &mut Self {
        self.file_name = file_name;
        self
    }

    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = Some(height);
        self
    }

    pub fn set_duration(&mut self, duration: f32) -> &mut Self {
        self.duration = Some(duration);
        self
    }
}
