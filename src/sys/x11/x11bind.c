#include "x11bind.h"
#include "window_utils.h"
#include <cstdio>
#include <malloc/_malloc.h>
#include <stdlib.h>
#include <string.h>

// X11 Docs:
// https://www.x.org/releases/current/doc/libX11/libX11/libX11.html

typedef struct SessionHandle {
  Display *display;
  Atom wm_state;
  Atom window_type;
  Atom window_type_normal;
} SessionHandle;

int list_displays() {
  // TODO(theomonnom): Read the /tmp/.X11-unix sockets
  return 0;
}

int new_session(SessionHandle **handle) {
  SessionHandle *h = (SessionHandle *)malloc(sizeof(SessionHandle));

  if (!h) {
    fprintf(stderr, "Failed to allocate memory for session handle\n");
    return 1;
  }

  h->display = NULL;
  h->wm_state = None;
  h->window_type = None;
  h->window_type_normal = None;
  return 0;
}

int list_screens(SessionHandle *handle, ScreenInfo **screens, int *count) {
  const int num_screens = XScreenCount(handle->display);
  ScreenInfo *s = (ScreenInfo *)malloc(sizeof(ScreenInfo) * num_screens);
  if (!s) {
    fprintf(stderr, "Failed to allocate memory for screens list\n");
    return 1;
  }

  for (int i = 0; i < num_screens; i++) {
    Screen *screen = XScreenOfDisplay(handle->display, i);
    ScreenInfo info;
    info.width = screen->width;
    info.height = screen->height;
    info.root_depth = screen->root_depth;
    info.white_pixel = screen->white_pixel;
    info.black_pixel = screen->black_pixel;

    s[i] = info;
  }

  *screens = s;
  *count = num_screens;
  return 0;
}

int list_screen_windows(SessionHandle *handle, int screen, WindowInfo **windows,
                        int *count) {
  Screen *s = XScreenOfDisplay(handle->display, screen);
  Window root = XRootWindowOfScreen(s);

  Window parent;
  Window *children;
  unsigned int num_children;

  if (XQueryTree(handle->display, root, &root, &parent, &children,
                 &num_children) != Success) {
    fprintf(stderr, "Failed to query for child windows for screen %d\n",
            screen);
    return 1;
  }

  // Alloc memory for the windows (All windows may not be valid, so num_children
  // is an upper bound)
  WindowInfo *infos = (WindowInfo *)malloc(sizeof(WindowInfo) * num_children);
  if (!infos) {
    fprintf(stderr, "Failed to allocate memory for windows list\n");
    return 1;
  }

  int total_windows = 0;
  for (unsigned int i = 0; i < num_children; i++) {
    // Iterates in reverse order to return windows from front to back
    Window window = children[num_children - i - 1];
    Window app_window =
        get_app_window(handle->display, window, handle->wm_state);
    if (app_window &&
        is_desktop_window(handle->display, app_window, handle->window_type,
                          handle->window_type_normal)) {
      total_windows++;

      WindowInfo *info = &infos[total_windows - 1];
      info->xid = app_window;
    }
  }

  XFree(children);

  *windows = infos;
  *count = total_windows;
  return 0;
}

int list_windows(SessionHandle *handle, WindowInfo **windows, int *count) {
  const int num_screens = XScreenCount(handle->display);

  int total_windows = 0;
  WindowInfo *combined_windows = NULL;

  for (int i = 0; i < num_screens; i++) {
    WindowInfo *scr_wins;
    int scr_count;
    if (list_screen_windows(handle, i, &scr_wins, &scr_count)) {
      continue;
    }

    if (combined_windows) {
      combined_windows = (WindowInfo *)realloc(
          combined_windows,
          sizeof(WindowInfo) * (total_windows + scr_count));

    } else {
      combined_windows =
          (WindowInfo *)malloc(sizeof(WindowInfo) * scr_count);
    }

    memcpy(combined_windows + total_windows, scr_wins,
           sizeof(WindowInfo) * scr_count);

    total_windows += scr_count;
    free(scr_wins);
  }

  *windows = combined_windows;
  *count = total_windows;
  return 0;
}

int open_display(SessionHandle *h, const char *display_name) {
  if (h->display) {
    fprintf(stderr, "Display is already open\n");
    return 1;
  }

  Display *display = XOpenDisplay(display_name);
  h->display = display;
  h->wm_state = XInternAtom(h->display, "WM_STATE", True);
  h->window_type = XInternAtom(h->display, "_NET_WM_WINDOW_TYPE", True);
  h->window_type_normal =
      XInternAtom(h->display, "_NET_WM_WINDOW_TYPE_NORMAL", True);

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

void bind_free(void *ptr) { free(ptr); }
