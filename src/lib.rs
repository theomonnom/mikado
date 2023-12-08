use bitflags::bitflags;
use std::borrow::Cow;
use thiserror::Error;

mod sys;

#[derive(Debug, Error)]
pub enum MediaError {
    #[error("internal error: {0}")]
    Internal(Cow<'static, str>),
}

pub type Result<T> = std::result::Result<T, MediaError>;

pub struct DeviceManager<O: DeviceObserver> {
    observer: O,
}

impl<O: DeviceObserver> DeviceManager<O> {
    pub fn new(observer: O) -> Self {
        Self { observer }
    }
}

pub trait DeviceObserver {
    fn on_camera_connected(&self, info: &CameraInfo) {}
    fn on_camera_disconnected(&self, info: &CameraInfo) {}

    fn on_microphone_connected(&self, info: &MicrophoneInfo) {}
    fn on_microphone_disconnected(&self, info: &MicrophoneInfo) {}

    fn on_speaker_connected(&self, info: &SpeakerInfo) {}
    fn on_speaker_disconnected(&self, info: &SpeakerInfo) {}
}

#[derive(Default, Debug, Clone)]
pub struct CameraInfo {}

#[derive(Default, Debug, Clone)]
pub struct MicrophoneInfo {}

#[derive(Default, Debug, Clone)]
pub struct SpeakerInfo {}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Permissions: u8 {
        const CAMERA = 0b0000_0001;
        const MICROPHONE = 0b0000_0010;
        const DISPLAY = 0b0000_0100;
        const SPEAKER = 0b0000_1000;
    }
}

pub fn request_permissions(perms: Permissions) -> Result<()> {
    sys::request_permissions()
}

pub fn list_cameras() -> Vec<CameraInfo> {
    sys::list_cameras()
}

pub fn list_microphones() -> Vec<MicrophoneInfo> {
    sys::list_microphones()
}

pub fn list_speakers() -> Vec<SpeakerInfo> {
    sys::list_speakers()
}

pub fn list_displays() -> Vec<DisplayInfo> {
    sys::list_displays()
}
