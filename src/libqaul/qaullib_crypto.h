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
 * create a sha1 @a hash from a file @a filepath.
 *
 * @retval 1 on success
 * @retval 0 on error
 */
int Ql_sha1_file(char *filepath, unsigned char *hash);

/**
 * create @a string from @a hash
 *
 * @retval 1 on success
 * @retval 0 on error
 */
int Ql_HashToString(unsigned char *hash, char *string);

/**
 * reconverts a hash string @a string to the @a hash
 *
 * @retval 1 on success
 * @retval 0 on error
 */
int Ql_StringToHash(char *string, unsigned char *hash);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
