/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"
#include "olsrd/hashing.h"
#include "qaullib_threads.h"
#include "qaullib_file_LL.h"


int qaul_file_LL_count;
struct qaul_file_LL_item Qaul_file_LL_table[HASHSIZE];


/**
 * Hashing function. Creates a key based on an @a filehash
 *
 * @retval the hash(a value in the (0 to HASHMASK-1) range)
 */
static uint32_t Qaullib_File_LL_Hashing(unsigned char* filehash);

// ------------------------------------------------------------
void Qaullib_File_LL_Init (void)
{
  int i;

  qaul_file_LL_count = 0;

  for (i = 0; i < HASHSIZE; i++)
  {
	  Qaul_file_LL_table[i].next = &Qaul_file_LL_table[i];
	  Qaul_file_LL_table[i].prev = &Qaul_file_LL_table[i];
  }
}


// ------------------------------------------------------------
void Qaullib_File_LL_InitNode(struct qaul_file_LL_node *node)
{
	node->index = 0;
	node->item = &Qaul_file_LL_table[0];
}

// ------------------------------------------------------------
void Qaullib_File_LL_InitNodeWithHash(struct qaul_file_LL_node *node, char *filehash)
{
	node->index = Qaullib_File_LL_Hashing(filehash);
	node->item = &Qaul_file_LL_table[node->index];
}

// ------------------------------------------------------------
int Qaullib_File_LL_NextNode (struct qaul_file_LL_node *node)
{
	for(; node->index < HASHSIZE;)
	{
		if(node->item->next != &Qaul_file_LL_table[node->index])
		{
			node->item = node->item->next;
			return 1;
		}
		node->index++;
		node->item =  &Qaul_file_LL_table[node->index];
	}
	return 0;
}

// ------------------------------------------------------------
int Qaullib_File_LL_NextNodePub (struct qaul_file_LL_node *node)
{
	while(Qaullib_File_LL_NextNode(node))
	{
		if(node->item->type == QAUL_FILETYPE_FILE && node->item->status >= QAUL_FILESTATUS_DOWNLOADED)
			return 1;
	}
	return 0;
}

// ------------------------------------------------------------
int Qaullib_File_LL_NextNodePubBinaries (struct qaul_file_LL_node *node)
{
	while(Qaullib_File_LL_NextNode(node))
	{
		if(node->item->type == QAUL_FILETYPE_EXECUTABLE && node->item->status >= QAUL_FILESTATUS_DOWNLOADED)
			return 1;
	}
	return 0;
}

// ------------------------------------------------------------
int Qaullib_File_LL_NextNodeGuiPriv (struct qaul_file_LL_node *node)
{
	while(Qaullib_File_LL_NextNode(node))
	{
		if(node->item->gui_notify == 1)
		{
			if(node->item->type == 1)
				return 1;
			else
				node->item->gui_notify = 0;
		}
	}
	return 0;
}


/**
 * Checks whether there is a next item within hashed linked list.
 * The function links the item into the @a node
 *
 * @retval 1 there is a next item
 * @retval 0 there is no next item
 */
int Qaullib_File_LL_NextItem (struct qaul_file_LL_node *node)
{
	if(node->item->next != &Qaul_file_LL_table[node->index])
	{
		node->item = node->item->next;
		return 1;
	}
	return 0;
}

// ------------------------------------------------------------
struct qaul_file_LL_item* Qaullib_File_LL_Add (struct qaul_file_LL_item *item)

{
	struct qaul_file_LL_item *new_item;
	new_item = (struct qaul_file_LL_item *)malloc(sizeof(struct qaul_file_LL_item));

	printf("Qaullib_File_LL_Add hash: %s\n", item->hashstr);

	// get index
	uint32_t LLhash = Qaullib_File_LL_Hashing(item->hash);

	// fill in content
	memcpy(&new_item->hash, item->hash, MAX_HASH_LEN);
	memcpy(&new_item->hashstr, item->hashstr, MAX_HASHSTR_LEN);
	memcpy(&new_item->hashstr[MAX_HASHSTR_LEN], "\0", 1);
	strncpy(new_item->suffix, item->suffix, MAX_SUFFIX_LEN);
	memcpy(&new_item->suffix[MAX_SUFFIX_LEN], "\0", 1);
	strncpy(new_item->description, item->description, MAX_DESCRIPTION_LEN);
	memcpy(&new_item->description[MAX_DESCRIPTION_LEN], "\0", 1);
	new_item->created_at = item->created_at;

	new_item->id = item->id;
	new_item->type = item->type;
	new_item->status = item->status;
	new_item->size = item->size;
	new_item->downloaded = item->downloaded;
	new_item->downloaded_chunk = 0;

	strncpy(new_item->adv_name, item->adv_name, MAX_USER_LEN);
	memcpy(&new_item->adv_name[MAX_USER_LEN], "\0", 1);
	memcpy(&new_item->adv_ip, &item->adv_ip, sizeof(union olsr_ip_addr));
	new_item->adv_validip = item->adv_validip;

	new_item->gui_notify = 1;

	// init discovery LL
	new_item->discoveryLL.next = &new_item->discoveryLL;
	new_item->discoveryLL.prev = &new_item->discoveryLL;

	// lock
	pthread_mutex_lock( &qaullib_mutex_fileLL );
	// create links
	new_item->prev = &Qaul_file_LL_table[LLhash];
	new_item->next = Qaul_file_LL_table[LLhash].next;

	Qaul_file_LL_table[LLhash].next = new_item;
	new_item->next->prev = new_item;
	qaul_file_LL_count++;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_fileLL );

	// check if file exists
	if(QAUL_DEBUG)
	{
		if(Qaullib_File_LL_HashExists(new_item->hash))
			printf("file found in LL\n");
		else
			printf("file not found in LL\n");
	}

	return new_item;
}


// ------------------------------------------------------------
void Qaullib_File_LL_Delete_Item (struct qaul_file_LL_item *item)
{
	if(QAUL_DEBUG)
		printf("Qaullib_File_LL_Delete_Item\n");

	// empty discovery list
	Qaullib_Filediscovery_LL_EmptyList(item);

	// lock
	pthread_mutex_lock( &qaullib_mutex_fileLL );

	item->prev->next = item->next;
	item->next->prev = item->prev;
	qaul_file_LL_count--;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_fileLL );

	free(item);
}

// ------------------------------------------------------------
int Qaullib_File_LL_HashSearch (char *filehash, struct qaul_file_LL_item **item)
{
	struct qaul_file_LL_node mynode;
	Qaullib_File_LL_InitNodeWithHash(&mynode, filehash);

	while(Qaullib_File_LL_NextItem(&mynode))
	{
		if(memcmp(&mynode.item->hash, filehash, MAX_HASH_LEN) == 0)
		{
			if(QAUL_DEBUG)
				printf("item found: %s\n", mynode.item->hashstr);
			*item = mynode.item;
			return 1;
		}
	}
	return 0;
}

// ------------------------------------------------------------
int Qaullib_File_LL_HashExists (char *filehash)
{
	struct qaul_file_LL_node mynode;
	Qaullib_File_LL_InitNodeWithHash(&mynode, filehash);
	while(Qaullib_File_LL_NextItem(&mynode))
	{
		if(memcmp(&mynode.item->hash, filehash, MAX_HASH_LEN) == 0)
		{
			return 1;
		}
	}
	return 0;
}

// ------------------------------------------------------------
int Qaullib_File_LL_FileAvailable (char *filehash)
{
	struct qaul_file_LL_node mynode;
	Qaullib_File_LL_InitNodeWithHash(&mynode, filehash);
	while(Qaullib_File_LL_NextItem(&mynode))
	{
		if(memcmp(&mynode.item->hash, filehash, MAX_HASH_LEN) == 0)
		{
			if(mynode.item->status >= QAUL_FILESTATUS_DOWNLOADED)
				return 1;
			else
				return 0;
		}
	}
	return 0;
}

// ------------------------------------------------------------
static uint32_t Qaullib_File_LL_Hashing (unsigned char *filehash)
{
	uint32_t hash;
	hash = jenkins_hash((const uint8_t *)filehash, MAX_HASH_LEN);
	//printf("Qaullib_File_LL_Hashing mask: %u u: %u\n", HASHMASK, hash & HASHMASK);
	return hash & HASHMASK;
}

// ------------------------------------------------------------
void Qaullib_Filediscovery_LL_DiscoveryMsgProcessing (struct qaul_fileavailable_msg *msg, union olsr_ip_addr *ip)
{
	struct qaul_file_LL_item *file;

	if(Qaullib_File_LL_HashSearch(msg->hash, &file))
	{
		char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
		inet_ntop(AF_INET, &ip->v4, (char *)&ipbuf, MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN));
		printf("Qaullib_Filediscovery_LL_AddSeederIp %s\n", ipbuf);

		Qaullib_Filediscovery_LL_AddSeederIp (file, ip);
	}
}

// ------------------------------------------------------------
void Qaullib_Filediscovery_LL_AddSeederIp (struct qaul_file_LL_item *file, union olsr_ip_addr *ip)
{
	struct qaul_filediscovery_LL_item *new_item;

	if(QAUL_DEBUG)
		printf("Qaullib_Filediscovery_LL_AddSeederIp\n");

	// check if the ip already exists
	if(!Qaullib_Filediscovery_LL_SeederExists(file, ip))
	{
		if(QAUL_DEBUG)
			printf("add seeder\n");

		new_item = (struct qaul_filediscovery_LL_item *)malloc(sizeof(struct qaul_filediscovery_LL_item));
		memcpy(&new_item->ip, ip, sizeof(union olsr_ip_addr));

		// lock
		pthread_mutex_lock( &qaullib_mutex_filediscoveryLL );

		new_item->prev = file->discoveryLL.prev;
		new_item->next = &file->discoveryLL;
		file->discoveryLL.prev->next = new_item;
		file->discoveryLL.prev = new_item;

		file->discovery_count++;
		file->discovery_timestamp = time(NULL);

		// unlock
		pthread_mutex_unlock( &qaullib_mutex_filediscoveryLL );

		// change the status of the file
		if(file->status == QAUL_FILESTATUS_DISCOVERING)
		{
			if(QAUL_DEBUG)
				printf("qaul file status changed to QAUL_FILESTATUS_DISCOVERED\n");

			file->status = QAUL_FILESTATUS_DISCOVERED;
		}
	}
	else
	{
		if(QAUL_DEBUG)
			printf("seeder exists\n");
	}
}

// ------------------------------------------------------------
void Qaullib_Filediscovery_LL_DeleteSeederIp (struct qaul_file_LL_item *file, union olsr_ip_addr *ip)
{
	struct qaul_filediscovery_LL_item *item;

	if(QAUL_DEBUG)
		printf("Qaullib_Filediscovery_LL_DeleteSeederIp\n");

	// get entry
	if(Qaullib_Filediscovery_LL_SearchIp(file, ip, item))
	{
		// delete ip
		Qaullib_Filediscovery_LL_DeleteItem(item);
	}
}

// ------------------------------------------------------------
int  Qaullib_Filediscovery_LL_GetBestSeeder (struct qaul_file_LL_item *file, union olsr_ip_addr **ip)
{
	struct qaul_filediscovery_LL_item *item, *best_item;
	int found;
	float LQ;

	if(QAUL_DEBUG)
		printf("Qaullib_Filediscovery_LL_GetBestSeeder\n");

	item = &file->discoveryLL;
	found = 0;
	LQ = 0;

	// loop through list and check the LQ for every seeder
	while(Qaullib_Filediscovery_LL_NextItem(file, &item))
	{
		// todo: check users LQ
		found = 1;
		best_item = item;
		break;
	}

	// return the best seeder
	if(found)
	{
		*ip = &best_item->ip;
		return 1;
	}

	return 0;
}

// ------------------------------------------------------------
int  Qaullib_Filediscovery_LL_SeederExists (struct qaul_file_LL_item *file, union olsr_ip_addr *ip)
{
	struct qaul_filediscovery_LL_item *discovery_item;
	return Qaullib_Filediscovery_LL_SearchIp (file, ip, discovery_item);
}

// ------------------------------------------------------------
int  Qaullib_Filediscovery_LL_SearchIp (struct qaul_file_LL_item *file, union olsr_ip_addr *ip, struct qaul_filediscovery_LL_item *discovery_item)
{
	discovery_item = &file->discoveryLL;

	// loop through list and check for ip
	while(Qaullib_Filediscovery_LL_NextItem(file, &discovery_item))
	{
		if(memcmp(&discovery_item->ip, ip, sizeof(ip)) == 0)
			return 1;
	}
	return 0;
}

// ------------------------------------------------------------
int  Qaullib_Filediscovery_LL_NextItem (struct qaul_file_LL_item *file, struct qaul_filediscovery_LL_item **discovery_item)
{
	struct qaul_filediscovery_LL_item *myitem;
	myitem = *discovery_item;

	if(myitem->next != &file->discoveryLL)
	{
		*discovery_item = myitem->next;
		return 1;
	}
	return 0;
}

// ------------------------------------------------------------
void Qaullib_Filediscovery_LL_EmptyList (struct qaul_file_LL_item *file)
{
	struct qaul_filediscovery_LL_item *discovery_item;
	discovery_item = &file->discoveryLL;
	// loop through list
	while(Qaullib_Filediscovery_LL_NextItem(file, &discovery_item))
	{
		discovery_item = discovery_item->prev;
		Qaullib_Filediscovery_LL_DeleteItem(discovery_item->next);
	}
}

// ------------------------------------------------------------
void Qaullib_Filediscovery_LL_DeleteItem (struct qaul_filediscovery_LL_item *item)
{
	// lock
	pthread_mutex_lock( &qaullib_mutex_filediscoveryLL );

	item->prev->next = item->next;
	item->next->prev = item->prev;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_filediscoveryLL );

	free(item);
}
