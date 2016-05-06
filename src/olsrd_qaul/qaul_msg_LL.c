/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <string.h>
#include "olsr.h"


#include "qaul_msg_LL.h"


struct qaul_msg_LL_item {
	struct qaul_msg_LL_item *next;           // next node
	struct qaul_msg_LL_item *prev;           // previous node
	union olsr_ip_addr ip;                    // originator address
	uint16_t           seqno;				  // olsr package sequence number
	time_t             time;                  // time when first received
};

void Qaul_Msg_LL_Add (uint16_t seqno, union olsr_ip_addr *ip);
void Qaul_Msg_LL_Delete_Item (struct qaul_msg_LL_item *item);


static int qaul_msg_LL_count;
static struct qaul_msg_LL_item root_item;


void Qaul_Msg_LL_Init (void)
{
	qaul_msg_LL_count = 0;
	root_item.next = &root_item;
	root_item.prev = &root_item;
}

int  Qaul_Msg_LL_IsDuplicate (uint16_t seqno, union olsr_ip_addr *ip)
{
	struct qaul_msg_LL_item *myitem = &root_item;

	// check if message already exists
	while(myitem->next != &root_item)
	{
		myitem = myitem->next;

		if(myitem->seqno == seqno)
		{
			if(memcmp(&myitem->ip, ip, olsr_cnf->ipsize) == 0)
			{
				OLSR_PRINTF(1, "item exists\n");
				return 1;
			}
		}
	}

	// it doesn't exist yet: add it to list
	Qaul_Msg_LL_Add (seqno, ip);

	return 0;
}


void Qaul_Msg_LL_Add (uint16_t seqno, union olsr_ip_addr *ip)
{
	// create new item
	struct qaul_msg_LL_item *new_item;
	new_item = (struct qaul_msg_LL_item *)malloc(sizeof(struct qaul_msg_LL_item));

	OLSR_PRINTF(1, "add item\n");

	// fill in content
	new_item->time = time(NULL);
	new_item->seqno = seqno;
	memcpy((char *)&new_item->ip, ip, sizeof(union olsr_ip_addr));

	// create links
	new_item->prev = &root_item;
	new_item->next = root_item.next;
	root_item.next = new_item;

	qaul_msg_LL_count++;
}


void Qaul_Msg_LL_Delete_Item (struct qaul_msg_LL_item *item)
{
	OLSR_PRINTF(1, "delete item\n");

	item->prev->next = item->next;
	item->next->prev = item->prev;
	qaul_msg_LL_count--;

	free(item);
}


/**
 * delete all items older than 30 seconds
 */
void Qaul_Msg_LL_Clean (void *foo __attribute__ ((unused)))
{
	time_t mytime = time(NULL) -30;
	struct qaul_msg_LL_item *myitem = &root_item;

	while(myitem->prev != &root_item)
	{
		myitem = myitem->prev;

		if(myitem->time < mytime)
			Qaul_Msg_LL_Delete_Item (myitem);
		else
			break;
	}
}




