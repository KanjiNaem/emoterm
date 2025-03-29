use image::{DynamicImage, ImageBuffer};
use opencv::{core::AlgorithmHint, imgproc, prelude::*};
use std::process::{Command, Stdio};
use std::str::FromStr;

pub fn mat_to_dynamic_image(mat: &Mat) -> Option<DynamicImage> {
    let mut rgb_mat = Mat::default();
    if imgproc::cvt_color(
        &mat,
        &mut rgb_mat,
        imgproc::COLOR_BGR2RGB,
        0,
        AlgorithmHint::ALGO_HINT_DEFAULT,
    )
    .is_ok()
    {
        if let Ok(_elem_size) = rgb_mat.elem_size() {
            if let Ok(size) = rgb_mat.size() {
                let reshaped_mat = rgb_mat.reshape(1, size.width * size.height).ok()?;
                let data_vec: Vec<u8> = reshaped_mat
                    .data_typed::<u8>()
                    .expect("Unexpected invalid data")
                    .to_vec();

                if let Some(img_buf) = ImageBuffer::<image::Rgb<u8>, _>::from_raw(
                    size.width as u32,
                    size.height as u32,
                    data_vec,
                ) {
                    return Some(DynamicImage::ImageRgb8(img_buf));
                }
            }
        }
    }
    None
}