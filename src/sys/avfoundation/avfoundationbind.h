#ifndef MIKADO_AVFOUNDATION_H
#define MIKADO_AVFOUNDATION_H

#define MAX_DEVICES 16
#define MAX_DEVICE_UID_CHARS 64
#define MAX_DEVICE_NAME_CHARS 64
#define MAX_DEVICE_MODEL_CHARS 64
#define MAX_DEVICE_MANUFACTURER_CHARS 64

typedef struct DeviceInfo {
  char uid[MAX_DEVICE_UID_CHARS + 1];
  char name[MAX_DEVICE_NAME_CHARS + 1];
  char model[MAX_DEVICE_MODEL_CHARS + 1];
  char manufacturer[MAX_DEVICE_MANUFACTURER_CHARS + 1];
} DeviceInfo;

typedef struct SessionHandle SessionHandle;

// On MacOS, this will always be VideoRotation0
typedef enum VideoRotation {
  VideoRotation0,
  VideoRotation90,
  VideoRotation180,
  VideoRotation270
} VideoRotation;

// maps to kCVPixelFormatType_*, shouldn't be hard to add new ones
typedef enum FrameFormat {
  FrameFormat_I420,
  FrameFormat_24RGB,
  FrameFormat_24BGR,
  FrameFormat_32ARGB,
  FrameFormat_32BGRA,
  FrameFormat_32ABGR,
  FrameFormat_32RGBA,
} FrameFormat;

typedef struct ComponentDesc {
  void *data;
  int len;
  int stride;
} ComponentDesc;

typedef struct BufferDesc {
  FrameFormat format;
  VideoRotation rotation;
  ComponentDesc *components;
  int components_len;
  int width;
  int height;
} BufferDesc;

// Supported formats of a VideoDeviceFormat
typedef struct VideoDeviceFormat {
  double min_fps;
  double max_fps;
  int width;
  int height;
  FrameFormat format;
} VideoDeviceFormat;

typedef void (*DataCallback)(BufferDesc *desc);

int list_cameras(DeviceInfo **devices, int *numDevices);
int new_session(DeviceInfo *info, SessionHandle **handle);
int open_session(SessionHandle *handle);
int close_camera(SessionHandle *handle);

// Video formats supported by the hardware
// This doesn't mean that we can't use another format for the capture output
int supported_video_formats(SessionHandle *handle);

#endif // MIKADO_AVFOUNDATION_H
