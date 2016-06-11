/**
 * This is free software.
 *
 * This is part of the libqaul debugging suite to give easy access to the qcry_* namespace
 */

#include "qaullib/qcry_wrapper.h"
#include <stdio.h>

int main(int argn, char **argv)
{
    printf("============= QAUL.NET DEBUGGER PROFESSIONAL 2016 =============\n");
    /** Init our crypto shizzle */
    qcry_devel_init();

    return 0;
}