/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QCRY_HELPER_
#define _QCRY_HELPER_

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/** ALL CONSTANT DEFINITIONS FOR THE QCRY NAMESPACE */
#define QCRY_STATUS_OK                  0
#define QCRY_STATUS_GEN_ERROR           1
#define QCRY_STATUS_KEYGEN_FAILED       6
#define QCRY_STATUS_ENTROPY_FAILED      8
#define QCRY_STATUS_RCRT_FAILED         10
#define QCRY_STATUS_BFR_TOO_SMALL       12
#define QCRY_STATUS_INVALID_PARAMS      14
#define QCRY_STATUS_SEED_FAILED         16
#define QCRY_STATUS_CTX_INVALID         18
#define QCRY_STATUS_INVALID             20
#define QCRY_STATUS_INVALID_KEYS        21
#define QCRY_STATUS_MALLOC_FAIL         22


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


#define QCRY_KEY_LEN { 2048, 192, 256 }

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

#ifdef __cplusplus
}
#endif // __cplusplus

#endif
