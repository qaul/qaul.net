/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * validation functions for input strings
 */

#include <stdio.h>
#include <string.h>


/**
 * validate IP argument
 */
int validate_ip (const char* str);

/**
 * validate interface argument
 */
int validate_interface (const char* str);

/**
 * validate service argument
 */
int validate_service (const char* str);

/**
 * validate essid argument
 */
int validate_essid (const char* str);

/**
 * validate service argument
 */
int validate_bssid (const char* str);

/**
 * validate number argument
 */
int validate_number (const char* str);

/**
 * validate path argument
 */
int validate_path (const char* str);

/**
 * check if the char is a number
 */
int validate_char_number (char mychar);

/**
 * check if the char is an ascii letter
 */
int validate_char_letter (char mychar);

/**
 * check char for problematic entities
 */
int validate_char_problematic (char mychar);
