/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_WHITELIST_LL
#define _QAULLIB_WHITELIST_LL

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include "../qaullib_defines.h"

#define  CAPTIVE_WHITELIST_TIMEOUT 60

int qaul_whitelist_init;


/**
 * The captive linked list contains all IP's of white listed web users for
 * which the web server simulates to be connected to the Internet.
 * This is important for iOS and OSX users to be able to use all functions
 * of their browser and their system, as network access is disabled
 * in offline local networks.
 * As another effect, the system will no longer show a login popup.
 *
 * List items are deleted after a timeout.
 */
struct qaul_whitelist_LL_item {
	struct qaul_whitelist_LL_item *next;        /// next node
	struct qaul_whitelist_LL_item *prev;        /// previous node

	union olsr_ip_addr ip;                    /// ip address
	time_t             time;                  /// time when last seen
};

/**
 * Pointer to the first entry of the captive list.
 * If list is empty this pointer is null.
 */
struct qaul_whitelist_LL_item *qaul_whitelist_LL_first;

/**
 * Initialize linked list
 */
void Qaullib_Whitelist_LL_Init (void);

/**
 * Sets the pointer in the @a node to the next item
 *
 * @retval 1 next item found
 * @retval 0 no next item
 */
int  Qaullib_Whitelist_LL_NextItem (struct qaul_whitelist_LL_item *item);

/**
 * Sets the pointer in the @a node to the previous IP.
 *
 * @retval 1 previous item found
 * @retval 0 no previous item
 */
int  Qaullib_Whitelist_LL_PrevItem (struct qaul_whitelist_LL_item *item);

/**
 * Add a new list entry with @a ip to the table.
 */
void Qaullib_Whitelist_LL_Add (union olsr_ip_addr *ip);

/**
 * Checks if an item for the IP exists
 *
 * @retval 1 item found
 * @retval 0 no item found
 */
int Qaullib_Whitelist_LL_Find_ByIP (union olsr_ip_addr *ip, struct qaul_whitelist_LL_item **item);

/**
 * Delete list entry by @a ip
 */
void Qaullib_Whitelist_LL_Delete (struct qaul_whitelist_LL_item *item);

/**
 * Clean linked list and delete all items older than CAPTIVE_TIMEOUT
 */
void Qaullib_Whitelist_LL_Clean (void);



#ifdef __cplusplus
}
#endif // __cplusplus

#endif
