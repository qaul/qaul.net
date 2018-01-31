/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _TESTS_H
#define _TESTS_H

/***
 * macros and defines used by tests only
 ***/

#include <stdio.h>
#include "qaul/utils/defines.h"
#define FAIL(M) printf(M); return(QL_ERROR);

#endif // _TESTS_H
