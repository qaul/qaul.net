/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"
#include "qaullib_topo_LL.h"
#include "qaullib_threads.h"


// ------------------------------------------------------------
void Qaullib_Topo_LL_Add (union olsr_ip_addr *src_ip, union olsr_ip_addr *dest_ip, float lq)
{
	struct qaul_topo_LL_item *new_item;
	new_item = (struct qaul_topo_LL_item *)malloc(sizeof(struct qaul_topo_LL_item));

	if(QAUL_DEBUG)
		printf("Qaullib_Topo_LL_Add\n");

	// fill in content
	memcpy((char *)&new_item->src_ip, src_ip, sizeof(union olsr_ip_addr));
	memcpy((char *)&new_item->dest_ip, dest_ip, sizeof(union olsr_ip_addr));
	new_item->lq = lq;

	// lock
	pthread_mutex_lock( &qaullib_mutex_topoLL );

	// create links
	new_item->next = qaul_topo_LL_first;
	qaul_topo_LL_first = new_item;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_topoLL );
}


// ------------------------------------------------------------
void Qaullib_Topo_LL_Delete_Item ()
{
	struct qaul_topo_LL_item *item;

	if(QAUL_DEBUG)
		printf("Qaullib_Topo_LL_Delete_Item\n");

	// lock
	pthread_mutex_lock( &qaullib_mutex_topoLL );

	item = qaul_topo_LL_first;
	qaul_topo_LL_first = item->next;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_topoLL );

	free(item);
}

