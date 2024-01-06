#include "x11bind.h"
#include <cstdio>

typedef struct SessionHandle {
  Display *display;
} SessionHandle;

int list_displays() {
  // TODO(theomonnom): Read the /tmp/.X11-unix sockets
  return 0;
}

int new_session(SessionHandle **handle) {
  return 0;
}

int list_screens(SessionHandle *handle, ScreenInfo **screens, int *count) {
  const int num_screens = XScreenCount(handle->display);
  for (int i = 0; i < num_screens; i++) {
    Screen *screen = XScreenOfDisplay(handle->display, i);
    ScreenInfo info;
    info.width = screen->width;
    info.height = screen->height;
    info.root_depth = screen->root_depth;
    info.white_pixel = screen->white_pixel;
    info.black_pixel = screen->black_pixel;
  }
  return 0;
}

int list_windows(SessionHandle *handle) {
  // For each screen, list every window
  int failed_screens = 0;
  const int num_screens = XScreenCount(handle->display);
  for (int i = 0; i < num_screens; i++) {
    Screen *screen = XScreenOfDisplay(handle->display, i);
  }
  return 0;
}


int open_display(SessionHandle *handle, const char *display_name) {
  Display *display = XOpenDisplay(display_name);
  handle->display = display;
  return 0;
}

int close_display(SessionHandle *handle) {
  if (!XCloseDisplay(handle->display)) {
    fprintf(stderr, "Failed to close display\n");
    return 1;
  }
  handle->display = NULL;
  return 0;
}
