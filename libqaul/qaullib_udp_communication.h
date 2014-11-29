/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_UDP_COMMUNICATION
#define _QAULLIB_UDP_COMMUNICATION

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include <stdio.h>      // for printf
#include <stdlib.h>
#include <string.h>     // for string and memset etc


int qaul_UDP_socket;
int qaul_UDP_started;

/**
 * Send the message to the requester
 */
void Qaullib_UDP_SendFileavailableMsg(struct qaul_fileavailable_msg *msg, union olsr_ip_addr *ip);

/**
 * Send the message to the requester
 */
void Qaullib_UDP_SendExeavailableMsg(struct qaul_exeavailable_msg *msg, union olsr_ip_addr *ip);

/**
 * Check for incoming messages
 */
void Qaullib_UDP_CheckSocket(void);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _QAULLIB_UDP_COMMUNICATION
