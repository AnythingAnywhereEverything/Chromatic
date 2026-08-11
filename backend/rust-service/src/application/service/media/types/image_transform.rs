#[derive(Debug, Clone, Copy)]
pub enum CropStyle {
    /// Freeform cropping based on exact width and height dimensions
    Absolute { width: u32, height: u32 },

    /// Normalized cropping based on a percentage of the original image dimensions (e.g., 0.5 for 50% of the original size)
    Normalized { width: f32, height: f32 },

    /// Proportion-locked cropping using a ratio and a defining dimension (e.g., width)
    Ratio { ratio: (u32, u32), scale: f32 },
}

#[derive(Debug, Clone, Copy)]
pub enum ResizeStyle {
    /// Resize to exact width and height dimensions
    /// will be cropped if the aspect ratio is different from the original image
    AbsoluteWithCrop { width: u32, height: u32 },

    /// Resize based on a percentage of the original image dimensions (e.g., 0.5 for 50% of the original size)
    Normalized { width: f32, height: f32 },

    /// Proportion-locked resizing using a ratio and a defining dimension (e.g., width)
    AbsoluteKeepsRatio { width: u32, height: u32 },
}