// #[path = "service_frame/frame_extraction.rs"] mod frames;
// #[path = "test/test_utils.rs"] mod test;
// #[path = "utils"] mod utils;
mod service_terminal;
mod service_frame;
mod test;
mod utils; // ok so this def looks stupid and ill prob have to redo this later


fn main() {
    // service_frame::frame_extraction::vid_cap_from_mp4_test("/home/kanjinaem/coding/emoterm/src/test/qtEmoGuyDancingTest.mp4").unwrap();

    service_terminal::terminal::test();
}

// somehow works if uri is passed directly as string ref, but not if passed from const string ref test const
// whyyy???/