/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _OLSRD_QAUL_IPC
#define _OLSRD_QAUL_IPC

#include "olsrd_plugin.h"
#include "plugin_util.h"
#include "qaul_messages.h"

// ------------------------------------------------------------
// defines



// ------------------------------------------------------------
// structs & variables
extern int qaul_ipc_port;


// ------------------------------------------------------------
// functions
int  qaul_ipc_init(void);
int  qaul_ipc_shutdown(void);
void qaul_ipc_accept(int fd, void *, unsigned int);
bool qaul_ipc_check_allowed_ip(const union olsr_ip_addr *addr);
void qaul_ipc_msg2gui(union olsr_message *m);
void qaul_ipc_receive(void *foo __attribute__ ((unused)));
void qaul_ipc_evaluate(union olsr_message *msg);
void qaul_ipc_evaluate_chat(union olsr_message *msg);
void Qaullib_IpcEvaluateUserhello(union olsr_message *msg);
void Qaullib_IpcEvaluateFilediscover(union olsr_message *msg);
void Qaullib_IpcEvaluateExediscover(union olsr_message *msg);
void qaul_ipc_evaluate_com(union olsr_message *msg);


#endif
