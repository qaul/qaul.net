/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * This file contains all validation functions for qaullib.
 */

#include "qaullib/validate.h"


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
        if(validate_char_number(str[i]) == 0 && strncmp(&str[i], ".", 1) != 0)
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
        if(validate_char_number(str[i]) == 0 &&
           validate_char_letter(str[i]) == 0
           )
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}

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
        if(validate_char_number(str[i]) == 0)
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}


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
        if(validate_char_number(str[i]) == 0 &&
           validate_char_letter(str[i]) == 0 &&
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


int validate_char_number (char mychar)
{
    if(strncmp(&mychar, "0", 1)==0)
        return 1;
    if(strncmp(&mychar, "1", 1)==0)
        return 1;
    if(strncmp(&mychar, "2", 1)==0)
        return 1;
    if(strncmp(&mychar, "3", 1)==0)
        return 1;
    if(strncmp(&mychar, "4", 1)==0)
        return 1;
    if(strncmp(&mychar, "5", 1)==0)
        return 1;
    if(strncmp(&mychar, "6", 1)==0)
        return 1;
    if(strncmp(&mychar, "7", 1)==0)
        return 1;
    if(strncmp(&mychar, "8", 1)==0)
        return 1;
    if(strncmp(&mychar, "9", 1)==0)
        return 1;

    return 0;
}


int validate_char_letter (char mychar)
{
    if(strncmp(&mychar, "a", 1)==0 || strncmp(&mychar, "A", 1)==0)
        return 1;
    if(strncmp(&mychar, "b", 1)==0 || strncmp(&mychar, "B", 1)==0)
        return 1;
    if(strncmp(&mychar, "c", 1)==0 || strncmp(&mychar, "C", 1)==0)
        return 1;
    if(strncmp(&mychar, "d", 1)==0 || strncmp(&mychar, "D", 1)==0)
        return 1;
    if(strncmp(&mychar, "e", 1)==0 || strncmp(&mychar, "E", 1)==0)
        return 1;
    if(strncmp(&mychar, "f", 1)==0 || strncmp(&mychar, "F", 1)==0)
        return 1;
    if(strncmp(&mychar, "g", 1)==0 || strncmp(&mychar, "G", 1)==0)
        return 1;
    if(strncmp(&mychar, "h", 1)==0 || strncmp(&mychar, "H", 1)==0)
        return 1;
    if(strncmp(&mychar, "i", 1)==0 || strncmp(&mychar, "I", 1)==0)
        return 1;
    if(strncmp(&mychar, "j", 1)==0 || strncmp(&mychar, "J", 1)==0)
        return 1;
    if(strncmp(&mychar, "k", 1)==0 || strncmp(&mychar, "K", 1)==0)
        return 1;
    if(strncmp(&mychar, "l", 1)==0 || strncmp(&mychar, "L", 1)==0)
        return 1;
    if(strncmp(&mychar, "m", 1)==0 || strncmp(&mychar, "M", 1)==0)
        return 1;
    if(strncmp(&mychar, "n", 1)==0 || strncmp(&mychar, "N", 1)==0)
        return 1;
    if(strncmp(&mychar, "o", 1)==0 || strncmp(&mychar, "O", 1)==0)
        return 1;
    if(strncmp(&mychar, "p", 1)==0 || strncmp(&mychar, "P", 1)==0)
        return 1;
    if(strncmp(&mychar, "q", 1)==0 || strncmp(&mychar, "Q", 1)==0)
        return 1;
    if(strncmp(&mychar, "r", 1)==0 || strncmp(&mychar, "R", 1)==0)
        return 1;
    if(strncmp(&mychar, "s", 1)==0 || strncmp(&mychar, "S", 1)==0)
        return 1;
    if(strncmp(&mychar, "t", 1)==0 || strncmp(&mychar, "T", 1)==0)
        return 1;
    if(strncmp(&mychar, "u", 1)==0 || strncmp(&mychar, "U", 1)==0)
        return 1;
    if(strncmp(&mychar, "v", 1)==0 || strncmp(&mychar, "V", 1)==0)
        return 1;
    if(strncmp(&mychar, "w", 1)==0 || strncmp(&mychar, "W", 1)==0)
        return 1;
    if(strncmp(&mychar, "x", 1)==0 || strncmp(&mychar, "X", 1)==0)
        return 1;
    if(strncmp(&mychar, "y", 1)==0 || strncmp(&mychar, "Y", 1)==0)
        return 1;
    if(strncmp(&mychar, "z", 1)==0 || strncmp(&mychar, "Z", 1)==0)
        return 1;

    return 0;
}


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
