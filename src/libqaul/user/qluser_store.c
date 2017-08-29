#include "../qaullib_private.h"
#include "qluser_store.h"


/** Holds data about a node */
struct qluser_node_t {
    union olsr_ip_addr  ip;
    struct qluser_t     **identities;
};

/** Holds data about a user identity */
struct qluser_t {
    const char *fp;
    const char *pubkey;
    char *name;
    struct qluser_node_t *node;
};

/** Store struct that references all node and user data */
typedef struct qluser_store {

} qluser_store;


/** Keep a static reference to a user store */
static struct qluser_store *store;


int qluser_store_initialise(const char *db_path, const char *key_path, unsigned int flags)
{
    /** Check if a store already exists */
    if(store != NULL) return QLUSER_STATUS_ALREADY_INIT;

    /* Malloc memory for the store */
    store = (qluser_store*) malloc(sizeof(qluser_store) * 1);
    memset(store, 0, sizeof(qluser_store));

    return QLUSER_STATUS_SUCCESS;
}