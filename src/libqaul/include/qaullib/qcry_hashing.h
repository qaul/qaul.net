#ifndef QAUL_QCRY_HASHING_H
#define QAUL_QCRY_HASHING_H

#include <stdlib.h>

struct qcry_hash_ctx {
    /** Things the user should set */
    unsigned int            hash;
    unsigned int            encoding;
    void                    *md_ctx;
    char                    *salt;

    /** Some things required to run the show */
    char                    *curr_bfr;
    size_t                  bfr_s, bfr_occ;
    short                   mgno;
};

/** Define some hash functions we provide */
#define QCRY_HASH_SHA256    (1 << 1)
#define QCRY_HASH_SHA512    (1 << 2)

#define QCRY_SHA512_LENGTH  64
#define QCRY_SHA256_LENGTH  32

/** Encodings we support. Base64 requires additional import */
#define QCRY_HASH_BINARY    (1 << 1)
#define QCRY_HASH_HEX       (1 << 2)
#define QCRY_HASH_BASE64    (1 << 3)

/**
 * Create and delete a hashing context, initialised for a hash function and a salt
 */
int qcry_hashing_init(struct qcry_hash_ctx *ctx, unsigned int hash, const char *salt);
int qcry_hashing_free(struct qcry_hash_ctx *ctx);

/** Append a message to the buffer **/
int qcry_hashing_append(struct qcry_hash_ctx *ctx, const char *msg);
int qcry_hashing_clear(struct qcry_hash_ctx *ctx);

/** Read the current state of the buffer. Can return NULL. Should only be used for debugging! **/
const char *qcry_hashing_buffer(struct qcry_hash_ctx *ctx);

/** Returns the length for the selected hash function digest */
size_t qcry_hashing_getlen(unsigned int hash);

/** Build the buffer to a hash and encode it into a return buffer */
int qcry_hashing_build(struct qcry_hash_ctx *ctx, unsigned int encoding, char *(*buffer));

#endif //QAUL_QCRY_HASHING_H
