use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Target not found.")]
    NotFound,
    #[error("Permission denied.")]
    PermissionDenied,
    #[error("Connection failed.")]
    ConnectionFailed,
    #[error("Unknown error.")]
    Unknown,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

}

#[derive(Debug, Error)]
pub enum FileError {
    #[error("File not found.")]
    NotFound,
    #[error("Permission denied.")]
    PermissionDenied,
    #[error("Invalid file format.")]
    InvalidFormat,
    #[error("File is too large.")]
    TooLarge,
    #[error("File is empty.")]
    Empty,
    #[error("Unknown error.")]
    Unknown,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),
}

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("File was just pushed.")]
    FileJustPushed,

    #[error("File is missing an ID.")]
    FileWithoutId,

    #[error("File is outside the container's relative path.")]
    FileOutsideContainer,

    #[error("Container failed to initialize.")]
    InitializationFailed,

    #[error("Container abort.")]
    Abort,

    #[error("Container is empty.")]
    EmptyContainer,

    #[error("Destination not specified.")]
    MissingTargetPath,

    #[error(transparent)]
    SnowflakeServiceError(#[from] SnowflakeServiceError),

    #[error(transparent)]
    StorageError(#[from] StorageError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[error(transparent)]
    FileError(#[from] FileError),
}

#[derive(Debug, Error)]
pub enum InspectionError {
    #[error("Failed to inspect media.")]
    InspectionFailed,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    LibvipsError(#[from] rs_vips::error::Error),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),
}

#[derive(Debug, Error)]
pub enum ExtractionError {
    #[error("Misconfigured extraction options.")]
    Misconfigured,

    #[error("Transmission too many files.")]
    TransmissionTooManyFiles,

    #[error("Invalid file type: {0}.")]
    InvalidFileType(String),
    
    #[error("Failed to Deserialize {0}.")]
    DeserializationFailed(String),

    #[error("Duplicate multipart field: {0}.")]
    DuplicateMultipartField(String),

    #[error("Failed to read multipart field '{0}': {1}")]
    FailedToReadMultipartField(String, String),

    #[error("Unknown multipart field: {0}.")]
    UnknownMultipartField(String),

    #[error("Unable to read field name.")]
    UnableToReadFieldName,

    #[error("Extraction timed out.")]
    Timeout,

    #[error("Transmission too slow.")]
    TransmissionTooSlow,

    #[error("Failed to extract media.")]
    ExtractionFailed,

    #[error("Transmission too large.")]
    TransmissionTooLarge,

    #[error(transparent)]
    ContainerError(#[from] ContainerError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    InspectionError(#[from] InspectionError),

    #[error(transparent)]
    MultipartError(#[from] axum::extract::multipart::MultipartError),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),
}

#[derive(Debug, Error)]
pub enum MediaServiceError {
    #[error("Failed to process the media.")]
    ProcessingFailed,

    #[error("Uploader ID not set.")]
    UploaderIdNotSet,

    #[error("No files in container.")]
    NoFilesInContainer,

    #[error("Internal system error.")]
    InternalServer,

    #[error("Storage unavailable.")]
    StorageUnavailable,

    #[error("Operation timed out.")]
    Timeout,

    #[error("Transmission too slow.")]
    TransmissionTooSlow,

    #[error("Unsupported format {0}.")]
    UnsupportedFormat(String),

    #[error(transparent)]
    FileError(#[from] FileError),
    
    #[error(transparent)]
    ProcessingError(#[from] MediaProcessorError),

    #[error(transparent)]
    SnowflakeError(#[from] SnowflakeServiceError),

    #[error(transparent)]
    LibvipsError(#[from] rs_vips::error::Error),

    #[error(transparent)]
    ExtractionError(#[from] ExtractionError),

    #[error(transparent)]
    InspectionError(#[from] InspectionError),

    #[error(transparent)]
    StorageError(#[from] StorageError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ContainerError(#[from] ContainerError),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum MediaProcessorError {
    #[error("Failed to trim video.")]
    VideoTrimFailed,

    #[error("Failed to generate thumbnail.")]
    ThumbnailGenerationFailed,

    #[error("Failed to strip metadata.")]
    MetadataStripFailed,

    #[error("Invalid crop scale: {0}.")]
    InvalidCropScale(f32),

    #[error("Failed to process the media.")]
    ProcessingFailed,

    #[error("Failed to get Width.")]
    GetWidthFailed,

    #[error("Failed to get Height.")]
    GetHeightFailed,

    #[error("Failed to get video duration.")]
    GetDurationFailed,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    LibvipsError(#[from] rs_vips::error::Error),

    #[error(transparent)]
    StorageError(#[from] StorageError),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

