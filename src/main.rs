// #[path = "service/frame_extraction.rs"] mod frames;
// #[path = "test/test_utils.rs"] mod test;
// #[path = "utils"] mod utils;
mod service;
mod test;
mod utils; // ok so this def looks stupid and ill prob have to redo this later


fn main() {
    // frame::vid_cap_from_mp4("/home/kanjinaem/coding/emoterm/src/test/qtEmoGuyDancingTest.mp4").unwrap();
    service::frame_extraction::vid_cap_from_mp4_test("/home/kanjinaem/coding/emoterm/src/test/qtEmoGuyDancingTest.mp4").unwrap();
}

// somehow works if uri is passed directly as string ref, but not if passed from const string ref test const
// whyyy???/