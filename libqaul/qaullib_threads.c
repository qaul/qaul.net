/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"

/**
 * start a pthread
 */
#ifdef _WIN32

int qaullib_pthread_start(qaullib_thread_func_t func, void *param) 
{
  HANDLE hThread;

  hThread = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE) func, param, 0,
                         NULL);
  if (hThread != NULL) {
    (void) CloseHandle(hThread);
  }

  return hThread == NULL ? -1 : 0;
}

#else

int qaullib_pthread_start(qaullib_thread_func_t func, void *param) 
{
  pthread_t thread_id;
  pthread_attr_t attr;
  int retval;

  (void) pthread_attr_init(&attr);
  (void) pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

  if ((retval = pthread_create(&thread_id, &attr, func, param)) != 0) {
    printf("%s: %s", __func__, strerror(retval));
  }

  return retval;
}

#endif // _WIN32
