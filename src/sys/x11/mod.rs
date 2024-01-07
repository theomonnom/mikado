// Used to record screen on systems that have x11

mod sys {
    include!(concat!(env!("OUT_DIR"), "/x11.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x11() {
        unsafe {
            sys::init_x11();
        }
    }
}
