
#ifndef _WIFI_CONFIG_LOG_H
#define _WIFI_CONFIG_LOG_H

/**
 * Write a formatted string to the Android syslog.
 *
 * @param level is the prioirty of the log message.
 * @param format is the formatted log message string.
 */
void android_syslog(int level, const char *format, ...);

#endif /* _WIFI_CONFIG_LOG_H */
