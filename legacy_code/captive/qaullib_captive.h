/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_CAPTIVE
#define _QAULLIB_CAPTIVE

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#define MAX_ADDRESS_SIZE 4

#include <stdio.h>      // for printf
#include <stdlib.h>
#include <string.h>     // for string and memset etc


void Qaullib_Captive_CreateIP(char *ip);
int Qaullib_Captive_IpExists(char *ip);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _QAULLIB_CAPTIVE
