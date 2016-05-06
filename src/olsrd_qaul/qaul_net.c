/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "tc_set.h"

#include "qaul_net.h"
#include "qaul_ipc.h"


void qaul_net_topo2gui(void)
{
	struct tc_entry *tc;
	union olsr_message *m;
	char  buffer[512];
	float linkcost;

	// create new message
	m = (union olsr_message *)buffer;
	// fill message
	m->v4.olsr_msgtype = QAUL_IPCTOPO_MESSAGE_TYPE;
	m->v4.olsr_msgsize = htons(sizeof(struct qaul_node_msg) + sizeof(struct olsrmsg));

	// loop through the topology and add it to the message
	OLSR_FOR_ALL_TC_ENTRIES(tc)
	{
		struct tc_edge_entry *tc_edge;
		OLSR_FOR_ALL_TC_EDGE_ENTRIES(tc, tc_edge)
		{
		  if (tc_edge->edge_inv)
		  {
			  struct qaul_node_msg *node = (struct qaul_node_msg *) &m->v4.message;
			  // fill the message
			  linkcost = (float)tc_edge->cost / 1024;
			  memcpy(&node->ip, &tc_edge->T_dest_addr, sizeof(union olsr_ip_addr));
			  memcpy(&node->gateway, &tc->addr, sizeof(union olsr_ip_addr));
			  memcpy(&node->lq, &linkcost, sizeof(float));

			  // send the message
			  qaul_ipc_msg2gui(m);
		  }
		} OLSR_FOR_ALL_TC_EDGE_ENTRIES_END(tc, tc_edge);
	} OLSR_FOR_ALL_TC_ENTRIES_END(tc);
}


void qaul_net_meshtopo2gui(void)
{
	struct tc_entry *tc;
	struct tc_edge_entry *tc_edge;
	union olsr_message *m;
	char  buffer[512];
	float linkcost;
	struct qaul_ipc_msg *ipc_msg;

	// create new message
	m = (union olsr_message *)buffer;
	// fill message
	m->v4.olsr_msgtype = QAUL_IPCMESHTOPO_MESSAGE_TYPE;
	m->v4.olsr_msgsize = htons(sizeof(struct qaul_node_msg) + sizeof(struct olsrmsg));

	// loop through the topology and add it to the message
	OLSR_FOR_ALL_TC_ENTRIES(tc)
	{
		OLSR_FOR_ALL_TC_EDGE_ENTRIES(tc, tc_edge)
		{
		  if (tc_edge->edge_inv)
		  {
			  struct qaul_node_msg *node = (struct qaul_node_msg *) &m->v4.message;
			  // fill the message
			  linkcost = (float)tc_edge->cost / 1024;
			  memcpy(&node->ip, &tc->addr, sizeof(union olsr_ip_addr));
			  memcpy(&node->gateway, &tc_edge->T_dest_addr, sizeof(union olsr_ip_addr));
			  memcpy(&node->lq, &linkcost, sizeof(float));

			  // send the message
			  qaul_ipc_msg2gui(m);
		  }
		} OLSR_FOR_ALL_TC_EDGE_ENTRIES_END(tc, tc_edge);
	} OLSR_FOR_ALL_TC_ENTRIES_END(tc);

	// send finished message
	ipc_msg = (struct qaul_ipc_msg *) &m->v4.message;
	m->v4.olsr_msgtype = QAUL_IPCCOM_MESSAGE_TYPE;
	m->v4.olsr_msgsize = htons(sizeof(struct qaul_ipc_msg) + sizeof(struct olsrmsg));
	ipc_msg->type = QAUL_IPCCOM_MESHTOPO_SENT;
	qaul_ipc_msg2gui(m);
}


