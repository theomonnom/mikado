// Used to record screen on systems that have x11

mod sys {
    include!(concat!(env!("OUT_DIR"), "/x11.rs"));
}
