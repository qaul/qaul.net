#include "qluser_store.h"

#include <cuckoo.h>
#include <dirent.h>
#include <memory.h>
#include "../crypto/qcry_helper.h"
#include "../olsrd/olsr_types.h"

/** Holds data about a node */
typedef struct qluser_node_t {
    union olsr_ip_addr  *ip;
    struct qluser_t     **identities;
};

/** Holds data about a user identity */
struct qluser_t {
    char *name;
    const char *fp;
    mbedtls_pk_context *pubkey;
    struct qluser_node_t *node;
};


/* Static storage context for all indexable fields */
static cuckoo_map *fp_map = NULL, *ip_map = NULL, *n_map = NULL;
#define INIT_MAP_SIZE 17
#define CHECK_STORE if(fp_map == NULL || ip_map == NULL || n_map == NULL) return QLUSER_STATUS_NOT_INITIALISED;

int qluser_store_initialise(const char *db_path, const char *key_path, unsigned int flags)
{
    int ret;
    if(fp_map != NULL) return QLUSER_STATUS_ALREADY_INIT;

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
        ret = QLUSER_STATUS_INVALID_KEYSTORE;
        goto c3;
    }


    /* Return if we got here */
    return QLUSER_STATUS_SUCCESS;

    /* Clean up the mess we made and return failure */
    c3: cuckoo_free(n_map, CUCKOO_NO_CB);
    c2: cuckoo_free(ip_map, CUCKOO_NO_CB);
    c1: cuckoo_free(fp_map, CUCKOO_NO_CB);
    return ret;
}


int qluser_store_adduser(struct qluser_t *user, const char *fp)
{
    CHECK_STORE

    /* Check if the user already exists */
    if(cuckoo_contains(fp_map, fp) == 0) {
    }

    cuckoo_insert(fp_map, fp, user);
    cuckoo_insert(n_map, user->name, user);

    char *ip = "<foo>";
    cuckoo_insert(ip_map, ip, user);
}