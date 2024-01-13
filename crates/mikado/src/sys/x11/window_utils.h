#ifndef MIKADO_X11_WINDOW_UTILS_H
#define MIKADO_X11_WINDOW_UTILS_H

#include <X11/Xlib.h>
#include <stdbool.h>

typedef struct WindowRect {
  int x;
  int y;
  int width;
  int height;
} WindowRect;

// Iterates through `window` hierarchy to find first visible window, i.e. one
// that has WM_STATE property set to NormalState.
// See http://tronche.com/gui/x/icccm/sec-4.html#s-4.1.3.1 .
Window get_app_window(Display *display, Window window, Atom wm_state);

// Returns true if the `window` is a desktop element.
bool is_desktop_element(Display *display, Window window, Atom window_type,
                        Atom window_type_normal);

int get_window_rect(Display *display, Window window, WindowRect *rect,
                    XWindowAttributes *attrs);

int get_window_title(Display *display, Window window, char **title);

#endif // MIKADO_X11_WINDOW_UTILS_H
