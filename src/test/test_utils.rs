use opencv::{
    prelude::*, 
    videoio::{
        self,  
        CAP_PROP_FPS,
        VideoCapture,
    }, 
    highgui,
    Result,
};

use crate::utils::helper_fns::*;

#[allow(dead_code)]
pub fn vid_cap_from_mp4_in_player_test(file_path: &str) -> Result<()> {
    let win_name = "capture_window";
    highgui::named_window(win_name, highgui::WINDOW_AUTOSIZE)?;
    let mut vid_player = VideoCapture::from_file_def(file_path)?;
    let opened = videoio::VideoCapture::is_opened(&vid_player)?;
    if !opened {
        panic!("////////////////////// can't open vid file :( //////////////////////");
    } 

    let vid_fps: f64 = vid_player.get(CAP_PROP_FPS)?;
    let frame_delay: i32 = (1000.0 / vid_fps) as i32; 

    loop {
        let mut curr_frame = Mat::default();
        vid_player.read(&mut curr_frame)?;
        if curr_frame.size()?.width > 0 {
            highgui::imshow(win_name, &curr_frame)?;
        }
        
        let key = highgui::wait_key(frame_delay)?;
        if key == 113 {
            break;
        }
    }

    Ok(())
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub const TEST_MP4_PATH: &str = "/home/kanjinaem/coding/emoterm/src/test/qtEmoBoyDancingTest.mp4"; 