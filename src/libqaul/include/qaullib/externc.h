/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _EXTERNC_H
#define _EXTERNC_H

#ifdef __cplusplus
#define BEGIN_EXTERN_C  extern "C" {
#define END_EXTERN_C    }
#else /* __cplusplus */
#define BEGIN_EXTERN_C
#define END_EXTERN_C
#endif /* __cplusplus */

#endif // _EXTERNC_H
