use crate::CameraInfo;

//include!(concat!(env!("OUT_DIR"), "/avfoundation.rs"));

pub fn init() -> crate::Result<()> {
    Ok(())
}

pub fn list_cameras() -> crate::Result<Vec<CameraInfo>> {
    Ok(vec![])
}
