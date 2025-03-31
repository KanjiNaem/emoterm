#[path = "service/frame_extraction.rs"] mod frame;
#[path = "test/test_utils.rs"] mod test;
// #[path = "utils/errors.rs"] mod err;


fn main() {
    frame::get_vid_from_mp4("/home/kanjinaem/coding/emoterm/src/test/qtEmoGuyDancingTest.mp4").unwrap();
}

// somehow works if uri is passed directly as string ref, but not if passed from const string ref test const
// whyyy???/