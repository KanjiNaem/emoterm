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