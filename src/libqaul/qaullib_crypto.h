/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_CRYPTO
#define _QAULLIB_CRYPTO

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


/**
 * create @a string from @a hash
 *
 * @retval 1 on success
 * @retval 0 on error
 */
int Qaullib_HashToString(unsigned char *hash, char *string);

/**
 * reconverts a hash string @a string to the @a hash
 *
 * @retval 1 on success
 * @retval 0 on error
 */
int Qaullib_StringToHash(char *string, unsigned char *hash);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
