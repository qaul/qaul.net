/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"


int Qaullib_ValidateCharASCIILetterOrNumber(char *character)
{
	if(Qaullib_ValidateCharNumber(character) == 1 || Qaullib_ValidateCharASCIILetter(character) == 1)
		return 1;

	return 0;
}


int Qaullib_ValidateCharASCIILetter(char *character)
{
	if(Qaullib_ValidateCharLowercaseASCII(character) == 1 || Qaullib_ValidateCharUppercaseASCII(character) == 1)
		return 1;

	return 0;
}


int Qaullib_ValidateCharLowercaseASCII(char *character)
{
	if(memcmp(character, "a", 1) == 0)
		return 1;
	if(memcmp(character, "b", 1) == 0)
		return 1;
	if(memcmp(character, "c", 1) == 0)
		return 1;
	if(memcmp(character, "d", 1) == 0)
		return 1;
	if(memcmp(character, "e", 1) == 0)
		return 1;
	if(memcmp(character, "f", 1) == 0)
		return 1;
	if(memcmp(character, "g", 1) == 0)
		return 1;
	if(memcmp(character, "h", 1) == 0)
		return 1;
	if(memcmp(character, "i", 1) == 0)
		return 1;
	if(memcmp(character, "j", 1) == 0)
		return 1;
	if(memcmp(character, "k", 1) == 0)
		return 1;
	if(memcmp(character, "l", 1) == 0)
		return 1;
	if(memcmp(character, "m", 1) == 0)
		return 1;
	if(memcmp(character, "n", 1) == 0)
		return 1;
	if(memcmp(character, "o", 1) == 0)
		return 1;
	if(memcmp(character, "p", 1) == 0)
		return 1;
	if(memcmp(character, "q", 1) == 0)
		return 1;
	if(memcmp(character, "r", 1) == 0)
		return 1;
	if(memcmp(character, "s", 1) == 0)
		return 1;
	if(memcmp(character, "t", 1) == 0)
		return 1;
	if(memcmp(character, "u", 1) == 0)
		return 1;
	if(memcmp(character, "v", 1) == 0)
		return 1;
	if(memcmp(character, "w", 1) == 0)
		return 1;
	if(memcmp(character, "x", 1) == 0)
		return 1;
	if(memcmp(character, "y", 1) == 0)
		return 1;
	if(memcmp(character, "z", 1) == 0)
		return 1;

	return 0;
}


int Qaullib_ValidateCharUppercaseASCII(char *character)
{
	if(memcmp(character, "A", 1) == 0)
		return 1;
	if(memcmp(character, "B", 1) == 0)
		return 1;
	if(memcmp(character, "C", 1) == 0)
		return 1;
	if(memcmp(character, "D", 1) == 0)
		return 1;
	if(memcmp(character, "E", 1) == 0)
		return 1;
	if(memcmp(character, "F", 1) == 0)
		return 1;
	if(memcmp(character, "G", 1) == 0)
		return 1;
	if(memcmp(character, "H", 1) == 0)
		return 1;
	if(memcmp(character, "I", 1) == 0)
		return 1;
	if(memcmp(character, "J", 1) == 0)
		return 1;
	if(memcmp(character, "K", 1) == 0)
		return 1;
	if(memcmp(character, "L", 1) == 0)
		return 1;
	if(memcmp(character, "M", 1) == 0)
		return 1;
	if(memcmp(character, "N", 1) == 0)
		return 1;
	if(memcmp(character, "O", 1) == 0)
		return 1;
	if(memcmp(character, "P", 1) == 0)
		return 1;
	if(memcmp(character, "Q", 1) == 0)
		return 1;
	if(memcmp(character, "R", 1) == 0)
		return 1;
	if(memcmp(character, "S", 1) == 0)
		return 1;
	if(memcmp(character, "T", 1) == 0)
		return 1;
	if(memcmp(character, "U", 1) == 0)
		return 1;
	if(memcmp(character, "V", 1) == 0)
		return 1;
	if(memcmp(character, "W", 1) == 0)
		return 1;
	if(memcmp(character, "X", 1) == 0)
		return 1;
	if(memcmp(character, "Y", 1) == 0)
		return 1;
	if(memcmp(character, "Z", 1) == 0)
		return 1;

	return 0;
}


int Qaullib_ValidateCharNumber(char *character)
{
	if(memcmp(character, "0", 1) == 0)
		return 1;
	if(memcmp(character, "1", 1) == 0)
		return 1;
	if(memcmp(character, "2", 1) == 0)
		return 1;
	if(memcmp(character, "3", 1) == 0)
		return 1;
	if(memcmp(character, "4", 1) == 0)
		return 1;
	if(memcmp(character, "5", 1) == 0)
		return 1;
	if(memcmp(character, "6", 1) == 0)
		return 1;
	if(memcmp(character, "7", 1) == 0)
		return 1;
	if(memcmp(character, "8", 1) == 0)
		return 1;
	if(memcmp(character, "9", 1) == 0)
		return 1;

	return 0;
}
