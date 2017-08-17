/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_WHITELIST
#define _QAULLIB_WHITELIST

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


#include "../qaullib_defines.h"

/**
 * The whitelist contains all IP's of white listed web users for
 * which the web server simulates to be connected to the Internet.
 * This is important for iOS and OSX users to be able to use all functions
 * of their browser and their system, as network access is disabled
 * in offline local networks.
 * As another effect, the system will no longer show a login popup.
 *
 * List items are deleted after a timeout.
 */


/**
 * Add this @a ip to the white list
 */
void ql_whitelist_add (union olsr_ip_addr *ip);


/**
 * Check if this @a ip is white listed
 *
 * @retval 1  ip found: this @a ip is whitelisted
 * @retval 0  ip not found: this @ ip is not whitelisted
 */
int ql_whitelist_check (union olsr_ip_addr *ip);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
