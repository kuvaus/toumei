#ifndef TOUMEI_H_
#define TOUMEI_H_

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum ToumeiMode {
  Transparent,
  Opaque,
} ToumeiMode;

typedef enum ToumeiResult {
  Ok,
  ErrUnsupportedPlatform,
  ErrWindowNotFound,
  ErrOther,
} ToumeiResult;

enum ToumeiResult detect_tray_transparency(enum ToumeiMode *mode);

#endif  /* TOUMEI_H_ */
