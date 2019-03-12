/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <qaul/mod/userstore.h>
#include <qaul/mod/structures.h>

//#include <dirent.h>
//#include <memory.h>
//#include <malloc.h>

// #include "../crypto/qcry_helper.h"
// #include "../olsrd/olsr_types.h"
// #include "../olsrd/hashing.h"


// /* Static storage context for all indexable fields */
// static cuckoo_map *fp_map = NULL, *ip_map = NULL, *n_map = NULL, *node_map = NULL;
// static char *key_path, *db_path;

// int get_with(uint8_t t, qluser_t *user, void *idx);
// char *strhash_ip(union olsr_ip_addr *ip);
// void free_user(void *data);


// #define INIT_MAP_SIZE 17
// #define CHECK_STORE if(fp_map == NULL || ip_map == NULL || n_map == NULL || node_map == NULL) return QLUSER_NOT_INITIALISED;
// #define MAP_FP      0
// #define MAP_IP      1
// #define MAP_NAME    2


// int qluser_store_initialise(const char *db_path, const char *key_path, unsigned int flags)
// {
//     int ret;
//     if(fp_map != NULL) return QLUSER_ALREADY_INIT;

//     db_path = strdup(db_path);
//     key_path = strdup(key_path);

//     // TODO: Get cflags and ms from flags & config?
//     uint32_t cflags = CUCKOO_DEFAULT | CUCKOO_TABLES_THREE;
//     uint32_t ms = INIT_MAP_SIZE;

//     ret = cuckoo_init(&fp_map, ms, cflags);
//     if(ret) goto c1;

//     ret = cuckoo_init(&ip_map, ms, cflags);
//     if(ret) goto c2;

//     ret = cuckoo_init(&n_map, ms, cflags);
//     if(ret) goto c3;

//     ret = cuckoo_init(&node_map, ms, cflags);
//     if(ret) goto c4;

//     /**** Load all pubkey files from key_path ****/
//     DIR *dir;
//     struct dirent *ent;

//     /* Iterate over all keys that end in ".pub" and load them */
//     if ((dir = opendir(key_path)) != NULL) {
//         while ((ent = readdir(dir)) != NULL) {

//             /* Only look at regular files */
//             if(ent->d_type != DT_REG) continue;

//             /* Does it end with ".pub" ? */
//             char *name = ent->d_name;
//             size_t nlen = strlen(name);
//             if(strcmp(name + nlen - 4, ".pub") != 0) continue;

//             size_t fplen = strlen(name) - 3;
//             char fp[fplen];
//             memset(fp, 0, fplen);
//             memcpy(fp, name, fplen - 1);

//             /* Load key via qcry util functions */
//             mbedtls_pk_context *pubkey;
//             ret = qcry_load_pubkey(&pubkey, key_path, ent->d_name);
//             if(ret) goto c3;

//             /* Add pubkey (for fp) to pubkey table */
//             qluser_t *usr;
//             ret = cuckoo_retrieve(fp_map, fp, (void**) &usr);
//             if(ret) goto c3; // FIXME: Add new cleanup state for pubkeys?

//             /* Store pubkey reference in usr_t struct */
//             usr->pubkey = pubkey;
//         }
//         closedir(dir);
//     } else {
//         ret = QLUSER_INVALID_KEYSTORE;
//         goto c3;
//     }

//     /* Return if we got here */
//     return QLUSER_SUCCESS;

//     /* Clean up the mess we made and return failure */
//     c4: cuckoo_free(node_map, CUCKOO_NO_CB);
//     c3: cuckoo_free(n_map, CUCKOO_NO_CB);
//     c2: cuckoo_free(ip_map, CUCKOO_NO_CB);
//     c1: cuckoo_free(fp_map, CUCKOO_NO_CB);
//     return ret;
// }


// int qluser_store_adduser(const char *fp, const char *name)
// {
//     CHECK_STORE
//     int ret;

//     /** Check the input user struct for validity */
//     if(fp == NULL || name == NULL) return QLUSER_INVALID_PARAMS;

//     /* Check if the user already exists */
//     if(cuckoo_contains(fp_map, fp) == 0) return QLUSER_USER_EXISTS;

//     /* Malloc a user struct we can keep in the table forever */
//     qluser_t *user = (qluser_t*) malloc(sizeof(qluser_t));
//     if(user == NULL) return QLUSER_MALLOC_FAILED;
//     memset(user, 0, sizeof(qluser_t));

//     /* Fill new user struct with the data we already know */
//     user->fp = strdup(fp);
//     user->name = strdup(name);

//     /* Always insert fingerprint to table */
//     ret = cuckoo_insert(fp_map, fp, user);
//     if(ret) return QLUSER_INSERT_FAILED;

//     /* Only insert name or ip if they exist */
//     if(user->name != NULL) {
//         ret = cuckoo_insert(n_map, name, user);
//         if(ret) return QLUSER_INSERT_FAILED;
//     }

//     return QLUSER_SUCCESS;
// }


// int qluser_store_rmuser(const char *fp)
// {
//     CHECK_STORE
//     if(fp == NULL) return QLUSER_INVALID_PARAMS;
//     int ret;

//     /* Check that the user exists */
//     if(cuckoo_contains(fp_map, fp) != 0) return QLUSER_USER_NOT_FOUND;

//     /* Store user reference for cleaning later */
//     qluser_t *user;
//     ret = cuckoo_retrieve(fp_map, fp, (void**) &user);
//     if(ret) return QLUSER_USER_NOT_FOUND;

//     /* Always delete from fp table */
//     ret = cuckoo_remove(fp_map, fp, CUCKOO_NO_CB);
//     if(ret) return QLUSER_REMOVE_FAILED;

//     /* Always delete from name table */
//     ret = cuckoo_remove(n_map, user->name, CUCKOO_NO_CB);
//     if(ret) return QLUSER_REMOVE_FAILED;

//     /* Check if the IP is filled */
//     if(user->node != NULL) {

//         /* Delete from ip table */
//         char *ip = strhash_ip(user->node->ip);
//         ret = cuckoo_remove(ip_map, ip, CUCKOO_NO_CB);
//         if(ret) return QLUSER_REMOVE_FAILED;
//     }

//     /* Only free node if we are the last user */
//     if(user->node && cuckoo_size(user->node->ids) <= 1) {
//         free(user->node->ip);
//         // FIXME: We need to clean these more cleanly
//         ret = cuckoo_free(user->node->ids, CUCKOO_NO_CB);
//         if(ret) return QLUSER_REMOVE_FAILED;
//         free(user->node);
//         user->node = NULL;
//     }

//     /* Free fp and name strings */
//     free((char*) user->fp);
//     free(user->name);

//      Free pubkey if it exists 
//     if(user->pubkey != NULL) mbedtls_pk_free(user->pubkey);

//     /* Free user itself and then return */
//     free(user);
//     return QLUSER_SUCCESS;
// }


// int qluser_store_rmuser_all(const char *fp)
// {
//     CHECK_STORE

//     int ret = qluser_store_rmuser(fp);
//     if(ret) return ret;

//     // TODO: Remove user from database
//     return QLUSER_SUCCESS;
// }


// int qluser_store_add_ip(const char *fp, union olsr_ip_addr *ip)
// {
//     CHECK_STORE
//     int ret;

//     /* Make sure a user entry exists first */
//     if(cuckoo_contains(fp_map, fp) != 0) return QLUSER_USER_NOT_FOUND;

//     /* Create a new node for the IP if none exists already */
//     char *_ip = strhash_ip(ip);
//     if(cuckoo_contains(node_map, _ip) != 0) {

//         qluser_node_t *new = (qluser_node_t*) calloc(sizeof(qluser_node_t), 1);
//         if(new == NULL) return QLUSER_MALLOC_FAILED;

//         ret = cuckoo_init(&new->ids, INIT_MAP_SIZE, CUCKOO_DEFAULT | CUCKOO_TABLES_THREE);
//         if(ret) goto c1;

//         new->ip = ip;
//         ret = cuckoo_insert(node_map, _ip, new);
//         if(ret) return QLUSER_ERROR;
//     }

//     /* Get the node */
//     qluser_node_t *node;
//     ret = cuckoo_retrieve(node_map, _ip, (void**) &node);
//     if(ret) return QLUSER_NODE_NOT_FOUND;

//     /* Get the user */
//     qluser_t *user;
//     ret = cuckoo_retrieve(fp_map, fp, (void**) &user);
//     if(ret) return QLUSER_USER_NOT_FOUND;

//     /* Remove any old node that user already holds */
//     if(user->node != NULL) {
//         ret = cuckoo_remove(user->node->ids, fp, CUCKOO_NO_CB);
//         if(ret) return QLUSER_ERROR;
//         user->node = NULL;
//     }

//     /* Add user to node and node to user  <==> */
//     ret = cuckoo_insert(node->ids, fp, user);
//     if(ret) goto c2;
//     user->node = node;
//     return QLUSER_SUCCESS;

//     c2:
//     cuckoo_free(node->ids, CUCKOO_NO_CB);
//     free(node);

//     c1: free(node);
//     return ret;
// }


// int qluser_store_add_pubkey(const char *fp, mbedtls_pk_context *pubkey, enum qluser_trust_t trust)
// {
//     CHECK_STORE
//     if(cuckoo_contains(fp_map, fp) != 0) return QLUSER_USER_NOT_FOUND;

//     qluser_t *user;
//     int ret = cuckoo_retrieve(fp_map, fp, (void**) &user);
//     if(ret) return QLUSER_USER_NOT_FOUND;

//     if(user->pubkey != NULL) mbedtls_pk_free(user->pubkey);
//     user->pubkey = pubkey;
//     user->k_trust = trust;

//     /* Only store keys that we have verified to disk */
//     if(trust == VERIFIED) {
//         ret = qcry_save_pubkey(user->pubkey, key_path, user->fp);
//         if(ret) return ret;
//     }

//     return QLUSER_SUCCESS;
// }


// int qluser_store_set_keytrust(const char *fp, enum qluser_trust_t trust)
// {
//     CHECK_STORE
//     if(cuckoo_contains(fp_map, fp) != 0) return QLUSER_USER_NOT_FOUND;

//     qluser_t *user;
//     int ret = cuckoo_retrieve(fp_map, fp, (void**) &user);
//     if(ret) return QLUSER_USER_NOT_FOUND;

//     /* Store new key-trust and possibly save key to disk */
//     if(user->k_trust == VERIFIED) {
//         ret = qcry_save_pubkey(user->pubkey, key_path, user->fp);
//         if(ret) return ret;
//     }

//     return QLUSER_SUCCESS;
// }


// int qluser_store_getby_fp(struct qluser_t *user, const char *fp)
// {
//     return get_with(MAP_FP, user, (void*) fp);
// }


// int qluser_store_getby_name(struct qluser_t *user, const char *name)
// {
//     return get_with(MAP_NAME, user, (void*) name);
// }


// int qluser_store_getby_ip(struct qluser_t *user, union olsr_ip_addr *ip)
// {
//     return get_with(MAP_IP, user, (void*) ip);
// }


// int qluser_store_free()
// {
//     CHECK_STORE
//     int ret;

//     /* First clear all user structs from fp table */
//     ret = cuckoo_free(fp_map, free_user);
//     if(ret) return QLUSER_REMOVE_FAILED;

//     ret = cuckoo_free(ip_map, CUCKOO_NO_CB);
//     if(ret) return QLUSER_REMOVE_FAILED;

//     ret = cuckoo_free(n_map, CUCKOO_NO_CB);
//     if(ret) return QLUSER_REMOVE_FAILED;

//     return QLUSER_SUCCESS;
// }


// /****************************************************************************/


// int get_with(uint8_t t, qluser_t *user, void *idx)
// {
//     CHECK_STORE
//     qluser_t *_user;
//     int ret;

//     cuckoo_map *m;
//     switch(t) {
//         case MAP_FP:    m = fp_map; break;
//         case MAP_IP:    m = ip_map; break;
//         case MAP_NAME:  m = n_map;  break;
//         default:                    return QLUSER_INVALID_PARAMS;
//     }

//     if(cuckoo_contains(fp_map, idx) != 0) return QLUSER_USER_NOT_FOUND;
//     ret = cuckoo_retrieve(m, idx, (void**) &_user);
//     if(ret) return QLUSER_USER_NOT_FOUND;

//     /* Copy contents from storage */
//     memcpy(user, _user, sizeof(qluser_t));
//     return QLUSER_SUCCESS;
// }


// char *strhash_ip(union olsr_ip_addr *ip) {
//     uint32_t _ip = olsr_ip_hashing(ip);
//     return "" + _ip;
// }


// void free_user(void *data)
// {
//     qluser_t *user = (qluser_t*) data;
//     int ret;

//     /* Only free node if we are the last user */
//     if(user->node && cuckoo_size(user->node->ids) <= 1) {
//         free(user->node->ip);
//         // FIXME: We need to clean these more cleanly
//         ret = cuckoo_free(user->node->ids, CUCKOO_NO_CB);
//         if(ret) return;

//         free(user->node);
//         user->node = NULL;
//     }

//     /* Free fp and name strings */
//     free((char*) user->fp);
//     free(user->name);

//     /* Free pubkey if it exists */
//     if(user->pubkey != NULL) mbedtls_pk_free(user->pubkey);

//     /* Free user itself and then return */
//     free(user);
// }



/****************** USER STRUCT FUNCTIONS ******************/


ql_error_t qluser_create(enum qluser_t t, const char *username, const char *fp, union ql_user **user)
{
    union ql_user *u = calloc(sizeof(union ql_user), 1);
    switch (t){
        case INTERNAL:
            u->intern = calloc(sizeof(struct ql_user_internal), 1);
            break;
        case EXTERNAL:
            u->ext = calloc(sizeof(struct ql_user_internal), 1);
            break;
    }



    return SUCCESS;
}