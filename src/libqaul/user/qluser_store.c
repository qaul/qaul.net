#include "qluser_store.h"

#include <cuckoo.h>
#include "../crypto/qcry_keystore.h"

/** Holds data about a node */
typedef struct qluser_node_t {
    union olsr_ip_addr  *ip;
    struct qluser_t     **identities;
};

/** Holds data about a user identity */
struct qluser_t {
    const char *fp;
    const char *pubkey;
    char *name;
    struct qluser_node_t *node;
};


/* Static storage context for all indexable fields */
static cuckoo_map *fp_map, *ip_map, *n_map;
#define INIT_MAP_SIZE 17

int qluser_store_initialise(const char *db_path, const char *key_path, unsigned int flags)
{
    int ret;

    // TODO: Get cflags and ms from flags & config?
    uint32_t cflags = CUCKOO_DEFAULT | CUCKOO_TABLES_THREE;
    uint32_t ms = INIT_MAP_SIZE;

    ret = cuckoo_init(&fp_map, ms, cflags);
    if(ret) goto c1;

    ret = cuckoo_init(&ip_map, ms, cflags);
    if(ret) goto c2;

    ret = cuckoo_init(&n_map, ms, cflags);
    if(ret) goto c3;

    /* Return if we got here */
    return 0;

    /* Clean up the mess we made and return failure */
    c3: cuckoo_free(n_map, CUCKOO_NO_CB);
    c2: cuckoo_free(ip_map, CUCKOO_NO_CB);
    c1: cuckoo_free(fp_map, CUCKOO_NO_CB);
    return 42;
}
