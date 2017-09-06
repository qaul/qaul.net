/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _VALIDATE_H
#define _VALIDATE_H

#include "deprecated.h"

/**
 * validation functions for input strings
 */

/**
 * validate IP argument
 */
DEPRECATED int validate_ip (const char* str);

/**
 * validate IP address
 * return QL_TRUE address is valid
 *        QL_FALSE address was not parseable
 */
int validate_ipv4 (const char* str);
int validate_ipv6 (const char* str);

/**
 * validate interface argument
 */
DEPRECATED int validate_interface (const char* str);

/**
 * validate service argument
 */
DEPRECATED int validate_service (const char* str);

/**
 * validate essid argument
 */
DEPRECATED int validate_essid (const char* str);

/**
 * validate service argument
 */
DEPRECATED int validate_bssid (const char* str);

/**
 * validate number argument
 */
DEPRECATED int validate_number (const char* str);

/**
 * validate path argument
 */
DEPRECATED int validate_path (const char* str);

/**
 * check if the char is a number
 */
DEPRECATED int validate_char_number (char mychar);

/**
 * check if the char is an ascii letter
 */
DEPRECATED int validate_char_letter (char mychar);

/**
 * check char for problematic entities
 */
DEPRECATED int validate_char_problematic (char mychar);


/**
 * check if the character is a ASCII a-z, A-Z or 0-9 character
 *
 * @retval 0 is not a lowercase a-z letter
 * @retval 1 is a lowercase a-z letter
 */
DEPRECATED int Qaullib_ValidateCharASCIILetterOrNumber(char *character);

/**
 * check if the character is a ASCII a-z or A-Z letter
 *
 * @retval 0 is not a lowercase a-z letter
 * @retval 1 is a lowercase a-z letter
 */
DEPRECATED int Qaullib_ValidateCharASCIILetter(char *character);

/**
 * check if the character is a lowercase ASCII a-z character
 *
 * @retval 0 is not a lowercase a-z letter
 * @retval 1 is a lowercase a-z letter
 */
DEPRECATED int Qaullib_ValidateCharLowercaseASCII(char *character);

/**
 * check if the character is a uppercase ASCII A-Z character
 *
 * @retval 0 is not a uppercase a-z letter
 * @retval 1 is a uppercase a-z letter
 */
DEPRECATED int Qaullib_ValidateCharUppercaseASCII(char *character);

/**
 * check if the character is a number 0-9
 *
 * @retval 0 is not a number
 * @retval 1 is a number
 */
DEPRECATED int Qaullib_ValidateCharNumber(char *character);

#endif // _VALIDATE_H