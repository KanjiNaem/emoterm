use test::test_utils::TEST_MP4_PATH;

// #[path = "service_frame/frame_extraction.rs"] mod frames;
// #[path = "test/test_utils.rs"] mod test;
// #[path = "utils"] mod utils;

mod service_terminal;
mod service_runner;
mod test;
mod utils; // ok so this def looks stupid and ill prob have to redo this later


fn main() {
    // test::test_utils::vid_cap_from_mp4_in_player_test(TEST_MP4_PATH).unwrap();
    // service_terminal::terminal::test().unwrap();
}

// somehow works if uri is passed directly as string ref, but not if passed from const string ref test const
// whyyy???/