/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QCRY_CONTEXT_
#define _QCRY_CONTEXT_

int qcry_context_init(char *username, char *foobar);

char * qcry_context_getkey(char *username, char *token);

#endif // _QCRY_CONTEXT_

