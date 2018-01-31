/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _VALIDATE_H
#define _VALIDATE_H

#include "defines.h"

/**
 * check if the character is a number 0-9
 *
 * @retval QL_FALSE is not a number
 * @retval QL_TRUE is a number
 */
QL_BOOL qlutils_is_ascii_digit(const char c);

QL_BOOL qlutils_is_ascii_alpha(const char c);

QL_BOOL qlutils_is_ascii_alphanum(const char c);

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
 * check char for problematic entities
 */
DEPRECATED int validate_char_problematic (char mychar);

#endif // _VALIDATE_H