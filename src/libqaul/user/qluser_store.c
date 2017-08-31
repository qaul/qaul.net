#include "qluser_store.h"

#include <cuckoo.h>
#include <dirent.h>
#include <memory.h>
#include <malloc.h>

#include "../crypto/qcry_helper.h"
#include "../olsrd/olsr_types.h"
#include "../olsrd/hashing.h"


/* Static storage context for all indexable fields */
static cuckoo_map *fp_map = NULL, *ip_map = NULL, *n_map = NULL;
#define INIT_MAP_SIZE 17
#define CHECK_STORE if(fp_map == NULL || ip_map == NULL || n_map == NULL) return QLUSER_NOT_INITIALISED;
char *strhash_ip(union olsr_ip_addr *__ip);


int qluser_store_initialise(const char *db_path, const char *key_path, unsigned int flags)
{
    int ret;
    if(fp_map != NULL) return QLUSER_ALREADY_INIT;

    // TODO: Get cflags and ms from flags & config?
    uint32_t cflags = CUCKOO_DEFAULT | CUCKOO_TABLES_THREE;
    uint32_t ms = INIT_MAP_SIZE;

    ret = cuckoo_init(&fp_map, ms, cflags);
    if(ret) goto c1;

    ret = cuckoo_init(&ip_map, ms, cflags);
    if(ret) goto c2;

    ret = cuckoo_init(&n_map, ms, cflags);
    if(ret) goto c3;


    /**** Load all pubkey files from key_path ****/
    DIR *dir;
    struct dirent *ent;

    /* Iterate over all keys that end in ".pub" and load them */
    if ((dir = opendir(key_path)) != NULL) {
        while ((ent = readdir(dir)) != NULL) {

            /* Only look at regular files */
            if(ent->d_type != DT_REG) continue;

            /* Does it end with ".pub" ? */
            char *name = ent->d_name;
            size_t nlen = strlen(name);
            if(strcmp(name + nlen - 4, ".pub") != 0) continue;

            size_t fplen = strlen(name) - 3;
            char fp[fplen];
            memset(fp, 0, fplen);
            memcpy(fp, name, fplen - 1);

            /* Load key via qcry util functions */
            mbedtls_pk_context *pubkey;
            ret = qcry_load_pubkey(&pubkey, key_path, ent->d_name);
            if(ret) goto c3;

            /* Add pubkey (for fp) to pubkey table */
            qluser_t *usr;
            ret = cuckoo_retrieve(fp_map, fp, (void**) &usr);
            if(ret) goto c3; // FIXME: Add new cleanup state for pubkeys?

            /* Store pubkey reference in usr_t struct */
            usr->pubkey = pubkey;
        }
        closedir(dir);
    } else {
        ret = QLUSER_INVALID_KEYSTORE;
        goto c3;
    }

    /* Return if we got here */
    return QLUSER_SUCCESS;

    /* Clean up the mess we made and return failure */
    c3: cuckoo_free(n_map, CUCKOO_NO_CB);
    c2: cuckoo_free(ip_map, CUCKOO_NO_CB);
    c1: cuckoo_free(fp_map, CUCKOO_NO_CB);
    return ret;
}


int qluser_store_adduser(const char *fp, const char *name)
{
    CHECK_STORE
    int ret;

    /** Check the input user struct for validity */
    if(fp == NULL || name == NULL) return QLUSER_INVALID_PARAMS;

    /* Check if the user already exists */
    if(cuckoo_contains(fp_map, fp) == 0) return QLUSER_USER_EXISTS;

    /* Malloc a user struct we can keep in the table forever */
    qluser_t *user = (qluser_t*) malloc(sizeof(qluser_t));
    if(user == NULL) return QLUSER_MALLOC_FAILED;
    memset(user, 0, sizeof(qluser_t));

    /* Fill new user struct with the data we already know */
    user->fp = strdup(fp);
    user->name = strdup(name);

    /* Always insert fingerprint to table */
    ret = cuckoo_insert(fp_map, fp, user);
    if(ret) return QLUSER_INSERT_FAILED;

    /* Only insert name or ip if they exist */
    if(user->name != NULL) {
        ret = cuckoo_insert(n_map, name, user);
        if(ret) return QLUSER_INSERT_FAILED;
    }

    return QLUSER_SUCCESS;
}


int qluser_store_rmuser(const char *fp)
{
    CHECK_STORE
    if(fp == NULL) return QLUSER_INVALID_PARAMS;
    int ret;

    /* Check that the user exists */
    if(cuckoo_contains(fp_map, fp) != 0) return QLUSER_USER_NOT_FOUND;

    /* Store user reference for cleaning later */
    qluser_t *user;
    ret = cuckoo_retrieve(fp_map, fp, (void**) &user);
    if(ret) return QLUSER_USER_NOT_FOUND;

    /* Always delete from fp table */
    ret = cuckoo_remove(fp_map, fp, CUCKOO_NO_CB);
    if(ret) return QLUSER_REMOVE_FAILED;

    /* Always delete from name table */
    ret = cuckoo_remove(n_map, user->name, CUCKOO_NO_CB);
    if(ret) return QLUSER_REMOVE_FAILED;

    /* Check if the IP is filled */
    if(user->node != NULL) {

        /* Delete from ip table */
        char *ip = strhash_ip(user->node->ip);
        ret = cuckoo_remove(ip_map, ip, CUCKOO_NO_CB);
        if(ret) return QLUSER_REMOVE_FAILED;
    }

    /* Only free node if we are the last user */
    if(user->node && cuckoo_size(user->node->ids) <= 1) {
        free(user->node->ip);
        // FIXME: We need to clean these more cleanly
        ret = cuckoo_free(user->node->ids, CUCKOO_NO_CB);
        if(ret) return QLUSER_REMOVE_FAILED;
        free(user->node);
        user->node = NULL;
    }

    /* Free fp and name strings */
    free((char*) user->fp);
    free(user->name);

    /* Free pubkey if it exists */
    if(user->pubkey != NULL) mbedtls_pk_free(user->pubkey);

    /* Free user itself and then return */
    free(user);
    return QLUSER_SUCCESS;
}

//int qluser_store_add_ip(struct qluser_t *user, union olsr_ip_addr *ip)
//{
//    CHECK_STORE
//    int ret;
//
//    /* Make sure a user entry exists first */
//    if(cuckoo_contains(fp_map, user->fp) != 0) {
//        printf("Can't add an IP to a user that isn't known yet!\n");
//        return QLUSER_USER_NOT_FOUND;
//    }
//
//    /* Remove any known IP for this user */
//    char *_ip = strhash_ip(ip);
//    if(cuckoo_contains(ip_map, _ip) == 0) {
//        ret = cuckoo_remove(ip_map, _ip, CUCKOO_NO_CB);
//        if(ret) return QLUSER_INSERT_FAILED;
//    }
//}


/****************************************************************************/

char *strhash_ip(union olsr_ip_addr *__ip) {
    uint32_t _ip = olsr_ip_hashing(__ip);
    return "" + _ip;
}
