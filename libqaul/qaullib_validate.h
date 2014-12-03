/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * Collection of validation helper functions for qaul.net
 */

#ifndef _QAULLIB_VALIDATE
#define _QAULLIB_VALIDATE

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


/**
 * check if the character is a ASCII a-z, A-Z or 0-9 character
 *
 * @retval 0 is not a lowercase a-z letter
 * @retval 1 is a lowercase a-z letter
 */
int Qaullib_ValidateCharASCIILetterOrNumber(char *character);

/**
 * check if the character is a ASCII a-z or A-Z letter
 *
 * @retval 0 is not a lowercase a-z letter
 * @retval 1 is a lowercase a-z letter
 */
int Qaullib_ValidateCharASCIILetter(char *character);

/**
 * check if the character is a lowercase ASCII a-z character
 *
 * @retval 0 is not a lowercase a-z letter
 * @retval 1 is a lowercase a-z letter
 */
int Qaullib_ValidateCharLowercaseASCII(char *character);

/**
 * check if the character is a uppercase ASCII A-Z character
 *
 * @retval 0 is not a uppercase a-z letter
 * @retval 1 is a uppercase a-z letter
 */
int Qaullib_ValidateCharUppercaseASCII(char *character);

/**
 * check if the character is a number 0-9
 *
 * @retval 0 is not a number
 * @retval 1 is a number
 */
int Qaullib_ValidateCharNumber(char *character);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
