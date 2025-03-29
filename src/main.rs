use std::path::Path;

#[path = "service/frame_extraction.rs"] mod frame;
// #[path = "utils/errors.rs"] mod err;
mod utils;

fn main() {
    // frame::open_video(Path::new("/test/qtEmoBoyDancing.mp4"));
    frame::play_video_test();
}
