//
// Created by spacekookie on 20/06/16.
//

#ifndef QAUL_QAULLIB_USR_TABLE_H_H
#define QAUL_QAULLIB_USR_TABLE_H_H

/** Reverse include so we can use user structs */
#include "qaullib_user.h"

enum ENTRY_POS {
    NAME = 0, IP = 1, DATA = 2
};

/* Generic storage item for user map */
struct user_store_item {
    struct user_store_item      *next, *prev;
    long                        index;

    union olsr_ip_addr ip;                    /// ip address
    time_t             time;                  /// time when last seen
    float              lq;                    /// link quality
    char               name[MAX_USER_LEN +1]; /// user name
    char               icon[MAX_FILENAME_LEN +1]; /// icon name
    int                type;                  /// type of user: see user types
    int                changed;               /// changes for GUI notifications: see user changed
    int				   favorite;			  /// user is favorite (don't delete it)
    unsigned char      id[MAX_HASH_LEN];      /// user id (hash of IP & name)
    char               idstr[MAX_HASHSTR_LEN +1]; /// user id as string
};

struct qaul_user_LL_node {
    struct qaul_user_LL_item *item;           /// link to the LL item
    uint32_t                  index;          /// array index of the node
};

typedef struct  {
    unsigned int            size;
    struct user_store_item  anchor;
} qaul_user_store;

int Qaullib_usrstore_init(qaul_user_store *store);

/*  */
int Qaullib_usrstore_add(qaul_user_store *store, union olsr_ip_addr *ip, qaul_cry_user *user_data);

/* Remove a user from the store with a fingerprint. Usernames can collide. Fingerprints can't (shouldn't!) */
int Qaullib_usrstore_rm(qaul_user_store *store, const char *fingerprint);

/* De-allocate all space used by this user store */
int Qaullib_usrstore_free(qaul_user_store *store);

/**
 * walks through the store and deletes all expired entries
 *
 *  - mark all clients older than 30 seconds as deleted
 *  - mark all web clients older than 2 minutes as deleted
 *  - remove all clients which are older than 5 minutes
 */
int Qaullib_usrstore_clean(qaul_user_store *store);

int Qaullib_usrstore_ip_exists(qaul_user_store *store, union olsr_ip_addr *ip);

/**
 * to be called once at startup to create linked list
 */
void Qaullib_User_LL_Init (void);

/**
 * creates a new list entry for @a ip in the user table.
 *
 * @retval pointer to the item
 */
struct qaul_user_LL_item* Qaullib_User_LL_Add (union olsr_ip_addr *ip, unsigned char *id);

/**
 * delete @a item from list
 */
void Qaullib_User_LL_Delete_Item (struct qaul_user_LL_item *item);

/**
 * loops through the list and deletes all expired entries
 * mark all clients older than 30 seconds as deleted
 * mark all web clients older than 2 minutes as deleted
 * remove all clients which are older than 5 minutes
 */
void Qaullib_User_LL_Clean (void);

/**
 * loops through the list and checks if @a ip exists
 *
 * @retval 1 ip exists
 * @retval 0 ip does not exist
 */
int  Qaullib_User_LL_IpExists (union olsr_ip_addr *ip);

/**
 * loops through the list and checks if @a ip entry of a real user
 * (not web user) exists.
 * If it exists, @a item will contain the pointer to the first IP entry of it.
 *
 * @retval 1 entry exists
 * @retval 0 entry does not exist
 */
int  Qaullib_User_LL_IpGetFirst (union olsr_ip_addr *ip, struct qaul_user_LL_item **item);

/**
 * Update LL according to olsrd topology list.
 * Loop through LL and update all nodes with this @a ip.
 * If no real user for @a ip exists, a new one is created.
 */
void Qaullib_User_LL_IpTouch (union olsr_ip_addr *ip, float linkcost);

/**
 * Loops through the list and checks if @a id exists. If it can't
 * find an entry for that @a id it checks if a real user without id set
 * exists for that @a ip.
 *
 * @retval  1  id found
 * @retval  0  id not found, but a real user without id found for this ip
 * @retval -1  id not found
 */
int  Qaullib_User_LL_IdSearch (union olsr_ip_addr *ip, unsigned char *id, struct qaul_user_LL_item **item);

/**
 * initializes a @a node with the first entry of the user table
 */
void Qaullib_User_LL_InitNode (struct qaul_user_LL_node *node);

/**
 * initializes a @a node of the user table according to the @a ip
 */
void Qaullib_User_LL_InitNodeWithIP(struct qaul_user_LL_node *node, union olsr_ip_addr *ip);

/**
 * checks if there is a next item in the user table
 *
 * @retval 1 next node found, @a node contains the pointer to the item
 * @retval 0 no next node exists
 */
int  Qaullib_User_LL_NextNode (struct qaul_user_LL_node *node);



#endif //QAUL_QAULLIB_USR_TABLE_H_H
