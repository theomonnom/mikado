#ifndef MIKADO_X11_H
#define MIKADO_X11_H

#include <X11/Xlib.h>

#define MAX_SCREEN_CHARS 64

// One session can only handle one display at a time
typedef struct SessionHandle SessionHandle;

// TODO(theomonnom): How to deal about resolution changes?
typedef struct ScreenInfo {
  int width;
  int height;
  int root_depth; // bits per pixel
  unsigned long white_pixel;
  unsigned long black_pixel;
} ScreenInfo;

typedef struct WindowInfo {
  Window xid;
  char* title;
} WindowInfo;

int new_session(SessionHandle **handle);
int free_session(SessionHandle *handle);

int list_displays(void);

// Screen and windows of a specific display
int list_screens(SessionHandle *handle);
int list_windows(SessionHandle *handle);

// NULL display_name to get the default user display
int open_display(SessionHandle *handle, const char *display_name);
int close_display(SessionHandle *handle);

void bind_free(void *ptr); // Will be used on the Rust side to free memory

#endif // MIKADO_X11_H
