/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef QAUL_QCRY_HELPER_H
#define QAUL_QCRY_HELPER_H

/***************** QCRY ERROR CODES *****************/

#include <mbedtls/pk.h>

#define QCRY_STATUS_OK                  0
#define QCRY_STATUS_ERROR               1
#define QCRY_STATUS_FATAL               2

#define QCRY_STATUS_NOT_IMPLEMENTED     3

#define QCRY_STATUS_KEYGEN_FAILED       4
#define QCRY_STATUS_BFR_TOO_SMALL       10
#define QCRY_STATUS_INVALID_PARAMS      12

#define QCRY_STATUS_TARGET_EXISTS       13
#define QCRY_STATUS_SEED_FAILED         14
#define QCRY_STATUS_INVALID_CTX         16
#define QCRY_STATUS_INVALID_TARGET      18
#define QCRY_STATUS_INVALID_USERNO      19

#define QCRY_STATUS_INVALID_KEYS        21
#define QCRY_STATUS_MALLOC_FAIL         22
#define QCRY_STATUS_KEY_BUSY            24

#define QCRY_STATUS_DECODE_FAILED       28
#define QCRY_STATUS_ENCODE_FAILED       29

#define QCRY_STATUS_KS_NOTFOUND         30
#define QCRY_STATUS_KS_EXISTS           31

#define QCRY_STATUS_SIGN_OK             0
#define QCRY_STATUS_SIGN_BOGUS          40

/***************** QCRY ERROR CODES *****************/

/* Flags used by the key generators */
#define QCRY_KEYS_AES             100
#define QCRY_KEYS_ECC             120
#define QCRY_KEYS_RSA             140
#define QCRY_KEYS_PREST_ON        180
#define QCRY_KEYS_PREST_OFF       230
#define QCRY_KEYS_QUIET_ON        350
#define QCRY_KEYS_QUIET_OFF       370
#define QCRY_KEYS_PERM_ON         410
#define QCRY_KEYS_PERM_OFF        415
#define QCRY_CIPH_CBC             425

/* Generic helper macros for all crypto code */

#define MAGICK_NO   3
#define MIN_BFR_S   4
#define MAX_TIMEOUT 500
#define TIME_SLEEP  50
#define QAUL_FP_LEN 64

#define QAUL_MAX_MSG_LENGTH     1024    // Overkill
#define QAUL_SIGN_HASH_LEN      32      // SHA-256
#define QAUL_SIGNATURE_LEN      512     // Defined by how RSA works
#define QAUL_PUBKEY_LEN         16000

#define CHECK_SANE \
    if(ctx->mgno != MAGICK_NO) return QCRY_STATUS_INVALID_CTX; \
    int ret = 0;

/**
 * This macro checks if a buffer is full and should be increased in size.
 * Additionally it checks if a buffer is WAY too big for it's occupancy and
 * shrinks it. This macro should get used whenever things are
 * stored or removed from list buffers
 */
#define CHECK_BUFFER(type, bfr, max_s, curr_s) \
    { if(curr_s >= max_s) { \
        max_s *= 2; type *tmp = calloc(sizeof(type), max_s); \
        if(!tmp) return QCRY_STATUS_MALLOC_FAIL; \
        memcpy(tmp, bfr, sizeof(type) * curr_s); \
        free(bfr); \
        bfr = tmp; \
    } else if(curr_s * 3<= max_s) { max_s /= 2 ; \
    if(max_s < MIN_BFR_S) max_s = MIN_BFR_S; \
        type *tmp = calloc(sizeof(type), max_s); \
        if(!tmp) return QCRY_STATUS_MALLOC_FAIL; \
        memcpy(tmp, bfr, \
        sizeof(type) * curr_s); \
        free(bfr); \
        bfr = tmp; } \
    }

/**
 * create @a string from @a hash
 *
 * @retval 1 on success
 * @retval 0 on error
 */
int QCry_HashToString(unsigned char *hash, char *string);

/**
 * reconverts a hash string @a string to the @a hash
 *
 * @retval 1 on success
 * @retval 0 on error
 */
int QCry_StringToHash(char *string, unsigned char *hash);


/***
 *
 *
 *
 *
 * BASE 64 ENCODERS FOR HASHES!
 *
 * These have been depreciated. Use mbedtls_base64 instead!
 *
 *
 */

int qcry_base64_enclength(int str_len);

int qcry_base64_encode(char *buffer, const char *src, int src_len);

int qcry_base64_declen(const char *encoded);

int qcry_base64_decode(char *buffer, const char *encoded, int enc_len);


/**
 * Load a public key from disk storage inside a particular path.
 *
 * @param pub The pubkey context that will be filled
 * @param path The path hirarchy to load a key from
 * @param fp Fingerprint to load (<fingerprint>.pub is the filename)
 *
 * @return 0 for success or different error codes
 */
int qcry_load_pubkey(mbedtls_pk_context **pub, const char *path, const char *fp);


/**
 * Inverse function of #qcry_load_pubkey(...) which stores a public key as
 * plaintext in a file found in <path>/<fingerprint>.pub which can be loaded
 * again later.
 *
 * @param pub
 * @param path
 * @param fp
 * @return
 */
int qcry_save_pubkey(mbedtls_pk_context *pub, const char *path, const char *fp);


/**
 * This is a utility function that can be used to dump a public key context
 * into a plaintext buffer. This function is called when dumping public keys
 * to disk or when sending pubkeys over the network.
 *
 * A size is returned to indicate how big a buffer needs to be
 * allocated for this particular key.
 *
 * @param buffer
 * @param ksize
 * @param key
 * @return
 */
int qcry_serialise_pubkey(unsigned char **buffer, size_t *ksize, mbedtls_pk_context *key);


/**
 * The inverse function of #qcry_serialise_pubkey(...) which turns a plaintext
 * buffer back into an mbedtls pubkey context. This is used when loading pubkeys
 * from disk and also when receiving them from a networked node (foreign identities)
 *
 * @param key
 * @param ksize
 * @param buffer
 * @return
 */
int qcry_deserialise_pubkey(mbedtls_pk_context **key, size_t ksize, unsigned char *buffer);


#endif //QAUL_QCRY_HELPER_H
