/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * This file contains all validation functions for qaullib.
 */

#include "qaul/utils/validate.h"
#include <stdio.h>
#include <string.h>

/**
 * check if the character is a number 0-9
 *
 * @retval 0 is not a number
 * @retval 1 is a number
 */
QL_BOOL qlutils_is_ascii_digit(const char c) {
    if(c < '0' || c > '9') {
        return QL_FALSE;
    }
    return QL_TRUE;
}

QL_BOOL qlutils_is_ascii_alpha(const char c) {
    if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
	return QL_TRUE;
    }
    return QL_FALSE;
}

QL_BOOL qlutils_is_ascii_alphanum(const char c) {
    return (qlutils_is_ascii_digit(c) || qlutils_is_ascii_alpha(c));
}

int validate_ip (const char* str)
{
    int i;

    // check length
    if(strlen(str)>15)
    {
        printf("argument too long\n");
        return 0;
    }

    // check numbers and dots
    for(i=0; i<strlen(str); i++)
    {
        if(!qlutils_is_ascii_digit(str[i]) && strncmp(&str[i], ".", 1) != 0)
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}


int validate_interface (const char* str)
{
    int i;

    // check length
    if(strlen(str)>20)
    {
        printf("argument too long\n");
        return 0;
    }

    // check numbers and dots
    for(i=0; i<strlen(str); i++)
    {
        if(!qlutils_is_ascii_alphanum(str[i]))
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}

/*
int validate_service (const char* str)
{
    int i;
    
    // check length
    if(strlen(str)>50)
    {
        printf("argument too long\n");
        return 0;
    }
    
    // check numbers and dots
    for(i=0; i<strlen(str); i++)
    {
        if(validate_char_problematic(str[i]) == 0)
        {
            printf("invalid character %d : %c\n", i, str[i]);
            return 0;
        }
    }
    return 1;
}
*/

int validate_number (const char* str)
{
    int i;

    // check length
    if(strlen(str)>10)
    {
        printf("argument too long\n");
        return 0;
    }

    // check numbers and dots
    for(i=0; i<strlen(str); i++)
    {
        if(!qlutils_is_ascii_digit(str[i]))
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}

/*
int validate_path (const char* str)
{
    int i;

    // check length
    if(strlen(str)>200)
    {
        printf("argument too long\n");
        return 0;
    }

    // check if it is a valid path
    if(strncmp(&str[0], "/", 1) != 0)
    {
        printf("not an absolute path\n");
        return 0;
    }

    for(i=0; i<strlen(str); i++)
    {
        if(
           strncmp(&str[i], ":", 1) == 0
           )
        {
            printf("invalid character\n");
            return 0;
        }
    }

    return 1;
}
*/

int validate_essid (const char* str)
{
    int i;

    // check length
    if(strlen(str)>50)
    {
        printf("argument too long\n");
        return 0;
    }

    // check numbers and dots
    for(i=0; i<strlen(str); i++)
    {
        if(!qlutils_is_ascii_alphanum(str[i]) &&
           strncmp(&str[i], ".", 1) != 0 &&
           strncmp(&str[i], "-", 1) != 0 &&
           strncmp(&str[i], "_", 1) != 0
           )
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}

/*

int validate_bssid (const char* str)
{
    int i;

    // check length
    if(strlen(str)!=17)
    {
        printf("BSSID not correct\n");
        return 0;
    }

    // check validity of positions
    for(i=0; i<strlen(str); i++)
    {
		if(i == 2 || i == 5 || i == 8 || i == 11 || i == 14 )
		{
			// needs to be colon
			if(strncmp(&str[i], ".", 1) != 0)
			{
				printf("BSSID: invalid character at position %i\n", i);
				return 0;
			}
		}
        else
        {
			// needs to be a number or a letter
			if(validate_char_number(str[i]) == 0 && validate_char_letter(str[i]) == 0)
			{
				printf("BSSID: invalid character at position %i\n", i);
				return 0;
			}
		}
    }
    return 1;
}
*/

/*

int validate_char_problematic (char mychar)
{
    if(strncmp(&mychar, "\"", 1) == 0)
        return 0;
    if(strncmp(&mychar, "'", 1) == 0)
        return 0;
    if(strncmp(&mychar, "`", 1) == 0)
        return 0;
    if(strncmp(&mychar, ";", 1) == 0)
        return 0;
    if(strncmp(&mychar, "\\", 1) == 0)
        return 0;
    if(strncmp(&mychar, "&", 1) == 0)
        return 0;
    if(strncmp(&mychar, ">", 1) == 0)
        return 0;
    if(strncmp(&mychar, "<", 1) == 0)
        return 0;
    if(strncmp(&mychar, "|", 1) == 0)
        return 0;
    if(strncmp(&mychar, "$", 1) == 0)
        return 0;
    if(strncmp(&mychar, "*", 1) == 0)
        return 0;
    if(strncmp(&mychar, "%", 1) == 0)
        return 0;
    if(strncmp(&mychar, "?", 1) == 0)
        return 0;
    if(strncmp(&mychar, "!", 1) == 0)
        return 0;
    if(strncmp(&mychar, "#", 1) == 0)
        return 0;
    if(strncmp(&mychar, "~", 1) == 0)
        return 0;
    if(strncmp(&mychar, "=", 1) == 0)
        return 0;
    if(strncmp(&mychar, "(", 1) == 0)
        return 0;
    if(strncmp(&mychar, "[", 1) == 0)
        return 0;
    if(strncmp(&mychar, "{", 1) == 0)
        return 0;
    if(strncmp(&mychar, "^", 1) == 0)
        return 0;

    return 1;
}
*/
