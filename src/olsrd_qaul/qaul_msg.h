/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _OLSRD_QAUL_CHAT
#define _OLSRD_QAUL_CHAT


#include "olsr.h"
#include "olsr_types.h"
#include "link_set.h"
#include "qaul_messages.h"


// -------------------------------------------------------
int  qaul_msg_init(void);
void qaul_chat_autosend(void *foo __attribute__ ((unused)));

bool qaul_qaulmsg_parser(union olsr_message *m, struct interface_olsr *in_if __attribute__ ((unused)), union olsr_ip_addr *ipaddr __attribute__ ((unused)));
void qaul_qaulmsg_send_all(union olsr_message *mymsg);


#endif
