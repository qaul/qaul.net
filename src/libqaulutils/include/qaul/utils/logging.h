/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * Logging macros for qaul.net.
 *
 * The following log levels exist:
 * 	NONE
 * 	ERROR
 * 	WARN
 * 	INFO
 * 	DEBUG
 *
 * Default log level when compiling is DEBUG.
 * The messages can be included/excluded during compile time by setting a log level.
 *
 * Cmake build options:
 * -DQAUL_LOG_ENABLE=NO             (Disable all logging. Default is YES.)
 * -DQAUL_LOG_DEFAULTLEVEL=LOGLEVEL (Set log level to one of the available log levels. Default log level is DEBUG.)
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


/**
 * The following log levels exist:
 *
 * NONE  => LOG_NONE  => Nothing will be logged at all.
 * ERROR => LOG_ERROR => Log as error.
 *                       Only ERROR will be logged.
 * WARN  => LOG_WARN  => Log as warning.
 *                       ERROR and WARN will be logged.
 * INFO  => LOG_INFO  => Log as information.
 *                       ERROR, WARN and INFO will be logged.
 * DEBUG => LOG_DEBUG => Log as debug message.
 *                       ERROR, WARN, INFO and DEBUG will be logged.
 */
enum elog_level {
    LOG_NONE = 0,
    LOG_ERROR,
    LOG_WARN,
    LOG_INFO,
    LOG_DEBUG,
};

typedef enum elog_level loglevel_t;

#ifdef LOGGER_IMPLEMENTATION
loglevel_t loglevel = QAUL_DEFAULT_LOGLEVEL;

const char *const LOG_LEVEL_NAMES[] = {
    "NONE",
    "ERROR",
    "WARN",
    "INFO",
    "DEBUG"
};
#else
extern loglevel_t loglevel;
extern const char *const LOG_LEVEL_NAMES[];
#endif

#define Ql_levelname(M) LOG_LEVEL_NAMES[M]

/**
 * Log message as debug message.
 *
 * This message will be shown, when the program is compiled with log level
 * DEBUG.
 *
 * usage:
 * 	Ql_log_debug("My debug message");
 * 	Ql_log_debug("My %s message number %i", "debug", 2);
 */
#ifdef NDEBUG
#define Ql_log_debug(M, ...)
#else
#define Ql_log_debug(M, ...) Ql_logline(LOG_DEBUG, 0, M, ##__VA_ARGS__)
#endif


/**
 * Log message as error message.
 *
 * This message will be shown, when the program is compiled with either of these log levels:
 * ERROR, WARN, INFO or DEBUG.
 *
 * usage:
 * 	Ql_log_error("My Error message");
 * 	Ql_log_error("My %s message number %i", "error", 2);
 */
#define Ql_log_error(M, ...) if ( LOG_ERROR <= loglevel ) Ql_logline(LOG_ERROR, 1, M , ##__VA_ARGS__)


/**
 * Log message as error message.
 *
 * This message will be shown, when the program is compiled with either of these log levels:
 * WARN, INFO or DEBUG.
 *
 * usage:
 * 	Ql_log_warn("My Error message");
 * 	Ql_log_warn("My %s message number %i", "warn", 2);
 */
#define Ql_log_warn(M, ...) if ( LOG_WARN <= loglevel ) Ql_logline(LOG_WARN, 1, M , ##__VA_ARGS__)


/**
 * Log message as error message.
 *
 * This message will be shown, when the program is compiled with either of these log levels:
 * INFO or DEBUG.
 *
 * usage:
 * 	Ql_log_info("My Error message");
 * 	Ql_log_info("My %s message number %i", "info", 2);
 */
#define Ql_log_info(M, ...) if ( LOG_INFO <= loglevel ) Ql_logline(LOG_INFO, 0, M , ##__VA_ARGS__)


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

#define Ql_check(A, M, ...) if(!(A)) { Ql_log_error(M, ##__VA_ARGS__); errno=0; goto Ql_error; }

#define Ql_sentinel(M, ...)  { Ql_log_error(M, ##__VA_ARGS__); errno=0; goto Ql_error; }

#define Ql_check_mem(A) Ql_check((A), "Out of memory.")

#define Ql_check_debug(A, M, ...) if(!(A)) { ql_log_debug(M, ##__VA_ARGS__); errno=0; goto Ql_error; }

loglevel_t getLogLevel(void);
loglevel_t setLogLevel(loglevel_t newloglevel);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _LOGGING_H
