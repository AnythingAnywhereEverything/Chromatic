#![allow(dead_code)]

use crate::common::init_libvips;
use libvips::ops;

pub struct GeneratedTestImage {
    pub jpeg_bytes: Vec<u8>,
    pub normalized_position: (f32, f32),
    pub normalized_scale: f32,
}

pub struct GeneratedTestVerification {
    pub expected_size: (i32, i32),
}

pub async fn generate_testimage(
    detect_ratio: (u32, u32),
) -> (GeneratedTestImage, GeneratedTestVerification) {
    init_libvips();
    // Create random width and height between 400 and 1240
    let width = rand::random::<u32>() % 841 + 400;
    let height = rand::random::<u32>() % 841 + 400;

    let canvas_opts = ops::BlackOptions { bands: 3 };
    let canvas = ops::black_with_opts(width as i32, height as i32, &canvas_opts)
        .expect("Failed to generate a test image base");

    let mut scale = [1.0, 1.0, 1.0];
    let mut red_offset = [255.0, 0.0, 0.0];

    let canvas = ops::linear(&canvas, &mut scale[..], &mut red_offset[..])
        .expect("Failed to paint the test image red");

    let base_random_size = rand::random::<u32>() % 201 + 50;

    let box_width = detect_ratio.0 * base_random_size;
    let box_height = detect_ratio.1 * base_random_size;

    // Randomly position the box within the canvas
    let box_top = rand::random::<u32>() % (height - box_height);
    let box_left = rand::random::<u32>() % (width - box_width);

    let green_box = ops::black_with_opts(box_width as i32, box_height as i32, &canvas_opts)
        .expect("Failed to generate a green box base");

    let mut green_offset = [0.0, 255.0, 0.0];
    let green_box = ops::linear(&green_box, &mut scale[..], &mut green_offset[..])
        .expect("Failed to paint the green box");

    let final_image = ops::insert(&canvas, &green_box, box_left as i32, box_top as i32)
        .expect("Failed to insert the green box into the canvas");

    // Position normalized to image
    let normalized_position = (
        box_left as f32 / width as f32,
        box_top as f32 / height as f32,
    );

    // Largest crop that fits this aspect ratio
    let aspect = detect_ratio.0 as f32 / detect_ratio.1 as f32;

    let (max_crop_width, max_crop_height) = if width as f32 / height as f32 >= aspect {
        let h = height as f32;
        let w = h * aspect;
        (w, h)
    } else {
        let w = width as f32;
        let h = w / aspect;
        (w, h)
    };

    // Scale relative to the maximum crop for this aspect ratio
    let normalized_scale = box_width as f32 / max_crop_width;

    let jpeg_bytes = ops::jpegsave_buffer(&final_image)
        .expect("Failed to convert the final image to JPEG bytes");

    print!(
        "Generated test image with dimensions: {}x{}, box at ({}, {}) with size {}x{}, normalized position: {:?}, normalized scale: {}\n",
        width,
        height,
        box_left,
        box_top,
        box_width,
        box_height,
        normalized_position,
        normalized_scale
    );

    print!(
        "Verify normalized position accuracy: box_left as px = {}, box_top as px = {}\n",
        (normalized_position.0 * width as f32).round() as u32,
        (normalized_position.1 * height as f32).round() as u32
    );

    print!(
        "Verify normalized scale accuracy: box_width as px = {}, box_height as px = {}\n",
        (normalized_scale * max_crop_width).round() as u32,
        (normalized_scale * max_crop_height).round() as u32
    );

    let generated_image = GeneratedTestImage {
        jpeg_bytes,
        normalized_position,
        normalized_scale,
    };

    let verification = GeneratedTestVerification {
        expected_size: (box_width as i32, box_height as i32),
    };

    (generated_image, verification)
}

