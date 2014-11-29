/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_URLCODE
#define _QAULLIB_URLCODE

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


/********************************************//**
 * decode and encode url characters
 ***********************************************/

/**
 * Converts a hex character to its integer value
 */
char Qaullib_UrlFromHex(char chr);

/**
 * Converts an integer value to its hex character
 */
char Qaullib_UrlToHex(char code);

/**
 * Returns a url-encoded version of str
 * IMPORTANT: be sure to free() the returned string after use
 */
char *Qaullib_UrlEncode(char *str);

/**
 * Returns a url-decoded version of str
 * IMPORTANT: be sure to free() the returned string after use
 */
char *Qaullib_UrlDecode(char *str);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
