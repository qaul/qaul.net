/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"
#include "qaullib_appevent_LL.h"
#include "qaullib_threads.h"


// ------------------------------------------------------------
void Qaullib_Appevent_LL_Add (int event)
{
	struct qaul_appevent_LL_item *new_item;
	new_item = (struct qaul_appevent_LL_item *)malloc(sizeof(struct qaul_appevent_LL_item));

	// fill in content
	new_item->event = event;
	new_item->next = 0;

	// lock
	pthread_mutex_lock( &qaullib_mutex_appeventLL );

	// create links
	if(qaul_appevent_LL_first == 0)
		qaul_appevent_LL_first = new_item;
	else
		qaul_appevent_LL_last->next = new_item;

	qaul_appevent_LL_last = new_item;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_appeventLL );
}


// ------------------------------------------------------------
int Qaullib_Appevent_LL_Get ()
{
	struct qaul_appevent_LL_item *item;
	int event;

	if(qaul_appevent_LL_first == 0)
		event = 0;
	else
	{
		// lock
		pthread_mutex_lock( &qaullib_mutex_appeventLL );

		item = qaul_appevent_LL_first;
		event = item->event;

		// link to the next item
		qaul_appevent_LL_first = item->next;
		if(item->next == 0)
			qaul_appevent_LL_last = 0;

		// unlock
		pthread_mutex_unlock( &qaullib_mutex_appeventLL );

		free(item);
	}

	return event;
}

