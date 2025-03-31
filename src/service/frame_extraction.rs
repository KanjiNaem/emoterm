use super::super::utils::helper_fns::*;

use opencv::{
    prelude::*, 
    videoio::{
        self, 
        VideoCapture, 
    }, 
    Result,
};

pub fn vid_cap_from_mp4_test(file_path: &str) -> Result<()> {
    let mut vid_player = VideoCapture::from_file_def(file_path)?;
    let opened = videoio::VideoCapture::is_opened(&vid_player)?;
    if !opened {
        panic!("////////////////////// can't open vid file :( //////////////////////");
    } 

    let mut curr_frame = Mat::default();
    vid_player.read(&mut curr_frame)?;
    let test_vec: Vec<[u8; 3]> = mat_to_vec3(&curr_frame)?; //bgr
    dbg!(test_vec);
    
    Ok(())
}


