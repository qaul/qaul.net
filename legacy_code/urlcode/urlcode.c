/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <stdlib.h>
#include <string.h>
#include <ctype.h>


char Qaullib_UrlFromHex(char chr)
{
  return isdigit(chr) ? chr - '0' : tolower(chr) - 'a' + 10;
}

char Qaullib_UrlToHex(char code)
{
  static char hex[] = "0123456789abcdef";
  return hex[code & 15];
}

char *Qaullib_UrlEncode(char *str)
{
  char *pstr = str, *buf = (char *)malloc(strlen(str) * 3 + 1), *pbuf = buf;
  while (*pstr)
  {
    if (isalnum(*pstr) || *pstr == '-' || *pstr == '_' || *pstr == '.' || *pstr == '~') 
      *pbuf++ = *pstr;
    else if (*pstr == ' ') 
      *pbuf++ = '+';
    else 
      *pbuf++ = '%', *pbuf++ = Qaullib_UrlToHex(*pstr >> 4), *pbuf++ = Qaullib_UrlToHex(*pstr & 15);
    pstr++;
  }
  *pbuf = '\0';
  return buf;
}

char *Qaullib_UrlDecode(char *str)
{
  char *pstr = str, *buf = (char *)malloc(strlen(str) + 1), *pbuf = buf;
  while (*pstr) {
    if (*pstr == '%')
    {
      if (pstr[1] && pstr[2])
      {
        *pbuf++ = Qaullib_UrlFromHex(pstr[1]) << 4 | Qaullib_UrlFromHex(pstr[2]);
        pstr += 2;
      }
    }
    else if (*pstr == '+')
    {
      *pbuf++ = ' ';
    }
    else
    {
      *pbuf++ = *pstr;
    }
    pstr++;
  }
  *pbuf = '\0';
  return buf;
}
