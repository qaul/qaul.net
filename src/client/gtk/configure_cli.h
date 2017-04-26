/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * Configure local machine for qaul.net using linux CLI tools.
 */


#include "structures.h"


#ifndef _QAUL_CONFIGURE_CLI
#define _QAUL_CONFIGURE_CLI

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


/**
 * Find the wifi interface and write the name to @a network_settings
 *
 * @retval 1 wifi interface found
 * @retval 0 no wifi interface found
 */
int qaul_findWifiInterface_cli(qaul_network_settings* network);

/**
 * Check if a network interface with @a interface_name exists
 *
 * @retval 1 network interface exists
 * @retval 0 network interface not found
 */
int qaul_findNetworkInterface_cli(const char* interface_name);

/**
 * Get network interfaces JSON
 *
 * @retval 1 success
 * @retval 0 error
 */
int qaul_getInterfacesJson_cli(char* json_txt);

/**
 * Configure the network interface.
 */
void qaul_networkStart_cli(void);

/**
 * Remove network interface configuration.
 */
void qaul_networkStop_cli(void);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
