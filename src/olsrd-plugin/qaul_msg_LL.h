/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAUL_MSG_LL
#define _QAUL_MSG_LL

#include "olsr_types.h"

void Qaul_Msg_LL_Init (void);
int  Qaul_Msg_LL_IsDuplicate (uint16_t seqno, union olsr_ip_addr *ip);
void Qaul_Msg_LL_Clean (void *foo __attribute__ ((unused)));

#endif
