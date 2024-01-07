// Used to record screen on systems that have x11

mod sys {
    include!(concat!(env!("OUT_DIR"), "/x11.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;

    #[test]
    fn test_x11() {
        unsafe {
            sys::init_x11();

            let mut session = MaybeUninit::uninit();
            let error = sys::new_session(session.as_mut_ptr());
            let session = session.assume_init();

            println!("error: {}", error);
            println!("session: {:?}", session);

            // Open default display
            let error = sys::open_display(session, std::ptr::null_mut());
            println!("error: {}", error);

            let mut screens = MaybeUninit::uninit();
            let mut count = MaybeUninit::uninit();
            let error = sys::list_screens(session, screens.as_mut_ptr(), count.as_mut_ptr());
            let screens = screens.assume_init();
            let count = count.assume_init();

            println!("error: {}", error);
            println!("screens: {:?}", screens);
            println!("count: {}", count);

            for i in 0..count {
                // dereference screen
                let screen = *screens.add(i as usize);
                println!("screen: {:?}", screen);
            }

            let mut windows = MaybeUninit::uninit();
            let mut count = MaybeUninit::uninit();
            let error = sys::list_windows(session, windows.as_mut_ptr(), count.as_mut_ptr());
            println!("error: {}", error);
            let windows = windows.assume_init();
            let count = count.assume_init();

            for i in 0..count {
                // dereference display
                let window = *windows.add(i as usize);
                println!("window: {:?}", window);
                // Print window name
                let cstr = std::ffi::CStr::from_ptr(window.title);
                println!("title: {:?}", cstr);
            }
        }
    }
}
