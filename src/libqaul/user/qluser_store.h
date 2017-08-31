/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QLUSER_STORE_H
#define QLUSER_STORE_H

/* Some generic error and status messages */
#include <mbedtls/pk.h>
#include <cuckoo.h>

#define QLUSER_SUCCESS              0x0
#define QLUSER_ERROR                0x1
#define QLUSER_INVALID_PARAMS       0x2
#define QLUSER_ALREADY_INIT         0x3
#define QLUSER_DB_INVALID           0x4
#define QLUSER_DB_LOCKED            0x5
#define QLUSER_INVALID_KEYSTORE     0x6
#define QLUSER_NOT_INITIALISED      0x7
#define QLUSER_MALLOC_FAILED        0x8

/* Specific insert related issues */
#define QLUSER_INSERT_FAILED        0x20
#define QLUSER_USER_EXISTS          0x21
#define QLUSER_USER_NOT_FOUND       0x22
#define QLUSER_REMOVE_FAILED        0x23
#define QLUSER_PUBKEY_EXISTS        0x2A
#define QLUSER_PUBKEY_NOT_FOUND     0x2B
#define QLUSER_NODE_NOT_FOUND       0x3A


/** Holds data about a node */
typedef struct qluser_node_t {

    // TODO: Think about abstraction layer on IPs instead of olsr IPs
    union olsr_ip_addr  *ip;

    // FIXME: Shadow this inside the src file instead of inside the header?
    cuckoo_map          *ids;       // Indexed by fingerprint
} qluser_node_t;


/** Describes the different trust levels between users */
typedef enum qluser_trust_t {

    /** No public key is known */
            UNKNOWN = 0,

    /** TOFU: Trust on first use - but not really */
            PARTIAL = 1,

    /** Manually verified and this user checks out */
            VERIFIED = 2
};


/** Holds data about a user identity */
typedef struct qluser_t {
    char *name;
    const char *fp;

    // TODO: Maybe change this to a char buffer that we just keep (de)serialising?
    mbedtls_pk_context *pubkey;
    struct qluser_node_t *node;

    /* Store 4 different trust levels per user */
    // TODO: Implement any of this :)
    enum qluser_trust_t k_trust;
    int32_t             f_trust;
    int32_t             tl_trust;
    int32_t             msg_trust;
} qluser_t;


/**
 * Initialise the user store and fill it with already known users in
 * a network from a database and public keystore path. All other functions
 * will fail if this function was not called first.
 *
 * @param db_path Provide the path to the persistent user db
 * @param key_path Provide the path to the public keystore folder
 * @param flags Provide some configuration flags. See docs for details
 * @return Status return codesu
 */
int qluser_store_initialise(const char *db_path, const char *key_path, unsigned int flags);


/**
 * This function adds the bare-bone data required into two of the lookup tables
 * which allows future data to be added as well. To retrieve the newly created
 * used after the insertion please use @link{qluser_store_getby_fp}
 *
 * @param fp    The fingerprint of this user
 * @param name  The username of this user
 * @return      Status return code
 */
int qluser_store_adduser(const char *fp, const char *name);


/**
 * Remove a user from all storage tables, cleaning up all pre-allocated
 * memory from the tables and qluser_t structs. It also removes the reference
 * left in qluser_node_t and cleans it up if it was the only user on this
 * node.
 *
 * @param fp    The fingerprint of the user to remove
 * @return      Status return code
 */
int qluser_store_rmuser(const char *fp);


/**
 * Add an IP address to a user which represents a node on
 * the network. If a user has changed nodes, the old one
 * will be removed before
 *
 * @param fp    The fingerprint of the user
 * @param ip    IP data of the node involved
 * @return      Status return code
 */
int qluser_store_add_ip(const char *fp, union olsr_ip_addr *ip);


/**
 * Add a public key to a user that is also stored to disk in the
 * persistent keystore for later loading. In case pubkey storage
 * fails it will return an error code from the QCRY list which
 * indicates what type of error occured during the write process.
 *
 * @param fp
 * @param pubkey
 * @param trust
 * @return
 */
// TODO: Wrap mbdetls_pk_context in something that hides internals
int qluser_store_add_pubkey(const char *fp, mbedtls_pk_context *pubkey, enum qluser_trust_t trust);


/** Functions to fill up user data */
int qluser_store_set_keytrust(const char *fp, enum qluser_trust_t trust);
int qluser_store_set_msgtrust(const char *fp, int32_t trust);
int qluser_store_set_filetrust(const char *fp, int32_t trust);
int qluser_store_set_ltrust(const char *fp, int32_t trust);


/** Functions to search users with */
int qluser_store_getby_fp(struct qluser_t *user, const char *fp);
int qluser_store_getby_name(struct qluser_t *user, const char *name);
int qluser_store_getby_ip(struct qluser_t *user, union olsr_ip_addr *ip);


/**
 * Remove a user from the current non-persistent user storage
 *
 * @param user The user to delete
 * @return
 */
int qluser_store_rm(struct qluser_t *user);



/**
 * Remove the user from the current user storage as well as scrub
 * all information about this user from the persistent database as
 * well
 *
 * @param user The user to delete
 * @return
 */
int qluser_store_rmall(struct qluser_t *user);


/**
 * Free all resources from the user store and lock the database.
 *
 * @return Status return code
 */
int qluser_store_free();

#endif // QLUSER_STORE_H
