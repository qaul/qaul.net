/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef _QAUL_QLFORMAT_H
#define _QAUL_QLFORMAT_H

/********************** GENERAL **********************/

#include <qaul/error.h>
#include <cuckoo.h>
#include <glob.h>

// A simple value that can be checked against to make
// sure that a struct has been properly initialised
#define QL_MODULE_INITIALISED 0x1337
#define CHECK(field, ret) { if((field) == NULL) return ret; }
#define INITIALISED(field) { if((field)->initialised != QL_MODULE_INITIALISED) return NOT_INITIALISED; }

typedef enum ql_operation_t {

    // Cryptography operations
    ENCRYPT, DECRYPT, SIGN, VERIFY, INVALID

} ql_operation_t;


/**
 * Represents a path on the filesystem
 */
typedef struct ql_path {
    enum qaul_os *os;
    char **snippets;
    size_t num, size;
};

typedef struct ql_file {
    const char *name;
    const char *ext;
};

typedef struct ql_file_list {
    struct ql_file **files;
    size_t num, size;
};

/********************** NETWORKING **********************/

/**
 * Represents an IP address on the network
 */
typedef union ql_ip {
    short   v4[4];
    short   v6[16];
} ql_ip;

/**
 * Represents a node on the qaul network. Additionally to the
 * ip-address of the node it also references all users that are
 * present on the node
 */
typedef struct ql_node {
    union ql_ip *ip;
    cuckoo_map *ids;
} ql_node;


/********************** USER MANAGEMENT **********************/

typedef enum qluser_trust_t {

    /** No public key is known */
    UNKNOWN = 0,

    /** TOFU: Trust on first use - but not really */
    PARTIAL = 1,

    /** Manually verified and this user checks out */
    VERIFIED = 2
};

typedef enum qluser_t {
    INTERNAL, EXTERNAL
};


/**
 * A union that either represents an internal or an
 * external user, never both
 */
typedef union ql_user {
    struct ql_user_internal *intern;
    struct ql_user_external *ext;
} ql_user;


typedef struct ql_user_internal {
    char *username;
    char *fingerprint;
    struct ql_keypair *keypair;
} ql_user_internal;


typedef struct ql_user_external {
    char *username;
    char *fingerprint;
    struct ql_pubkey *pubkey;
    enum qluser_trust_t trust;
} ql_user_external;


/**
 * The userstore context that contains a bunch of tables
 * that are used to map all sorts of data to other sorts
 * of data.
 */
typedef struct ql_userstore {
    /* Map fp to user */
    cuckoo_map *fp_map;

    /* Map ip to user */
    cuckoo_map *ip_map;

    /* Map name to user */
    cuckoo_map *n_map;

    /* Map node to user */
    cuckoo_map *node_map;

    const struct ql_path *keys, *db;
} ql_userstore;


/********************** CRYPTO CORE **********************/


/** An enum that describes key types */
typedef enum ql_cipher_t {
    PK_RSA  = (1 << 1),
    ECDSA   = (1 << 2),
    AES256  = (1 << 3),
    NONE    = (1 << 4)
} ql_cipher_t;


/**
 * Contains the key sizes for different types
 */
static int QL_KEYLENGTHS[] = { 2048, 192, 256 };


/**
 * A structure that contains a public key
 *
 * The actual key formatting is specific to an implemetation
 * and as such shadowed to the outside. The crypto module will
 * cast the blob to whatever format is required at the time
 */
typedef struct ql_pubkey {
    enum ql_cipher_t type;
    void *blob;
} ql_pubkey;


/**
 * A structure that contains a secret (private) key
 *
 * The actual key formatting is specific to an implemetation
 * and as such shadowed to the outside. The crypto module will
 * cast the blob to whatever format is required at the time
 */
typedef struct ql_seckey {
    enum ql_cipher_t type;
    void *blob;
} ql_seckey;


/**
 * A structure that contains a complete user keypair
 */
typedef struct ql_keypair {
    enum ql_cipher_t type;
    struct ql_pubkey *pub;
    struct ql_seckey *sec;
} ql_keypair;


/**
 * Stores the result of a cryptographic operation.
 * Contains a reference fingerprint to associate result with a user
 */
typedef struct ql_crypto_result {
    const char *fp;
    size_t length;
    unsigned char *data;
} ql_crypto_result;


/**
 * The context struct for a crypto session
 *
 * It is initialised with an owner and a target, then
 * configured to a mode. All functions performed on it
 * afterwards can only be done if supported by the mode.
 *
 * @mbedtls_ctx is internally cast to whatever
 *                implementation is required
 */
typedef struct qlcry_session_ctx {
    unsigned short initialised;
    struct ql_user_internal *owner;
    struct ql_user_external **participants;
    size_t no_p, array_p;
    enum ql_cipher_t mode;

    /* crypto internals */
    void *random;
    void *entropy;

    ql_crypto_result **buffer;
    size_t buffer_length;
    ql_operation_t buffer_type;
} qlcry_session_ctx;

/********************** QAUL CORE **********************/

typedef struct qlauth_ctx {

} qlauth_ctx;


/********************** QAUL CORE **********************/


/**
 * Internal qaul structure that holds all other context structures
 */
typedef struct ql_core {

} ql_core;


#endif //QAUL_QLFORMAT_H
