/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _DEFINES_H
#define _DEFINES_H

/***
 * boolean logic
 ***/
#include <stdbool.h>    // comments: non c99

#define QL_BOOL bool    // typedef int QL_BOOL
#define QL_TRUE true    // 1
#define QL_FALSE false  // 0

/***
 * process logic
 ***/
#include <stdlib.h>

#define QL_SUCCESS EXIT_SUCCESS
#define QL_ERROR EXIT_FAILURE

#endif // _DEFINES_H
