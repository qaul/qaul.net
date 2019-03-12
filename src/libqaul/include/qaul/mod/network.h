/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAUL_QLNET_H
#define _QAUL_QLNET_H

#include <qaul/mod/structures.h>
#include <qaul/error.h>


/**
 * Turn a qaul IP address into a string
 * @return
 */
char *ql_net_ip_to_string(union ql_ip ip);


/**
 * Turn a string into an ip address.
 *
 * If in the pattern <num>.<num>.<num>.<num> it selects
 * an ipv4 address. If of pattern *:*:*:*:*:*:*:*:*:*:*:*
 * it selects an ipv6 address.
 *
 * @param str
 * @return
 */
ql_ip ql_net_string_to_ip(const char *str);

#endif //QAUL_QLNET_H
