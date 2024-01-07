#import "avfoundationbind.h"
#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>
#include <stdlib.h>

int frameFormatToFourCC(FrameFormat format, FourCharCode *code);
int fourCCToFrameFormat(FourCharCode code, FrameFormat *format);

@interface CameraCapturer
    : NSObject <AVCaptureVideoDataOutputSampleBufferDelegate>

- (void)captureOutput:(AVCaptureOutput *)output
    didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer
           fromConnection:(AVCaptureConnection *)connection;

@end

@implementation CameraCapturer

- (void)captureOutput:(AVCaptureOutput *)output
    didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer
           fromConnection:(AVCaptureConnection *)connection {
}

@end

typedef struct SessionHandle {
  AVCaptureDevice *device;
} SessionHandle;

int ListCameras(DeviceInfo **devices, int *numDevices) {
  static DeviceInfo sDevices[MAX_DEVICES];

  NSArray *types = @[
    AVCaptureDeviceTypeBuiltInWideAngleCamera,
    AVCaptureDeviceTypeContinuityCamera,
    AVCaptureDeviceTypeDeskViewCamera,
  ];

  AVCaptureDeviceDiscoverySession *session = [AVCaptureDeviceDiscoverySession
      discoverySessionWithDeviceTypes:types
                            mediaType:AVMediaTypeVideo
                             position:AVCaptureDevicePositionUnspecified];

  for (int i = 0; i < session.devices.count; i++) {
    if (i >= MAX_DEVICES) {
      NSLog(@"Too many devices while enumerating cameras, ignoring (count=%lu)",
            session.devices.count);
      break;
    }

    AVCaptureDevice *device = session.devices[i];

    DeviceInfo *info = &sDevices[i];
    strncpy(info->name, device.localizedName.UTF8String, MAX_DEVICE_NAME_CHARS);
    info->name[MAX_DEVICE_NAME_CHARS] = '\0';

    strncpy(info->uid, device.uniqueID.UTF8String, MAX_DEVICE_UID_CHARS);
    info->uid[MAX_DEVICE_UID_CHARS] = '\0';

    strncpy(info->model, device.modelID.UTF8String, MAX_DEVICE_MODEL_CHARS);
    info->uid[MAX_DEVICE_MODEL_CHARS] = '\0';

    strncpy(info->manufacturer, device.manufacturer.UTF8String,
            MAX_DEVICE_MANUFACTURER_CHARS);
    info->uid[MAX_DEVICE_MANUFACTURER_CHARS] = '\0';
  }

  *numDevices = session.devices.count;
  *devices = sDevices;
  return 0;
}

int init_session(DeviceInfo *info, SessionHandle **handle) {
  SessionHandle *session = malloc(sizeof(SessionHandle));

  NSString *deviceId = [NSString stringWithUTF8String:info->uid];
  AVCaptureDevice *device = [AVCaptureDevice deviceWithUniqueID:deviceId];

  if (device == NULL) {
    NSLog(@"device not found");
    return 1;
  }

  session->device = device;
  *handle = session;
  return 0;
}

int open_camera(DeviceInfo *info, SessionHandle *handle,
                VideoDeviceFormat format) {
  AVCaptureSession *session = [[AVCaptureSession alloc] init];

  NSError *error;
  AVCaptureDeviceInput *input =
      [[AVCaptureDeviceInput alloc] initWithDevice:handle->device error:&error];

  if (error != NULL) {
    NSLog(@"Error while opening camera: %@", error);
    return 1;
  }

  if (![session canAddInput:input]) {
    NSLog(@"Cannot add input while opening camera");
    return 1;
  }

  [session addInput:input];

  AVCaptureVideoDataOutput *output = [[AVCaptureVideoDataOutput alloc] init];
  CameraCapturer *capturer = [[CameraCapturer alloc] init];

  dispatch_queue_t queue =
      dispatch_queue_create("mikadoCaptureQueue", DISPATCH_QUEUE_SERIAL);

  [output setSampleBufferDelegate:capturer queue:queue];

  // output.videoSettings = @{
  //   (NSString *)kCVPixelBufferPixelFormatTypeKey :
  //   @(kCVPixelFormatType_32BGRA)
  // };

  FourCharCode code;
  int err = frameFormatToFourCC(format.format, &code);
  if (err != 0) {
    NSLog(@"Error while opening camera: format not supported");
    return 1;
  }

  [output setVideoSettings:@{
    (NSString *)kCVPixelBufferPixelFormatTypeKey : @(code),
    (NSString *)kCVPixelBufferWidthKey : @(format.width),
    (NSString *)kCVPixelBufferHeightKey : @(format.height),

  }];

  if (![session canAddOutput:output]) {
    NSLog(@"Cannot add output while opening camera");
    return 1;
  }
  [session addOutput:output];
  [session startRunning];
  return 0;
}

int close_camera(SessionHandle *handle) { return NULL; }

int supported_video_formats(SessionHandle *handle) {

  for (AVCaptureDeviceFormat *format in handle->device.formats) {
    /*DeviceFormat *deviceFormat;
    CMVideoDimensions dimensions =
        CMVideoFormatDescriptionGetDimensions(format.formatDescription);
    FourCharCode code =
        CMFormatDescriptionGetMediaSubType(format.formatDescription);

    deviceFormat->width = dimensions.width;
    deviceFormat->height = dimensions.height;*/

    NSLog(@"Format: %@", format);
  }

  return 0;
}

static NSString *fourCCString(FourCharCode code) {
  NSString *result = [NSString
      stringWithFormat:@"%c%c%c%c", (code >> 24) & 0xff, (code >> 16) & 0xff,
                       (code >> 8) & 0xff, code & 0xff];
  NSCharacterSet *characterSet = [NSCharacterSet whitespaceCharacterSet];
  return [result stringByTrimmingCharactersInSet:characterSet];
}

int frameFormatToFourCC(FrameFormat format, FourCharCode *code) {
  switch (format) {
  case FrameFormat_I420:
    *code = kCVPixelFormatType_420YpCbCr8Planar;
    break;
  case FrameFormat_24RGB:
    *code = kCVPixelFormatType_24RGB;
    break;
  case FrameFormat_24BGR:
    *code = kCVPixelFormatType_24BGR;
    break;
  case FrameFormat_32ARGB:
    *code = kCVPixelFormatType_32ARGB;
    break;
  case FrameFormat_32BGRA:
    *code = kCVPixelFormatType_32BGRA;
    break;
  case FrameFormat_32ABGR:
    *code = kCVPixelFormatType_32ABGR;
    break;
  case FrameFormat_32RGBA:
    *code = kCVPixelFormatType_32RGBA;
    break;
  default:
    return 1;
  }
  return 0;
}

int fourCCToFrameFormat(FourCharCode code, FrameFormat *format) {
  switch (code) {
  case kCVPixelFormatType_420YpCbCr8Planar:
    *format = FrameFormat_I420;
    break;
  case kCVPixelFormatType_24RGB:
    *format = FrameFormat_24RGB;
    break;
  case kCVPixelFormatType_24BGR:
    *format = FrameFormat_24BGR;
    break;
  case kCVPixelFormatType_32ARGB:
    *format = FrameFormat_32ARGB;
    break;
  case kCVPixelFormatType_32BGRA:
    *format = FrameFormat_32BGRA;
    break;
  case kCVPixelFormatType_32ABGR:
    *format = FrameFormat_32ABGR;
    break;
  case kCVPixelFormatType_32RGBA:
    *format = FrameFormat_32RGBA;
    break;
  default:
    return 1;
  }
  return 0;
}
