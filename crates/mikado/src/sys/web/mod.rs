use mutex::Mutex;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{self, JsFuture};
use web_sys::Event;

mod audio_device;
mod mutex;

pub fn init() -> crate::Result<()> {
    let window = web_sys::window().ok_or(MediaError::Internal(
        "should have a window in this context".into(),
    ))?;

    let device_change_lock = Rc::new(Mutex::new(()));
    let media_devices = window
        .navigator()
        .media_devices()
        .map_err(|_| MediaError::Internal("should have media devices in this context".into()))?;

    let ondevicechange = Closure::<dyn Fn(_)>::new(move |ev| {
        handle_device_change(ev, device_change_lock.clone());
    })
    .into_js_value();
    media_devices.set_ondevicechange(Some(ondevicechange.unchecked_ref()));

    Ok(())
}

fn handle_device_change(_: Event, order_lock: Rc<Mutex<()>>) {
    wasm_bindgen_futures::spawn_local(async move {
        let _lock = order_lock.lock().await;

        match enumerate_devices().await {
            Ok(devices) => {
                // ret the new devices
            }
            Err(e) => log::warn!("ondevicechange: failed to enumerate devices: {:?}", e),
        }
    });
}

pub async fn list_cameras() -> crate::Result<Vec<DeviceInfo>> {
    let window = web_sys::window().unwrap();
    let media_devices = window.navigator().media_devices().unwrap();

    let devices = JsFuture::from(media_devices.enumerate_devices().unwrap())
        .await
        .map_err(|_| MediaError::Internal("failed to enumerate devices".into()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_enumerate_devices() {
        init().unwrap();
        let devices = enumerate_devices().await;
        println!("devices: {:?}", devices);
    }
}
