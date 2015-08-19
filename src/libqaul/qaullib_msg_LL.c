/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"
#include "qaullib_threads.h"


// ------------------------------------------------------------
void Qaullib_Msg_LL_Init (void)
{
	qaul_msg_LL_first = 0;
}

// ------------------------------------------------------------
int Qaullib_Msg_LL_FirstItem (struct qaul_msg_LL_node *node, int id)
{
	int count, max_count;
	if(id == 0)
		max_count = MAX_MSG_FIRST;
	else
		max_count = MAX_MSG_COUNT;

	// check if first message is newer
	if(qaul_msg_LL_first == 0 || qaul_msg_LL_first->id <= id)
		return 0;

	node->item = qaul_msg_LL_first;

	// search oldest message
	count = 0;
	while(
			count < max_count &&
			node->item->next != 0 &&
			node->item->id > id
			)
	{
		node->item = node->item->next;
		count++;
	}

	return 1;
}

// ------------------------------------------------------------
int Qaullib_Msg_LL_FirstWebItem (struct qaul_msg_LL_node *node, int id)
{
	int count, max_count;
	struct qaul_msg_LL_item *tmp_item;

	node->item = 0;
	if(id == 0)
		max_count = MAX_MSG_FIRST_WEB;
	else
		max_count = MAX_MSG_COUNT_WEB;

	// check if first message is newer
	if(qaul_msg_LL_first == 0 || qaul_msg_LL_first->id <= id)
		return 0;

	tmp_item = qaul_msg_LL_first;
	if(tmp_item->type == QAUL_MSGTYPE_PUBLIC_IN || tmp_item->type == QAUL_MSGTYPE_PUBLIC_OUT)
		node->item = tmp_item;

	// search oldest message
	count = 0;
	while(
			count < MAX_MSG_COUNT &&
			tmp_item->next != 0 &&
			tmp_item->id > id
			)
	{
		tmp_item = tmp_item->next;
		count++;

		if(tmp_item->type == QAUL_MSGTYPE_PUBLIC_IN || tmp_item->type == QAUL_MSGTYPE_PUBLIC_OUT)
			node->item = tmp_item;
	}

	if(node->item != 0)
		return 1;

	return 0;
}

// ------------------------------------------------------------
int Qaullib_Msg_LL_NextItem (struct qaul_msg_LL_node *node)
{
	if(node->item != 0 && node->item->next != 0)
	{
		node->item = node->item->next;
		return 1;
	}
	return 0;
}

// ------------------------------------------------------------
int Qaullib_Msg_LL_PrevItem (struct qaul_msg_LL_node *node)
{
	if(
			node->item != 0 &&
			node->item != qaul_msg_LL_first &&
			node->item->prev != 0
			)
	{
		node->item = node->item->prev;
		return 1;
	}
	return 0;
}

// ------------------------------------------------------------
int Qaullib_Msg_LL_PrevWebItem (struct qaul_msg_LL_node *node)
{
	struct qaul_msg_LL_item *tmp_item;
	tmp_item = node->item;

	while(
			tmp_item != 0 &&
			tmp_item != qaul_msg_LL_first &&
			tmp_item->prev != 0
			)
	{
		tmp_item = tmp_item->prev;

		if(tmp_item->type == QAUL_MSGTYPE_PUBLIC_IN || tmp_item->type == QAUL_MSGTYPE_PUBLIC_OUT)
		{
			node->item = tmp_item;
			return 1;
		}
	}
	return 0;
}

// ------------------------------------------------------------
void Qaullib_Msg_LL_Add (struct qaul_msg_LL_item *item)
{
	struct qaul_msg_LL_item *new_item;
	new_item = (struct qaul_msg_LL_item *)malloc(sizeof(struct qaul_msg_LL_item));

	// fill in values
	new_item->id = item->id;
	new_item->type = item->type;
	strncpy(new_item->name, item->name, MAX_USER_LEN);
	memcpy(&new_item->name[MAX_USER_LEN], "\0", 1);
	strncpy(new_item->msg, item->msg, MAX_MESSAGE_LEN);
	memcpy(&new_item->msg[MAX_MESSAGE_LEN], "\0", 1);
	new_item->time = item->time;
	new_item->read = item->read;
	new_item->ipv = item->ipv;
	strncpy(new_item->ip, item->ip, MAX_IP_LEN);
	memcpy(&new_item->ip[MAX_IP_LEN], "\0", 1);
	new_item->prev = 0;
	new_item->next = 0;

	// lock
	pthread_mutex_lock( &qaullib_mutex_msgLL );

	// set pointers
	new_item->next = qaul_msg_LL_first;
	if(qaul_msg_LL_first != 0)
		qaul_msg_LL_first->prev = new_item;
	qaul_msg_LL_first = new_item;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_msgLL );

	// delete old messages
	Qaullib_Msg_LL_DeleteOld();
}

// ------------------------------------------------------------
void Qaullib_Msg_LL_AddNext (struct qaul_msg_LL_item *item, struct qaul_msg_LL_node *node)
{
	struct qaul_msg_LL_item *new_item;
	new_item = (struct qaul_msg_LL_item *)malloc(sizeof(struct qaul_msg_LL_item));

	// fill in values
	new_item->id = item->id;
	new_item->type = item->type;
	strncpy(new_item->name, item->name, MAX_USER_LEN);
	memcpy(&new_item->name[MAX_USER_LEN], "\0", 1);
	strncpy(new_item->msg, item->msg, MAX_MESSAGE_LEN);
	memcpy(&new_item->msg[MAX_MESSAGE_LEN], "\0", 1);
	new_item->time = item->time;
	new_item->read = item->read;
	new_item->ipv = item->ipv;
	strncpy(new_item->ip, item->ip, MAX_IP_LEN);
	memcpy(&new_item->ip[MAX_IP_LEN], "\0", 1);
	new_item->prev = 0;
	new_item->next = 0;

	// set pointers
	if(node->item == 0)
	{
		node->item = new_item;
	}
	else
	{
		new_item->prev = node->item;
		node->item->next = new_item;
		node->item = new_item;
	}
}

// ------------------------------------------------------------
void Qaullib_Msg_LL_Delete_Item (struct qaul_msg_LL_item *item)
{
	// lock
	pthread_mutex_lock( &qaullib_mutex_msgLL );

	if(item->prev != 0)
		item->prev->next = item->next;
	if(item->next != 0)
		item->next->prev = item->prev;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_msgLL );

	free(item);
}

// ------------------------------------------------------------
void Qaullib_Msg_LL_DeleteOld (void)
{
	int count;
	struct qaul_msg_LL_node node;

	node.item = qaul_msg_LL_first;
	count = 0;

	// are there more than MAX_MSG_COUNT
	while(Qaullib_Msg_LL_NextItem(&node))
	{
		count++;
	}

	// delete all over MAX_MSG_COUNT
	while(count > MAX_MSG_COUNT && Qaullib_Msg_LL_PrevItem(&node))
	{
		Qaullib_Msg_LL_Delete_Item(node.item->next);
		count--;
	}
}

// ------------------------------------------------------------
void Qaullib_Msg_LL_DeleteTmp (struct qaul_msg_LL_node *node)
{
	// delete all items in the list
	while(Qaullib_Msg_LL_NextItem(node))
	{
		Qaullib_Msg_LL_Delete_Item(node->item->prev);
	}
}
