/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _OLSRD_QAUL_NET
#define _OLSRD_QAUL_NET


#include "olsr.h"
#include "olsr_types.h"
#include "link_set.h"
#include "qaul_messages.h"

/**
 * network organisation
 */


/**
 * send network topology information via ipc
 * for user link quality display
 */
void qaul_net_topo2gui(void);

/**
 * send network topology information via ipc
 * for mesh network display
 */
void qaul_net_meshtopo2gui(void);


#endif
