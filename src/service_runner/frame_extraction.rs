// use super::super::test::test_utils::*;
use super::super::utils::helper_fns::*;

use image::DynamicImage;
use opencv::{
    prelude::*, 
    videoio::{
        self, 
        VideoCapture, 
    }, 
    Result,
};

enum FrameIterator {
    Image(Option<DynamicImage>),
    Video(VideoCapture),
}

impl FrameIterator {
    pub fn skip_frames(&mut self, skip_amount: usize) {
        match self {
            FrameIterator::Image(_) => {}
            FrameIterator::Video(ref mut vid) => {
                for _ in 0..skip_amount {
                    let mut frame = Mat::default();
                    if !vid.read(&mut frame).unwrap_or(false) || frame.empty() {
                        break;
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        match self {
            FrameIterator::Image(_) => {}
            FrameIterator::Video(ref mut vid) => {
                let _ = vid.set(opencv::videoio::CAP_PROP_POS_AVI_RATIO, 0.0);
            }
        }
    }
}

impl Iterator for FrameIterator {
    type Item = DynamicImage;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FrameIterator::Image(ref mut img) => img.take(),
            FrameIterator::Video(ref mut video) => cap_vid_frame(video),
        }
    }
}

pub struct VideoData {
    pub frame_iterator: FrameIterator,
    pub fps: f64,

}

