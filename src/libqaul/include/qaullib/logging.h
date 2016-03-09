#ifndef _LOGGING_H
#define _LOGGING_H

#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/time.h>
#include <time.h>

#include "QaulConfig.h"

#ifdef __cplusplus
extern "C" {
#endif

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

#define ql_levelname(M) LOG_LEVEL_NAMES[M]

#ifdef NDEBUG
#define ql_debug(M, ...)
#else
#define ql_debug(M, ...) ql_logline(LOG_DEBUG, 0, M, ##__VA_ARGS__)
#endif

#define ql_clean_errno() (errno == 0 ? "None" : strerror(errno))

#define ql_logline(L, E, M, ...) { \
	char date[20]; \
	struct timeval tv; \
	gettimeofday(&tv, NULL); \
	strftime(date, sizeof(date) / sizeof(*date), "%Y-%m-%dT%H:%M:%S", gmtime(&tv.tv_sec)); \
	if ( E ) { \
		fprintf(stderr, "%s.%03dZ [%s] (%s:%d: errno: %s) " M "\n", &date[0], (tv.tv_usec/1000), ql_levelname( L ), __FILE__, __LINE__, ql_clean_errno(), ##__VA_ARGS__); \
	} else { \
		fprintf(stderr, "%s.%03dZ [%s] (%s:%d) " M "\n", &date[0], (tv.tv_usec/1000), ql_levelname( L ), __FILE__, __LINE__, ##__VA_ARGS__); \
	} \
} \

#define ql_log_err(M, ...) if ( LOG_ERROR <= loglevel ) ql_logline(LOG_ERROR, 1, M , ##__VA_ARGS__)

#define ql_log_warn(M, ...) if ( LOG_WARN <= loglevel ) ql_logline(LOG_WARN, 1, M , ##__VA_ARGS__)

#define ql_log_info(M, ...) if ( LOG_INFO <= loglevel ) ql_logline(LOG_INFO, 0, M , ##__VA_ARGS__)

#define ql_check(A, M, ...) if(!(A)) { ql_log_err(M, ##__VA_ARGS__); errno=0; goto ql_error; }

#define ql_sentinel(M, ...)  { ql_log_err(M, ##__VA_ARGS__); errno=0; goto ql_error; }

#define ql_check_mem(A) ql_check((A), "Out of memory.")

#define ql_check_debug(A, M, ...) if(!(A)) { ql_debug(M, ##__VA_ARGS__); errno=0; goto error; }

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _LOGGING_H
