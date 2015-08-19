/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_THREADS
#define _QAULLIB_THREADS

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#ifdef _WIN32
	#include "win32/pthread.h"
#else
	#include <pthread.h>
#endif // _WIN32

// --------------------------------------------------------------------------
// mutexes & locks

pthread_mutex_t qaullib_mutex_userLL;
pthread_mutex_t qaullib_mutex_topoLL;
pthread_mutex_t qaullib_mutex_appeventLL;
pthread_mutex_t qaullib_mutex_msgLL;
pthread_mutex_t qaullib_mutex_fileLL;
pthread_mutex_t qaullib_mutex_filediscoveryLL;
pthread_mutex_t qaullib_mutex_DhcpLL;
pthread_mutex_t qaullib_mutex_wget;


// --------------------------------------------------------------------------

typedef void * (*qaullib_thread_func_t)(void *);
int qaullib_pthread_start(qaullib_thread_func_t func, void *param);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _QAULLIB_THREADS
