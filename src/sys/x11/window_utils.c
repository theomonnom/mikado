#include "window_utils.h"
#include <X11/Xmd.h>
#include <X11/Xutil.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

bool is_desktop_element(Display *display, Window window, Atom window_type,
                        Atom window_type_normal) {
  if (window == 0)
    return false;

  // First look for _NET_WM_WINDOW_TYPE. The standard
  // (http://standards.freedesktop.org/wm-spec/latest/ar01s05.html#id2760306)
  // says this hint *should* be present on all windows, and we use the existence
  // of _NET_WM_WINDOW_TYPE_NORMAL in the property to indicate a window is not
  // a desktop element (that is, only "normal" windows should be shareable).
  Atom actual_type;
  int actual_format;
  unsigned long bytes_after;
  unsigned long size;
  unsigned char *data = NULL;
  Status status = XGetWindowProperty(
      display, window, window_type, 0L, ~0L, False, AnyPropertyType,
      &actual_type, &actual_format, &size, &bytes_after, &data);

  if (status == Success && size > 0) {
    CARD32 *end = (CARD32 *)data + size;
    bool is_normal = false;
    for (CARD32 *p = (CARD32 *)data; p < end; ++p) {
      if (*p == window_type_normal) {
        is_normal = true;
        break;
      }
    }

    XFree(data);
    return !is_normal;
  }

  // Fall back on using the hint.
  XClassHint class_hint;
  status = XGetClassHint(display, window, &class_hint);
  if (status == 0) {
    // No hints, assume this is a normal application window.
    return false;
  }

  bool is_desktop = strcmp("gnome-panel", class_hint.res_name) == 0 ||
                    strcmp("desktop_window", class_hint.res_name) == 0;

  XFree(class_hint.res_name);
  XFree(class_hint.res_class);
  return is_desktop;
}

Window get_app_window(Display *display, Window window, Atom wm_state) {
  Atom actual_type;
  int actual_format;
  unsigned long bytes_after;
  unsigned long size;
  unsigned char *data = NULL;
  int status = XGetWindowProperty(display, window, wm_state, 0L, ~0L, False,
                                  AnyPropertyType, &actual_type, &actual_format,
                                  &size, &bytes_after, &data);

  CARD32 state = WithdrawnState;
  if (status == Success && size > 0) {
    state = *(CARD32 *)data;
    XFree(data);
  }

  if (state == NormalState) {
    // Window has WM_STATE==NormalState. Return it.
    return window;
  } else if (state == IconicState) {
    // Window is in minimized. Skip it.
    return 0;
  }

  // If the window is in WithdrawnState then look at all of its children.
  Window root, parent;
  Window *children;
  unsigned int num_children;
  if (!XQueryTree(display, window, &root, &parent, &children, &num_children)) {

    fprintf(stderr, "Failed to query for child windows although window"
                    "does not have a valid WM_STATE.\n");
    return 0;
  }

  Window app_window = 0;
  for (unsigned int i = 0; i < num_children; ++i) {
    app_window = get_app_window(display, children[i], wm_state);
    if (app_window)
      break;
  }

  if (children)
    XFree(children);
  return app_window;
}

int get_window_rect(Display *display, Window window, WindowRect *rect) {
  XWindowAttributes attr;
  if (!XGetWindowAttributes(display, window, &attr)) {
    fprintf(stderr, "Failed to get window attributes\n");
    return 1;
  }
  rect->x = attr.x;
  rect->y = attr.y;
  rect->width = attr.width;
  rect->height = attr.height;

  int offset_x = 0;
  int offset_y = 0;
  if (!XTranslateCoordinates(display, window, attr.root, rect->x, rect->y,
                             &offset_x, &offset_y, &window)) {
    fprintf(stderr, "Failed to translate coordinates\n");
    return 1;
  }
  rect->x += offset_x;
  rect->y += offset_y;
  return 0;
}

int get_window_title(Display *display, Window window, char **title) {
  int status;
  bool nok = 1;
  XTextProperty window_name;
  window_name.value = nullptr;
  if (window) {
    status = XGetWMName(display, window, &window_name);
    if (status && window_name.value && window_name.nitems) {
      int cnt;
      char **list = nullptr;
      status = Xutf8TextPropertyToTextList(display, &window_name, &list, &cnt);
      if (status >= Success && cnt && *list) {
        if (cnt > 1) {
          fprintf(stderr,
                  "Window has %d text properties, only using the first "
                  "one.\n",
                  cnt);
        }
        char *dst = (char *)malloc(strlen(*list) + 1);
        strcpy(dst, *list);
        *title = dst;
        nok = 0;
      }
      if (list) {
        XFreeStringList(list);
      }
    }
    if (window_name.value) {
      XFree(window_name.value);
    }
  }

  return nok;
}
