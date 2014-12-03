
#include "functions.h"

#include <stdio.h> // fprintf sprintf stderr

#define NS_INADDRSZ 4
#define NS_IN6ADDRSZ 16
#define NS_INT16SZ 2


void sleep(unsigned int Sec)
{
  Sleep(Sec * 1000);
}

int inet_aton(const char *AddrStr, struct in_addr *Addr)
{
  Addr->s_addr = inet_addr(AddrStr);

  return 1;
}

char * StrError(unsigned int ErrNo)
{
  static char Msg[1000];

#if !defined WINCE
  FormatMessage(FORMAT_MESSAGE_FROM_SYSTEM, NULL, ErrNo, MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT), Msg, sizeof(Msg), NULL);
#else
  short WideMsg[1000];

  FormatMessage(FORMAT_MESSAGE_FROM_SYSTEM, NULL, ErrNo, MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT), WideMsg, sizeof(WideMsg) / 2,
                NULL);

  if (WideCharToMultiByte(CP_ACP, 0, WideMsg, -1, Msg, sizeof(Msg), NULL, NULL) == 0)
    strscpy(Msg, "[cannot convert string]", sizeof(Msg));
#endif

  return Msg;
}


void WinSockPError(char *Str)
{
  fprintf(stderr, "ERROR - %s: %s", Str, StrError(WSAGetLastError()));
}


static char *
inet_ntop4(const unsigned char *src, char *dst, int size)
{
  static const char fmt[] = "%u.%u.%u.%u";
  char tmp[sizeof "255.255.255.255"];

  if (sprintf(tmp, fmt, src[0], src[1], src[2], src[3]) > size)
    return (NULL);

  return strscpy(dst, tmp, size);
}

static char *
inet_ntop6(const unsigned char *src, char *dst, int size)
{
  char tmp[sizeof "ffff:ffff:ffff:ffff:ffff:ffff:255.255.255.255"], *tp;
  struct {
    int base, len;
  } best, cur;
  u_int words[NS_IN6ADDRSZ / NS_INT16SZ];
  int i;

  memset(words, '\0', sizeof words);

  for (i = 0; i < NS_IN6ADDRSZ; i += 2)
    words[i / 2] = (src[i] << 8) | src[i + 1];

  best.base = -1;
  cur.base = -1;

  for (i = 0; i < (NS_IN6ADDRSZ / NS_INT16SZ); i++) {
    if (words[i] == 0) {
      if (cur.base == -1)
        cur.base = i, cur.len = 1;

      else
        cur.len++;
    }

    else {
      if (cur.base != -1) {
        if (best.base == -1 || cur.len > best.len)
          best = cur;

        cur.base = -1;
      }
    }
  }

  if (cur.base != -1) {
    if (best.base == -1 || cur.len > best.len)
      best = cur;
  }

  if (best.base != -1 && best.len < 2)
    best.base = -1;

  tp = tmp;

  for (i = 0; i < (NS_IN6ADDRSZ / NS_INT16SZ); i++) {
    if (best.base != -1 && i >= best.base && i < (best.base + best.len)) {
      if (i == best.base)
        *tp++ = ':';

      continue;
    }

    if (i != 0)
      *tp++ = ':';

    if (i == 6 && best.base == 0 && (best.len == 6 || (best.len == 5 && words[5] == 0xffff))) {
      if (!inet_ntop4(src + 12, tp, sizeof tmp - (tp - tmp)))
        return (NULL);

      tp += strlen(tp);

      break;
    }

    tp += sprintf(tp, "%x", words[i]);
  }

  if (best.base != -1 && (best.base + best.len) == (NS_IN6ADDRSZ / NS_INT16SZ))
    *tp++ = ':';

  *tp++ = '\0';

  if ((tp - tmp) > size)
    return (NULL);

  return strscpy(dst, tmp, size);
}

char *
inet_ntop(int af, const void *src, char *dst, int size)
{
  switch (af) {
  case AF_INET:
    return (inet_ntop4(src, dst, size));

  case AF_INET6:
    return (inet_ntop6(src, dst, size));

  default:
    return (NULL);
  }
}


char *strscpy(char *d, const char *s, size_t len)
{
     char *d_orig = d;

     if (!len) {
         return d;
     }
     while (--len && (*d++ = *s++));
     *d = '\0';
     return d_orig;
}

// --------------------------------------------------
// inet_pton & inet_ntop windows port of olsrd
// --------------------------------------------------
#define NS_INADDRSZ 4
#define NS_IN6ADDRSZ 16
#define NS_INT16SZ 2

static int
inet_pton4(const char *src, unsigned char *dst)
{
  int saw_digit, octets, ch;
  u_char tmp[NS_INADDRSZ], *tp;

  saw_digit = 0;
  octets = 0;
  *(tp = tmp) = 0;

  while ((ch = *src++) != '\0') {
    if (ch >= '0' && ch <= '9') {
      unsigned int new = *tp * 10 + (ch - '0');

      if (new > 255)
        return (0);

      *tp = new;

      if (!saw_digit) {
        if (++octets > 4)
          return (0);

        saw_digit = 1;
      }
    }

    else if (ch == '.' && saw_digit) {
      if (octets == 4)
        return (0);

      *++tp = 0;

      saw_digit = 0;
    }

    else
      return (0);
  }

  if (octets < 4)
    return (0);

  memcpy(dst, tmp, NS_INADDRSZ);
  return (1);
}

static int
inet_pton6(const char *src, unsigned char *dst)
{
  static const char xdigits[] = "0123456789abcdef";
  u_char tmp[NS_IN6ADDRSZ], *tp, *endp, *colonp;
  const char *curtok;
  int ch, saw_xdigit;
  u_int val;

  tp = memset(tmp, '\0', NS_IN6ADDRSZ);
  endp = tp + NS_IN6ADDRSZ;
  colonp = NULL;

  if (*src == ':')
    if (*++src != ':')
      return (0);

  curtok = src;
  saw_xdigit = 0;
  val = 0;

  while ((ch = tolower(*src++)) != '\0') {
    const char *pch;

    pch = strchr(xdigits, ch);

    if (pch != NULL) {
      val <<= 4;
      val |= (pch - xdigits);

      if (val > 0xffff)
        return (0);

      saw_xdigit = 1;
      continue;
    }

    if (ch == ':') {
      curtok = src;

      if (!saw_xdigit) {
        if (colonp)
          return (0);

        colonp = tp;
        continue;
      }

      else if (*src == '\0') {
        return (0);
      }

      if (tp + NS_INT16SZ > endp)
        return (0);

      *tp++ = (u_char) (val >> 8) & 0xff;
      *tp++ = (u_char) val & 0xff;
      saw_xdigit = 0;
      val = 0;
      continue;
    }

    if (ch == '.' && ((tp + NS_INADDRSZ) <= endp) && inet_pton4(curtok, tp) > 0) {
      tp += NS_INADDRSZ;
      saw_xdigit = 0;
      break;
    }

    return (0);
  }

  if (saw_xdigit) {
    if (tp + NS_INT16SZ > endp)
      return (0);

    *tp++ = (u_char) (val >> 8) & 0xff;
    *tp++ = (u_char) val & 0xff;
  }

  if (colonp != NULL) {
    const int n = tp - colonp;
    int i;

    if (tp == endp)
      return (0);

    for (i = 1; i <= n; i++) {
      endp[-i] = colonp[n - i];
      colonp[n - i] = 0;
    }

    tp = endp;
  }

  if (tp != endp)
    return (0);

  memcpy(dst, tmp, NS_IN6ADDRSZ);
  return (1);
}

int
inet_pton(int af, const char *src, void *dst)
{
  switch (af) {
  case AF_INET:
    return (inet_pton4(src, dst));

  case AF_INET6:
    return (inet_pton6(src, dst));

  default:
    return -1;
  }
}
