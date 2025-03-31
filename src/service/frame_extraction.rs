use opencv::{highgui, prelude::*, videoio::{self, VideoCapture, VideoCaptureProperties, CAP_PROP_FPS}, Result};

pub fn get_vid_from_mp4(file_path: &str) -> Result<()> {
    let win_name = "capture_window";
    highgui::named_window(win_name, highgui::WINDOW_AUTOSIZE)?;
    let mut vid_player = VideoCapture::from_file_def(file_path)?;
    let opened = videoio::VideoCapture::is_opened(&vid_player)?;
    if !opened {
        panic!("////////////////////// can't open vid file :( //////////////////////");
    } 

    loop {
        let mut curr_frame = Mat::default();
        vid_player.read(&mut curr_frame)?;
        if curr_frame.size()?.width > 0 {
            highgui::imshow(win_name, &curr_frame)?;
        }
        let vid_fps = vid_player.get(CAP_PROP_FPS)? as i32;
        let key = highgui::wait_key(vid_fps)?;
        if key == 113 {
            break;
        }
    }

    Ok(())
}