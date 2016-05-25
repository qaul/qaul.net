/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * Logging macros for qaul.net.
 *
 * The following log levels:
 * 	DEBUG
 * 	INFO
 * 	WARN
 * 	ERROR
 *
 * The logging messages can be excluded during compile time and if present
 * switched on at program startup.
 * Default log level is WARN.
 *
 * Use the logging macros as follows:
 *
 * Ql_log_debug("My debug log message");
 * Ql_log_info("My info log message");
 * Ql_log_warn("My warn log message");
 * Ql_log_error("My error log message");
 */

#ifndef _LOGGING_H
#define _LOGGING_H

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/time.h>
#include <time.h>
#include "QaulConfig.h"

enum elog_level {
    LOG_NONE = 0,
    LOG_ERROR,
    LOG_WARN,
    LOG_INFO,
    LOG_DEBUG,
};

typedef enum elog_level loglevel_t;

loglevel_t loglevel = QAUL_DEFAULT_LOGLEVEL;

const char *const LOG_LEVEL_NAMES[] = {
    "NONE",
    "ERROR",
    "WARN",
    "INFO",
    "DEBUG"
};

#define Ql_levelname(M) LOG_LEVEL_NAMES[M]

#ifdef NDEBUG
#define Ql_log_debug(M, ...)
#else
#define Ql_log_debug(M, ...) Ql_logline(LOG_DEBUG, 0, M, ##__VA_ARGS__)
#endif

#define Ql_clean_errno() (errno == 0 ? "None" : strerror(errno))

#define Ql_logline(L, E, M, ...) { \
	char date[20]; \
	struct timeval tv; \
	gettimeofday(&tv, NULL); \
	strftime(date, sizeof(date) / sizeof(*date), "%Y-%m-%dT%H:%M:%S", gmtime(&tv.tv_sec)); \
	if ( E ) { \
		fprintf(stderr, "%s.%03dZ [%s] (%s:%d: errno: %s) " M "\n", &date[0], (tv.tv_usec/1000), Ql_levelname( L ), __FILE__, __LINE__, Ql_clean_errno(), ##__VA_ARGS__); \
	} else { \
		fprintf(stderr, "%s.%03dZ [%s] (%s:%d) " M "\n", &date[0], (tv.tv_usec/1000), Ql_levelname( L ), __FILE__, __LINE__, ##__VA_ARGS__); \
	} \
} \

#define Ql_log_err(M, ...) if ( LOG_ERROR <= loglevel ) Ql_logline(LOG_ERROR, 1, M , ##__VA_ARGS__)

#define Ql_log_warn(M, ...) if ( LOG_WARN <= loglevel ) Ql_logline(LOG_WARN, 1, M , ##__VA_ARGS__)

#define Ql_log_info(M, ...) if ( LOG_INFO <= loglevel ) Ql_logline(LOG_INFO, 0, M , ##__VA_ARGS__)

#define Ql_check(A, M, ...) if(!(A)) { Ql_log_err(M, ##__VA_ARGS__); errno=0; goto Ql_error; }

#define Ql_sentinel(M, ...)  { Ql_log_err(M, ##__VA_ARGS__); errno=0; goto Ql_error; }

#define Ql_check_mem(A) Ql_check((A), "Out of memory.")

#define Ql_check_debug(A, M, ...) if(!(A)) { ql_debug(M, ##__VA_ARGS__); errno=0; goto Ql_error; }


#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _LOGGING_H
