#ifndef MIKADO_X11_XPIXEL_BUFFER_H
#define MIKADO_X11_XPIXEL_BUFFER_H

#include "window_utils.h"
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/Xmd.h>
#include <X11/extensions/XShm.h>

typedef struct XPixelBuffer {
  Display *display;
  Window window;
  WindowRect window_rect;

  XImage *image;
  XImage *shm_image;
  XShmSegmentInfo *shm_segment_info;
  Pixmap shm_pixmap;
  GC shm_gc;

  CARD8 *icc_profile;
  int icc_profile_size;
} XPixelBuffer;

int new_xpixel_buffer(XPixelBuffer **buffer);
int free_xpixel_buffer(XPixelBuffer *buffer);

int xpixel_buffer_init(XPixelBuffer *bufffer, Display *display, Window window,
                       Atom icc_profile);
int xpixel_buffer_release(XPixelBuffer *buffer);

#endif // MIKADO_X11_XPIXEL_BUFFER_H
