use crate::application::service::media::processor::types::MediaProcessorOptions;


#[derive(Clone, Debug)]
pub struct MediaServiceOptions {
    pub upload_route: String,
    pub uploader_id: i64,
    pub container: Option<Container>,
    pub processor: Option<MediaProcessorOptions>,
}

#[derive(Clone, Debug)]
pub struct Container {
    pub generate_thumbhash: bool,
    pub no_processing: bool,
    pub use_file_id_sub_container: bool,

    // Only available for processing modules
    pub use_hash_names: bool,
    // Only available for non processing modules
    pub use_raw_names: bool,
    pub use_raw_datas: bool,
    // Only available for hashed image name
    pub use_animated_image_indicator: bool,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            generate_thumbhash: true,
            no_processing: false,
            use_file_id_sub_container: false,
            use_hash_names: false,
            use_raw_names: false,
            use_raw_datas: false,
            use_animated_image_indicator: false,
        }
    }
}