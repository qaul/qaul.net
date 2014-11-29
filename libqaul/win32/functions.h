#ifndef _WIN32_FUNCTIONS_H
#define	_WIN32_FUNCTIONS_H

#include <winsock2.h> // struct in_addr
#include <ws2tcpip.h>

void sleep(unsigned int Sec);
int inet_aton(const char *AddrStr, struct in_addr *Addr);
char * StrError(unsigned int ErrNo);
void WinSockPError(char *Str);
static char * inet_ntop4(const unsigned char *src, char *dst, int size);
static char * inet_ntop6(const unsigned char *src, char *dst, int size);
char * inet_ntop(int af, const void *src, char *dst, int size);
char *strscpy(char *d, const char *s, size_t len);
int inet_pton(int af, const char *src, void *dst);
char *inet_ntop(int af, const void *src, char *dst, int size);

#endif
