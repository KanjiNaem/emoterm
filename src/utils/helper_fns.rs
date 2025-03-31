use opencv::{
    core::{
    Mat,
    MatTraitConst,
    MatTraitConstManual,
    },
    Result
};

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

#[allow(dead_code)]
pub fn assign_ascii_for_brightness_av(input_vec: [u8; 3]) -> String {
    let brightness_ascii_map: Vec<&str> = vec![" ", ".", "","", "-", "~", ":", ";", "=", "!", "*", "?", "$", "@", "#"];
    let weighted_brightness_av: f64 = (0.2126 * input_vec[0] as f64) + (0.0722 * input_vec[1] as f64) + (0.7152 * input_vec[2] as f64);
    let char_range: f64 = 255.0 / brightness_ascii_map.len() as f64;
    let index_min_eval: usize = f64::min((weighted_brightness_av / char_range) as f64, (brightness_ascii_map.len() - 1) as f64) as usize;
    let assigned_ascii: String = String::from(brightness_ascii_map[index_min_eval]);
    return assigned_ascii;
}