/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _OLSRD_TYPES
#define _OLSRD_TYPES

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#ifdef WIN32
#include <winsock2.h>
#include <ws2tcpip.h>

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef signed char int8_t;
typedef signed short int16_t;
typedef signed int int32_t;
#else
#ifndef INVALID_SOCKET
	#define INVALID_SOCKET (-1)
#endif
#include <sys/types.h>
#include <inttypes.h>
#endif

#include "olsrd/mantissa.h"
#include "olsrd/olsr_types.h"

/**
 * OLSRD Datatypes
 */
#ifdef WIN32
typedef unsigned char olsr_u8_t;
typedef unsigned short olsr_u16_t;
typedef unsigned int olsr_u32_t;
typedef char olsr_8_t;
typedef short olsr_16_t;
typedef int olsr_32_t;
#else
typedef u_int8_t olsr_u8_t;
typedef u_int16_t olsr_u16_t;
typedef u_int32_t olsr_u32_t;
typedef int8_t olsr_8_t;
typedef int16_t olsr_16_t;
typedef int32_t olsr_32_t;
#endif

#undef  MAX
#define MAX(x,y)	(((x) > (y)) ? (x) : (y))

#define CLOSE(fd)  do { close(fd); (fd) = -1; } while (0)



/*
 * Multiple Interface Declaration message
 */

/*
 * Defined as struct for further expansion
 * For example: do we want to tell what type of interface
 * is associated whit each address?
 */
struct midaddr {
  olsr_u32_t addr;
};

struct midmsg {
  struct midaddr mid_addr[1];
};

/*
 * IPv6
 */

struct midaddr6 {
  struct in6_addr addr;
};

struct midmsg6 {
  struct midaddr6 mid_addr[1];
};

/*
 * Host and Network Association message
 */

struct hnapair {
  olsr_u32_t addr;
  olsr_u32_t netmask;
};

struct hnamsg {
  struct hnapair hna_net[1];
};

/*
 * IPv6
 */

struct hnapair6 {
  struct in6_addr addr;
  struct in6_addr netmask;
};

struct hnamsg6 {
  struct hnapair6 hna_net[1];
};


#ifdef __cplusplus
}
#endif // __cplusplus

#endif


