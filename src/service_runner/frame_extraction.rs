// use super::super::test::test_utils::*;
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
    let mut ascii_vec: Vec<String> = vec![String::from(""); test_vec.len()];
    
    let mut curr_px: usize = 0;
    for curr_brg_vec in test_vec {
        ascii_vec[curr_px] = assign_ascii_for_brightness_av(curr_brg_vec.clone());
        curr_px += 1;
    } 

    // dbg!(test_vec);
    // dbg!(&ascii_vec);
    Ok(())
}


