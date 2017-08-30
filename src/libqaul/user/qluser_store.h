/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QLUSER_STORE_H
#define QLUSER_STORE_H

/* Some generic error and status messages */
#include <mbedtls/pk.h>

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
#define QLUSER_PUBKEY_EXISTS        0x23
#define QLUSER_PUBKEY_NOT_FOUND     0x24


/** Holds data about a node */
typedef struct qluser_node_t {
    union olsr_ip_addr  *ip;
    struct qluser_t     **identities;
    uint32_t            len, used;
} qluser_node_t;


/** Holds data about a user identity */
typedef struct qluser_t {
    char *name;
    const char *fp;

    // TODO: Maybe change this to a char buffer that we just keep (de)serialising?
    mbedtls_pk_context *pubkey;
    struct qluser_node_t *node;
} qluser_t;


/** Describes the different trust levels between users */
typedef enum qluser_trust_t {

    /** No public key is known */
    UNKNOWN,

    /** TOFU: Trust on first use - but not really */
    PARTIAL,

    /** Manually verified and this user checks out */
    VERIFIED
};


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
 * @return      <0> for SUCCESS or ERROR code
 */
int qluser_store_adduser(const char *fp, const char *name);


/** Functions to fill up user data */
int qluser_store_add_ip(struct qluser_t user, union olsr_ip_addr *ip);
int qluser_store_add_name(struct qluser_t user, const char *name);
int qluser_store_add_pubkey(struct qluser_t user, const char *pubkey);
int qluser_store_add_trustlvl(struct qluser_t user, enum qluser_trust_t);


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
