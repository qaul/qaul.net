/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "../qaullib_private.h"
#include "../qaullib_threads.h"
#include "whitelist.h"
#include "whitelist_LL.h"


// ------------------------------------------------------------
void ql_whitelist_add (union olsr_ip_addr *ip)
{
	struct qaul_whitelist_LL_item *item;

	if(qaul_whitelist_init < 1)
		return;

	printf("ql_whitelist_add\n");

	// check if entry exists
	if(Qaullib_Whitelist_LL_Find_ByIP(ip, &item))
	{
		printf("ql_whitelist_add IP already whitelisted\n");
		// update timestamp
		item->time = time(NULL);
	}
	else
	{
		printf("ql_whitelist_add add IP\n");
		// add list entry
		Qaullib_Whitelist_LL_Add(ip);
	}
}

// ------------------------------------------------------------
int ql_whitelist_check (union olsr_ip_addr *ip)
{
	struct qaul_whitelist_LL_item *item;

	if(qaul_whitelist_init < 1)
		return 0;

	return Qaullib_Whitelist_LL_Find_ByIP(ip, &item);
}

// ------------------------------------------------------------
int qaul_whitelist_check_hostname (const char* hostname, int len)
{
	if(len >= 9 && strncmp(hostname +len-9, "apple.com", 9) == 0)
		return 1;
	if(len >= 22 && strncmp(hostname +len-22, "akamaitechnologies.com", 22) == 0)
		return 1;
	if(len >= 14 && strncmp(hostname +len-14, "akamaiedge.net", 14) == 0)
		return 1;
	if(len >= 11 && strncmp(hostname +len-11, "edgekey.net", 11) == 0)
		return 1;
	if(len >= 17 && strncmp(hostname +len-17, "thinkdifferent.us", 17) == 0)
		return 1;
	if(len >= 10 && strncmp(hostname +len-10, "airport.us", 10) == 0)
		return 1;
	if(len >= 10 && strncmp(hostname +len-10, "ibook.info", 10) == 0)
		return 1;
	if(len >= 11 && strncmp(hostname +len-11, "itools.info", 11) == 0)
		return 1;
	if(len >= 19 && strncmp(hostname +len-19, "appleiphonecell.com", 19) == 0)
		return 1;

	return 0;
}

// ------------------------------------------------------------
void Qaullib_Whitelist_LL_Init (void)
{
	qaul_whitelist_LL_first = (struct qaul_whitelist_LL_item *)malloc(sizeof(struct qaul_whitelist_LL_item));
	qaul_whitelist_LL_first->next = 0;
	qaul_whitelist_LL_first->prev = 0;
	qaul_whitelist_init = 1;
}

// ------------------------------------------------------------
int Qaullib_Whitelist_LL_NextItem (struct qaul_whitelist_LL_item *item)
{
	if(item != 0 && item->next != 0)
	{
		item = item->next;
		return 1;
	}

	return 0;
}

// ------------------------------------------------------------
int Qaullib_Whitelist_LL_PrevItem (struct qaul_whitelist_LL_item *item)
{
	if(
			item != 0 &&
			item != qaul_whitelist_LL_first &&
			item->prev != 0
			)
	{
		item = item->prev;
		return 1;
	}
	return 0;
}

// ------------------------------------------------------------
void Qaullib_Whitelist_LL_Add (union olsr_ip_addr *ip)
{
	struct qaul_whitelist_LL_item *new_item;
	new_item = (struct qaul_whitelist_LL_item *)malloc(sizeof(struct qaul_whitelist_LL_item));

	if(QAUL_DEBUG)
		printf("Qaullib_Whitelist_LL_Add\n");

	// fill in content
	new_item->ip.v4 = ip->v4;
	new_item->time = time(NULL);
	// lock
	pthread_mutex_lock( &qaullib_mutex_whitelistLL );

	// create links
	new_item->prev = qaul_whitelist_LL_first;
	new_item->next = qaul_whitelist_LL_first->next;
	if(qaul_whitelist_LL_first->next != 0)
	{
		qaul_whitelist_LL_first->next->prev = new_item;
	}
	qaul_whitelist_LL_first->next = new_item;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_whitelistLL );
}

// ------------------------------------------------------------
int Qaullib_Whitelist_LL_Find_ByIP (union olsr_ip_addr *ip, struct qaul_whitelist_LL_item **item)
{
	struct qaul_whitelist_LL_item *myitem = qaul_whitelist_LL_first;
	int i=0;

	while(Qaullib_Whitelist_LL_NextItem(myitem))
	{
		myitem = myitem->next;

		printf("Qaullib_Whitelist_LL_Find_ByIP i = %i, %u\n", i, (uint32_t)myitem->ip.v4.s_addr);
		i++;
		// check if older than timeout
		if(myitem->time < time(NULL) -CAPTIVE_WHITELIST_TIMEOUT)
		{
			printf("Qaullib_Whitelist_LL_Find_ByIP to old, deleting\n");
			myitem = myitem->prev;
			Qaullib_Whitelist_LL_Delete(myitem->next);
		}
		else
		{
			// compare IP
			if((uint32_t)myitem->ip.v4.s_addr == (uint32_t)ip->v4.s_addr)
			//if(myitem->ip.v4.s_addr == ip)
			{
				printf("Qaullib_Whitelist_LL_Find_ByIP found\n");
				*item = myitem;
				return 1;
			}
		}
	}

	return 0;
}

// ------------------------------------------------------------
void Qaullib_Whitelist_LL_Delete (struct qaul_whitelist_LL_item *item)
{
	// lock
	pthread_mutex_lock( &qaullib_mutex_whitelistLL );

	if(item->prev != 0)
		item->prev->next = item->next;
	if(item->next != 0)
		item->next->prev = item->prev;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_whitelistLL );

	free(item);
}

// ------------------------------------------------------------
void Qaullib_Whitelist_LL_Clean (void)
{
	struct qaul_whitelist_LL_item *item;

	item = qaul_whitelist_LL_first;

	// which is older than the timeout
	while(Qaullib_Whitelist_LL_NextItem(item))
	{
		item = item->next;

		if(item->time < time(NULL) -CAPTIVE_WHITELIST_TIMEOUT)
		{
			Qaullib_Whitelist_LL_Delete(item);
		}
	}
}

