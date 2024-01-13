#include "xpixel_buffer.h"
#include "window_utils.h"
#include <X11/Xutil.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/shm.h>

int xpixel_buffer_release_shm(XPixelBuffer *buffer);
int xpixel_buffer_init_pixmap(XPixelBuffer *buffer);

bool is_window_valid(Display *display, Window window) { return false; }

int new_xpixel_buffer(XPixelBuffer **buffer) {
  XPixelBuffer *b = (XPixelBuffer *)malloc(sizeof(XPixelBuffer));
  b->display = NULL;
  b->window = 0;

  *buffer = b;
  return 0;
}

int xpixel_buffer_init(XPixelBuffer *buffer, Display *display, Window window,
                       Atom icc_profile) {
  xpixel_buffer_release(buffer);
  buffer->display = display;

  XWindowAttributes attrs;
  if (!get_window_rect(display, window, &buffer->window_rect, &attrs)) {
    return 1;
  }

  Atom actual_type;
  int actual_format;
  unsigned long bytes_after;
  unsigned long size;
  unsigned char *data = NULL;
  int status = XGetWindowProperty(display, window, icc_profile, 0, ~0L, False,
                                  AnyPropertyType, &actual_type, &actual_format,
                                  &size, &bytes_after, &data);

  if (status == Success && size > 0) {
    CARD8 *icc_profile = (CARD8 *)malloc(sizeof(CARD8) * size);
    memcpy(icc_profile, data, size);
    buffer->icc_profile = icc_profile;
    buffer->icc_profile_size = size;
    XFree(data);
  } else {
    fprintf(stderr, "Failed to get ICC profile for window\n");
    return 1;
  }

  buffer->window = window;

  // Try to init SHM
  Visual *visual = attrs.visual;
  int default_depth = attrs.depth;

  int major, minor;
  Bool have_pixmaps;
  if (!XShmQueryVersion(display, &major, &minor, &have_pixmaps)) {
    // Shared memory not supported. CaptureRect will use the XImage API instead.
    return 0;
  }

  Bool using_shm = false;
  buffer->shm_segment_info = (XShmSegmentInfo *)malloc(sizeof(XShmSegmentInfo));
  buffer->shm_segment_info->shmid = -1;
  buffer->shm_segment_info->shmaddr = NULL;
  buffer->shm_segment_info->readOnly = False;
  buffer->shm_image = XShmCreateImage(
      display, visual, default_depth, ZPixmap, NULL, buffer->shm_segment_info,
      buffer->window_rect.width, buffer->window_rect.height);

  if (buffer->shm_image) {
    buffer->shm_segment_info->shmid =
        shmget(IPC_PRIVATE,
               buffer->shm_image->bytes_per_line * buffer->shm_image->height,
               IPC_CREAT | 0600);

    if (buffer->shm_segment_info->shmid != -1) {
      void *shmat_result = shmat(buffer->shm_segment_info->shmid, 0, 0);
      if (shmat_result != (void *)-1) {
        buffer->shm_segment_info->shmaddr = (char *)shmat_result;
        buffer->shm_image->data = buffer->shm_segment_info->shmaddr;

        using_shm = XShmAttach(display, buffer->shm_segment_info);
        XSync(display, False);
      }
    } else {
      fprintf(stderr,
              "Failed to create shared memory segment image. Performance "
              "may be degraded.\n");
      return 1;
    }
  }

  if (!using_shm) {
    xpixel_buffer_release_shm(buffer);
    return 0;
  }

  if (have_pixmaps) {
  }

  return 0;
}

int xpixel_buffer_init_pixmap(XPixelBuffer *buffer, int depth) {
  if (XShmPixmapFormat(buffer->display) != ZPixmap)
    return 1;

  buffer->shm_pixmap = XShmCreatePixmap(
      buffer->display, buffer->window, buffer->shm_segment_info->shmaddr,
      buffer->shm_segment_info, buffer->window_rect.width,
      buffer->shm_image->height, depth);
  XSync(buffer->display, False);

  return 0;
}

int xpixel_buffer_release(XPixelBuffer *buffer) {
  if (buffer->image) {
    XDestroyImage(buffer->image);
    buffer->image = NULL;
  }

  if (buffer->shm_image) {
    XDestroyImage(buffer->shm_image);
    buffer->shm_image = NULL;
  }

  if (buffer->shm_pixmap) {
    XFreePixmap(buffer->display, buffer->shm_pixmap);
    buffer->shm_pixmap = 0;
  }

  if (buffer->shm_gc) {
    XFreeGC(buffer->display, buffer->shm_gc);
    buffer->shm_gc = NULL;
  }

  xpixel_buffer_release_shm(buffer);
  buffer->window = 0;
  return 0;
}

int xpixel_buffer_release_shm(XPixelBuffer *buffer) {
  // Release shared memory segment
  if (buffer->shm_segment_info) {
    if (buffer->shm_segment_info->shmid)
      shmdt(buffer->shm_segment_info->shmaddr);
    if (buffer->shm_segment_info->shmid)
      shmctl(buffer->shm_segment_info->shmid, IPC_RMID, NULL);
  }
  buffer->shm_segment_info = NULL;
  return 0;
}

// TODO(theomonnom): XDamage & XFixes -> Opts
