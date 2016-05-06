/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include <olsr_protocol.h>
#include "net_olsr.h"
#include "scheduler.h"
#include "parser.h" // olsr_parser_add

#include "olsrd_plugin.h"
#include "qaul_config.h"
#include "qaul_msg.h"
#include "qaul_msg_LL.h"
#include "qaul_ipc.h"

// ------------------------------------------------------------
// variables
// ------------------------------------------------------------
int qaul_chat_counter = 0;


// ------------------------------------------------------------
// generic message procession
// ------------------------------------------------------------
bool qaul_qaulmsg_parser(union olsr_message *m, struct interface_olsr *in_if __attribute__ ((unused)), union olsr_ip_addr *ipaddr __attribute__ ((unused)))
{
	union olsr_ip_addr originator;
	uint16_t seqno;

	OLSR_PRINTF(1, "[QAUL] qaul_qaulmsg_parser message arrived\n");

	// Fetch the originator & the id of the messsage
	if (olsr_cnf->ip_version == AF_INET)
	{
		OLSR_PRINTF(1, "[QAUL] message type: %i\n", m->v4.olsr_msgtype);
		memcpy(&originator, &m->v4.originator, olsr_cnf->ipsize);
		seqno = ntohs(m->v4.seqno);
	}
	else
	{
		memcpy(&originator, &m->v6.originator, olsr_cnf->ipsize);
		seqno = ntohs(m->v6.seqno);
	}

	// Check if message originated from this node
	if (ipequal(&originator, &olsr_cnf->main_addr))
		return false;

	// Check for duplicates
	if (Qaul_Msg_LL_IsDuplicate (seqno, &originator))
		return false;

	// send it to the qaul application via ipc
	qaul_ipc_msg2gui(m);

	OLSR_PRINTF(1, "[QAUL] message processed\n");

	// Forward the message
	return true;
}

// ------------------------------------------------------------
void qaul_qaulmsg_send_all(union olsr_message *mymsg)
{
	// send buffer: huge
	char buffer[1024];
	union olsr_message *message = (union olsr_message *)buffer;
	struct interface_olsr *ifn;
	int my_timeout, mysize;

	my_timeout = 30;

	OLSR_PRINTF(1, "[QAUL] qaul_qaulmsg_send_all\n");

	// fill message
	if (olsr_cnf->ip_version == AF_INET)
	{
		// IPv4
		message->v4.olsr_msgtype = mymsg->v4.olsr_msgtype;
		message->v4.olsr_vtime = reltime_to_me(my_timeout * MSEC_PER_SEC);
		memcpy(&message->v4.originator, &olsr_cnf->main_addr, olsr_cnf->ipsize);
		message->v4.ttl = MAX_TTL;
		message->v4.hopcnt = 0;
		message->v4.seqno = htons(get_msg_seqno());
		mysize = ntohs(mymsg->v4.olsr_msgsize);
		memcpy(&message->v4.message, &mymsg->v4.message, mysize -sizeof(struct olsrmsg));
		message->v4.olsr_msgsize = mymsg->v4.olsr_msgsize;
	}
	else
	{
		// IPv6
		message->v6.olsr_msgtype = mymsg->v6.olsr_msgtype;
		message->v6.olsr_vtime = reltime_to_me(my_timeout * MSEC_PER_SEC);
		memcpy(&message->v6.originator, &olsr_cnf->main_addr, olsr_cnf->ipsize);
		message->v6.ttl = MAX_TTL;
		message->v6.hopcnt = 0;
		message->v6.seqno = htons(get_msg_seqno());
		mysize = ntohs(mymsg->v6.olsr_msgsize);
		memcpy(&message->v6.message, &mymsg->v6.message, mysize -sizeof(struct olsrmsg6));
		message->v6.olsr_msgsize = mymsg->v6.olsr_msgsize;
	}
	//looping trough interfaces
	for (ifn = ifnet; ifn; ifn = ifn->int_next)
	{
		OLSR_PRINTF(1, "QAUL: Generating packet - [%s]\n", ifn->int_name);

		if (net_outbuffer_push(ifn, message, mysize) != mysize)
		{
			// send data and try again
			net_output(ifn);
			if (net_outbuffer_push(ifn, message, mysize) != mysize)
			{
				OLSR_PRINTF(1, "QAUL: could not send on interface: %s\n", ifn->int_name);
			}
		}
	}
}

// ------------------------------------------------------------
// initialize the system
// ------------------------------------------------------------

int qaul_msg_init(void)
{
	// init linked list
	Qaul_Msg_LL_Init ();

	// register message parser
	olsr_parser_add_function(&qaul_qaulmsg_parser, QAUL_CHAT_PARSER_TYPE);
	olsr_parser_add_function(&qaul_qaulmsg_parser, QAUL_USERHELLO_PARSER_TYPE);
	olsr_parser_add_function(&qaul_qaulmsg_parser, QAUL_FILEDISCOVER_PARSER_TYPE);
	olsr_parser_add_function(&qaul_qaulmsg_parser, QAUL_EXEDISCOVER_PARSER_TYPE);

	// schedule message cleaning
	olsr_start_timer(3 * MSEC_PER_SEC, 0, OLSR_TIMER_PERIODIC, &Qaul_Msg_LL_Clean, NULL, 0);

	// automatically send a chat message every 3 seconds
	//olsr_start_timer(3 * MSEC_PER_SEC, 0, OLSR_TIMER_PERIODIC, &qaul_chat_autosend, NULL, 0);

	return 1;
}


// ------------------------------------------------------------
// Test Messages
// ------------------------------------------------------------
// Send every 3 seconds a message

/*
void qaul_chat_autosend(void *foo __attribute__ ((unused)))
{
	struct qaul_chat_msg chat_message;
	//olsr_printf(1, "[QAUL] send test: %i\n", qaul_chat_counter);
	strcpy(chat_message.name, "[qaul test]");
	sprintf(chat_message.msg, "%i", qaul_chat_counter);
	chat_send_all(&chat_message);
	qaul_chat_counter++;
	return;
}
*/

