#include <qaullib/qcry_wrapper.h>
#include "crypto/qcry_context.h"

#include <stdio.h>

int qcry_devel_init()
{
    printf("Wrapper INIT\n");

    char *username;
    char *foobar;
    qcry_context_init(username, foobar);
    return 0;
}