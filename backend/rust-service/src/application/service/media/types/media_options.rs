use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]

// * ----------------------------
// * Enums
// * ----------------------------

pub enum ValidationType {
    Whitelisted,
    Blacklisted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    // Blacklist & Whitelist
    GenericJpeg,
    GenericPng,
    GenericWebP,
    GenericMp4,
    GenericGif,

    // Categorized
    Image,
    Video,
    Audio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaCategory {
    Image,
    Video,
    Audio,
    Document,
    Code,
    Archive,
    Unknown,
}

impl PartialEq<MediaCategory> for &MediaCategory {
    fn eq(&self, other: &MediaCategory) -> bool {
        *self == other
    }
}

// * ----------------------------
// * Media Options
// * ----------------------------

#[derive(Debug, Clone)]
pub struct ValidationOptions {
    pub validation_type: ValidationType,
    pub value: Vec<MediaType>,
}
// * ----------------------------
// * Multipart Extractor Options
// * ----------------------------

pub trait MultipartSchema {
    fn multipart_fields() -> &'static [MultipartField];

    fn multipart_field(name: &str) -> Option<&'static MultipartField> {
        Self::multipart_fields()
            .iter()
            .find(|field| field.name == name)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MultipartFieldKind {
    Text,
    File,
}

#[derive(Debug, Clone, Copy)]
pub enum MultipartFieldCardinality {
    Single,
    Optional,
    Many,
    OptionalMany,
}

#[derive(Debug, Clone, Copy)]
pub struct MultipartField {
    pub name: &'static str,
    pub kind: MultipartFieldKind,
    pub cardinality: MultipartFieldCardinality,
}

pub struct FieldTypeFilter {
    pub max_file_size: Option<usize>,
    pub affected_types: Option<Vec<MediaType>>,
}

pub struct MultipartExtractorOptions {
    // Hard limit, Cannot be override by field options
    pub max_file_size: Option<usize>,
    // Hard Limit, Cannot be override by field options
    pub max_files: Option<usize>,
    pub validation: Option<ValidationOptions>,
    pub filter: Option<Vec<FieldTypeFilter>>,
}

impl Default for MultipartExtractorOptions {
    fn default() -> Self {
        Self {
            max_file_size: None,
            max_files: None,
            validation: None,
            filter: None,
        }
    }
}