/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_MSG_LL
#define _QAULLIB_MSG_LL

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include "qaullib_defines.h"

/**
 * The message linked list contains all recently received messages.
 * The messages are added to a linked list according
 * to their id in the data base.
 */
struct qaul_msg_LL_item {
	struct qaul_msg_LL_item *next;            /// next node
	struct qaul_msg_LL_item *prev;            /// previous node

    int  id;                                  /// data base ID of the file entry
    int  type;                                /// type of the file, see QAUL_MSGTYPE_XXX
    char name[MAX_USER_LEN +1];               /// senders qaul user name
    char msg[MAX_MESSAGE_LEN +1];             /// chat message
    int  time;                                /// time when message was sent or received
    int  read;                                /// 1 if message has been read by the GUI,
    										  /// 0 if message has not been read
    int  ipv;                                 /// IP version protocol: 4 | 6
    char ip[MAX_IP_LEN +1];                   /// IP string
    union olsr_ip_addr ip_union;              /// IP union
};

/**
 * A message node.
 * Contains a pointer to a @a item and acts as a
 * helper element for the linked list functions.
 */
struct qaul_msg_LL_node {
	struct qaul_msg_LL_item *item;            /// pointer to selected item
};

/**
 * Pointer to the first item of the list.
 * If list is empty this pointer is null.
 */
struct qaul_msg_LL_item *qaul_msg_LL_first;

/**
 * to be called once at startup to create the linked list
 */
void Qaullib_Msg_LL_Init (void);

/**
 * Add a new @a item to the list.
 */
void Qaullib_Msg_LL_Add (struct qaul_msg_LL_item *item);

/**
 * Add a new @a item as next item to an existing item linked in @a node.
 * Set the pointer in @a node to the new item.
 */
void Qaullib_Msg_LL_AddNext (struct qaul_msg_LL_item *item, struct qaul_msg_LL_node *node);

/**
 * Checks if a message item is available with an id bigger
 * than @a id and sets the pointer in the @a node to the
 * next item.
 *
 * @retval 1 an item is available
 * @retval 0 no items availalbe
 */
int  Qaullib_Msg_LL_FirstItem (struct qaul_msg_LL_node *node, int id);

/**
 * Checks if a message item for the web client is available
 * with an id bigger than @a id and sets the pointer in the
 * @a node to the next item.
 *
 * @retval 1 next item found
 * @retval 0 no next item
 */
int  Qaullib_Msg_LL_FirstWebItem (struct qaul_msg_LL_node *node, int id);

/**
 * Sets the pointer in the @a node to the next item
 *
 * @retval 1 next item found
 * @retval 0 no next item
 */
int  Qaullib_Msg_LL_NextItem (struct qaul_msg_LL_node *node);

/**
 * Sets the pointer in the @a node to the previous message.
 *
 * @retval 1 previous item found
 * @retval 0 no previous item
 */
int  Qaullib_Msg_LL_PrevItem (struct qaul_msg_LL_node *node);

/**
 * Sets the pointer in the @a node to the previous message for
 * the web interface.
 *
 * @retval 1 previous item found
 * @retval 0 no previous item
 */
int  Qaullib_Msg_LL_PrevWebItem (struct qaul_msg_LL_node *node);

/**
 * delete @a item from list
 */
void Qaullib_Msg_LL_DeleteItem (struct qaul_msg_LL_item *item);

/**
 * Delete all items that are over MAX_MSG_COUNT
 */
void Qaullib_Msg_LL_DeleteOld (void);

/**
 * Delete all items that are over MAX_MSG_COUNT
 */
void Qaullib_Msg_LL_DeleteTmp (struct qaul_msg_LL_node *node);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
