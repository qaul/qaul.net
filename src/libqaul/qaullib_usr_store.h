//
// Created by spacekookie on 20/06/16.
//

#ifndef QAUL_QAULLIB_USR_TABLE_H_H
#define QAUL_QAULLIB_USR_TABLE_H_H

/** Reverse include so we can use user structs */
#include <time.h>
#include "qaullib_user.h"
#include "qaullib_defines.h"

enum ENTRY_POS {
    NAME = 0, IP = 1, DATA = 2
};

/* Generic storage item for user map */
struct user_store_item {
    struct user_store_item      *next;                      /// next node
    struct user_store_item      *prev;                      /// previous node
    union olsr_ip_addr          ip;                         /// ip address
    time_t                      time;                       /// time when last seen
    float                       lq;                         /// link quality
    char                        name[MAX_USER_LEN + 1];     /// user name
    char                        fp[QAUL_FP_LEN];            /// user key fingerprint
    char                        icon[MAX_FILENAME_LEN +1];  /// icon name
    int                         type;                       /// type of user: see user types
    int                         changed;                    /// changes for GUI notifications: see user changed
    int				            favorite;			        /// user is favorite (don't delete it)
    unsigned char               id[MAX_HASH_LEN];           /// user id (hash of IP & name)
    char                        idstr[MAX_HASHSTR_LEN +1];  /// user id as string
};

//struct qaul_user_LL_node {
//    struct qaul_user_LL_item *item;           /// link to the LL item
//    uint32_t                  index;          /// array index of the node
//};

typedef struct  {
    unsigned int            size;
    struct user_store_item  anchor;
} qaul_user_store;

int Qaullib_usrstore_init();

/*  */
int Qaullib_usrstore_add(union olsr_ip_addr *ip, qaul_cry_user *user_data);

/* Remove a user from the store with a fingerprint. Usernames can collide. Fingerprints can't (shouldn't!) */
int Qaullib_usrstore_rm(const char *fingerprint);

/* De-allocate all space used by this user store */
int Qaullib_usrstore_free();

/**
 * walks through the store and deletes all expired entries
 *
 *  - mark all clients older than 30 seconds as deleted
 *  - mark all web clients older than 2 minutes as deleted
 *  - remove all clients which are older than 5 minutes
 */
int Qaullib_usrstore_clean();

int Qaullib_usrstore_ip_exists(union olsr_ip_addr *ip);

int Qaullib_usrstore_fp_exists(unsigned char *fingerprint);

#endif //QAUL_QAULLIB_USR_TABLE_H_H
