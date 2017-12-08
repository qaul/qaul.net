/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_QLCRY_H
#define QAUL_QLCRY_H


/**
 * An enum that describes information handled
 * by the qaul crypto modules
 */
typedef enum ql_crydata_t {
    FINGERPRINT,
    PUBKEY,
};

/**
 * An enum to describe different key types
 */
typedef enum ql_crykey_t {
    RSA, AES256
};

/**
 * A structure that contains a public key
 *
 * The actual key formatting is specific to an implemetation
 * and as such shadowed to the outside. The crypto module will
 * cast the blob to whatever format is required at the time
 */
typedef struct ql_pubkey {
    enum ql_crykey_t type;
    void *blob;
};

/**
 * A structure that contains a secret (private) key
 *
 * The actual key formatting is specific to an implemetation
 * and as such shadowed to the outside. The crypto module will
 * cast the blob to whatever format is required at the time
 */
typedef struct ql_seckey {
    enum ql_crykey_t type;
    void *blob;
};

/**
 * A structure that contains a complete user keypair
 */
typedef struct ql_keypair {
    enum ql_crykey_t type;
    struct ql_pubkey *pub;
    struct ql_seckey *sec;
};

/**
 * The context structure used for the crypto-module
 */
typedef struct ql_cry_ctx {
    
};



int qlcry_start_session(struct ql_cry_ctx *ctx, struct ql_keypair *owner, struct ql_keypair *target);

int ql_cry_stop_session(struct ql_cry_ctx *);

//start_session
//
//        stop_session
//
//sign_data
//
//        verify_data
//
//encrypt_data
//
//        decrypt_data

#endif //QAUL_QLCRY_H
