/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef _QAUL_QLFORMAT_H
#define _QAUL_QLFORMAT_H

/********************** GENERAL **********************/

#include <qaul/error.h>
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
    enum qaul_os os;
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



/********************** USER MANAGEMENT **********************/


/**
 * A structure that contains user information
 */
typedef struct ql_user {
    char *username;
    char *fingerprint;
    struct ql_keypair *keypair;
} ql_user;


/**
 * Describes a piece of data attached to a user
 *
 * TODO: Maybe move to qaul.h
 */
typedef enum ql_userdata_t {
    FINGERPRINT,
    PUBKEY,
} ql_userdata_t;


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
    struct ql_user *owner;
    struct ql_user **participants;
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
