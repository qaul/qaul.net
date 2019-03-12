/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _DEPRECATED_H
#define _DEPRECATED_H

#ifdef __GNUC__
#define DEPRECATED __attribute__((deprecated))
#elif defined(_MSC_VER)
#define DEPRECATED __declspec(deprecated)
#else
#pragma message("WARNING: You need to implement DEPRECATED for this compiler")
#define DEPRECATED
#endif


//#define DEPRECATED(M) __attribute__((deprecated(M)))

#endif // _DEPRECATED_H
