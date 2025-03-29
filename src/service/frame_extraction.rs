// use crate::utils::errors::*;
use crate::utils::{errors::*, helper_fns::*};

use image::{DynamicImage};
use opencv::{prelude::*, videoio::VideoCapture};
use std::{path::Path};
use opencv::{highgui, videoio, Result};


pub enum FrameIterator {
    Video(VideoCapture),
    AnimatedImage {
        frames: Vec<DynamicImage>,
        current_frame: usize,
    },
}

impl Iterator for FrameIterator {
    type Item = DynamicImage;

    fn next(&mut self) -> Option<Self::Item> {
        match self {

            FrameIterator::Video(ref mut video) => capture_video_frame(video),
            FrameIterator::AnimatedImage {
                ref frames,
                ref mut current_frame,
            } => {
                if *current_frame == frames.len() {
                    None
                } else {
                    let frame = frames.get(*current_frame).cloned();
                    *current_frame += 1;
                    frame
                }
            }
        }
    }
}

pub fn play_video_test() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width > 0 {
			highgui::imshow(window, &frame)?;
		}
		let key = highgui::wait_key(10)?;
		if key > 0 && key != 255 {
			break;
		}
	}
	Ok(())
}

fn capture_video_frame(video: &mut VideoCapture) -> Option<DynamicImage> {
    let mut frame = Mat::default();
    if video.read(&mut frame).unwrap_or(false) && !frame.empty() {
        mat_to_dynamic_image(&frame)
    } else {
        None
    }
}