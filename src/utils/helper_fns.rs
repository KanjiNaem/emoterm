use image::{
    DynamicImage, 
    ImageBuffer,
};

use opencv::{
    core::{
        AlgorithmHint, 
        Mat, 
        MatTraitConst,
        MatTraitConstManual,
    }, 
    imgproc, 
    videoio::{
        VideoCapture, 
        VideoCaptureTrait
    }, 
    Result,
};

#[allow(dead_code)]
fn mat_to_dyn_image(input_mat: &Mat) -> Option<DynamicImage> {
    let mut px_mat = Mat::default();
    if imgproc::cvt_color(
        &input_mat,
        &mut px_mat,
        imgproc::COLOR_BGR2RGB,
        0,
        AlgorithmHint::ALGO_HINT_DEFAULT,
    )
.is_ok()
    {
        if let Ok(_elem_size) = px_mat.elem_size() {
            if let Ok(size) = px_mat.size() {
                let new_mat = px_mat.reshape(1, size.width * size.height).ok()?;
                let data_vec: Vec<u8> = new_mat
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

#[allow(dead_code)]
pub fn assign_ascii_for_brightness_av(input_vec: [u8; 3]) -> String {
    let brightness_ascii_map: Vec<&str> = vec![" ", ".", "","", "-", "~", ":", ";", "=", "!", "*", "?", "$", "@", "#"];
    let weighted_brightness_av: f64 = (0.2126 * input_vec[0] as f64) + (0.0722 * input_vec[1] as f64) + (0.7152 * input_vec[2] as f64);
    let char_range: f64 = 255.0 / brightness_ascii_map.len() as f64;
    let index_min_eval: usize = f64::min((weighted_brightness_av / char_range) as f64, (brightness_ascii_map.len() - 1) as f64) as usize;
    let assigned_ascii: String = String::from(brightness_ascii_map[index_min_eval]);
    return assigned_ascii;
}

pub fn cap_vid_frame(input_video: &mut VideoCapture) -> Option<DynamicImage> {
    let mut frame = Mat::default();
    if input_video.read(&mut frame).unwrap_or(false) && !frame.empty() {
        mat_to_dyn_image(&frame)
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn mat_to_vec3(mat: &Mat) -> Result<Vec<[u8; 3]>> {
    if mat.channels() != 3 {
        panic!("Expected 3-channel Mat");
    }
    if mat.depth() != opencv::core::CV_8U {
        panic!("Expected 8-bit unsigned Mat");
    }

    let rows = mat.rows() as usize;
    let cols = mat.cols() as usize;
    let mut result = Vec::with_capacity(rows * cols);
    
    let bytes = mat.data_bytes()?;
    let expected_bytes = rows * cols * 3;
    if bytes.len() != expected_bytes {
        panic!("//////////// crash :(, some bs happened, cries ////////////");
    }

    for chunk in bytes.chunks_exact(3) {
        result.push([chunk[0], chunk[1], chunk[2]]);
    }

    Ok(result)
}