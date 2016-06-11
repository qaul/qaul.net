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
#define QCRY_STATUS_OK                 0x0
#define QCRY_STATUS_GEN_ERROR          -1
#define QCRY_STATUS_KEYGEN_FAILED      0xFF2
#define QCRY_STATUS_ENTROPY_FAILED     0xFF3
#define QCRY_STATUS_RCRT_FAILED        0xFF4
#define QCRY_STATUS_BUFFER_TOO_SMALL   0xFF5
#define QCRY_STATUS_INVALID_PARAMS     0xFF6
#define QCRY_STATUS_SEED_FAILED        0xFF7
#define QCRY_STATUS_INVALID            0xFF8

// #define QCRY_STATUS_OK                 0xFF6
// #define QCRY_STATUS_OK                 0xFF7
// #define QCRY_STATUS_OK                 0xFF8
// #define QCRY_STATUS_OK                 0xFF9
// #define QCRY_STATUS_OK                 0xFFA
// #define QCRY_STATUS_OK                 0xFFB
// #define QCRY_STATUS_OK                 0xFFC
// #define QCRY_STATUS_OK                 0xFFD
// #define QCRY_STATUS_OK                 0xFFE

#define QCRY_KEYS_AES             0x10
#define QCRY_KEYS_ECC             0x11
#define QCRY_KEYS_RSA             0x12
#define QCRY_KEYS_PREST_ON        0x20
#define QCRY_KEYS_PREST_OFF       0x21
#define QCRY_KEYS_QUIET_ON        0x30
#define QCRY_KEYS_QUIET_OFF       0x31
#define QCRY_KEYS_PERM_ON         0x40
#define QCRY_KEYS_PERM_OFF        0x41

#define QCRY_CIPH_CBC             0x51
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
