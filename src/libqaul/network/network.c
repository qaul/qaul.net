/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <qaul/mod/network.h>
#include <qaul/utils/strings.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

char *ql_net_ip_to_string(union ql_ip ip)
{
    char buffer[16];
    memset(buffer, 0, sizeof(char) * 16);

    for(int i = 0; i < 4; i++) {
        char str[4];
        memset(str, 0, sizeof(char) * 4);
        sprintf(str, "%o", ip.v4[i]);
        strcat(buffer, str);
    }

    return buffer;
}


ql_ip ql_net_string_to_ip(const char *str)
{
    char **vals = str_split(str, '.');
    ql_ip ip = {
        .v4 = {
            ((short) atoi(vals[0])),
            ((short) atoi(vals[1])),
            ((short) atoi(vals[2])),
            ((short) atoi(vals[3]))
        }
    };
    return ip;
}