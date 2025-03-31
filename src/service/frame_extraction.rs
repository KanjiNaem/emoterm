use opencv::{
    highgui, 
    prelude::*, 
    videoio::{
        self, 
        VideoCapture, 
        VideoCaptureProperties, 
        CAP_PROP_FPS
    }, 
    Result
};

pub fn vid_cap_from_mp4(file_path: &str) -> Result<()> {
    let win_name = "capture_window";
    highgui::named_window(win_name, highgui::WINDOW_AUTOSIZE)?;
    let mut vid_player = VideoCapture::from_file_def(file_path)?;
    let opened = videoio::VideoCapture::is_opened(&vid_player)?;
    if !opened {
        panic!("////////////////////// can't open vid file :( //////////////////////");
    } 

    let vid_fps: f64 = vid_player.get(CAP_PROP_FPS)?;
    let frame_delay: i32 = ((1.0 / vid_fps) * 1000.0) as i32; 

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


pub fn vid_cap_from_mp4_test(file_path: &str) -> Result<()> {
    let win_name = "capture_window";
    let mut vid_player = VideoCapture::from_file_def(file_path)?;
    let opened = videoio::VideoCapture::is_opened(&vid_player)?;
    if !opened {
        panic!("////////////////////// can't open vid file :( //////////////////////");
    } 


    let mut curr_frame = Mat::default();
    vid_player.read(&mut curr_frame)?; 
    // dbg!(&curr_frame.data_typed::<opencv::CV_8UC3>()?);

    // let vid_fps: f64 = vid_player.get(CAP_PROP_FPS)?;
    // let frame_delay: i32 = ((1.0 / vid_fps) * 1000.0) as i32; 

    // loop {
    //     let mut curr_frame = Mat::default();
    //     vid_player.read(&mut curr_frame)?;
    //     if curr_frame.size()?.width > 0 {
    //         highgui::imshow(win_name, &curr_frame)?;
    //     }
        
    //     let key = highgui::wait_key(frame_delay)?;
    //     if key == 113 {
    //         break;
    //     }
    // }

    Ok(())
}