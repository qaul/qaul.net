#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#ifdef ANDROID
#include <android/log.h>
#endif

void android_syslog(int level, const char *format, ...)
{
  va_list arglist;
  va_start(arglist, format);
  __android_log_vprint(level, "wificonfig", format, arglist);
  va_end(arglist);

  return;
}
