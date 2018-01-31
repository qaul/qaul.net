/**
 * generates a portable pthread API for windows
 *
 * information from mongoose source code
 */

#include <windows.h> // fprintf sprintf stderr
#include <stdio.h>   // fprintf sprintf stderr
#include "pthread.h"


#ifdef _WIN32
int pthread_mutex_init(pthread_mutex_t *mutex, void *unused) 
{
  unused = NULL;
  *mutex = CreateMutex(NULL, FALSE, NULL);
  return *mutex == NULL ? -1 : 0;
}

int pthread_mutex_destroy(pthread_mutex_t *mutex) 
{
  return CloseHandle(*mutex) == 0 ? -1 : 0;
}

int pthread_mutex_lock(pthread_mutex_t *mutex) 
{
  return WaitForSingleObject(*mutex, INFINITE) == WAIT_OBJECT_0? 0 : -1;
}

int pthread_mutex_unlock(pthread_mutex_t *mutex) 
{
  return ReleaseMutex(*mutex) == 0 ? -1 : 0;
}

int pthread_cond_init(pthread_cond_t *cv, const void *unused) 
{
  unused = NULL;
  cv->signal = CreateEvent(NULL, FALSE, FALSE, NULL);
  cv->broadcast = CreateEvent(NULL, TRUE, FALSE, NULL);
  return cv->signal != NULL && cv->broadcast != NULL ? 0 : -1;
}

int pthread_cond_wait(pthread_cond_t *cv, pthread_mutex_t *mutex) 
{
  HANDLE handles[] = {cv->signal, cv->broadcast};
  ReleaseMutex(*mutex);
  WaitForMultipleObjects(2, handles, FALSE, INFINITE);
  return ReleaseMutex(*mutex) == 0 ? -1 : 0;
}

int pthread_cond_signal(pthread_cond_t *cv) 
{
  return SetEvent(cv->signal) == 0 ? -1 : 0;
}

int pthread_cond_broadcast(pthread_cond_t *cv) 
{
  // Implementation with PulseEvent() has race condition, see
  // http://www.cs.wustl.edu/~schmidt/win32-cv-1.html
  return PulseEvent(cv->broadcast) == 0 ? -1 : 0;
}

int pthread_cond_destroy(pthread_cond_t *cv) 
{
  return CloseHandle(cv->signal) && CloseHandle(cv->broadcast) ? 0 : -1;
}

pthread_t pthread_self(void) 
{
  return GetCurrentThreadId();
}

#endif // _WIN32
