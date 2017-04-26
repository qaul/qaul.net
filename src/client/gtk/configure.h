/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * Configure local machine for qaul.net
 *
 * configure network
 * start routing
 * configure firewall
 * configure Internet sharing
 * configure port forwarding
 */


#include "structures.h"


#ifndef _QAUL_CONFIGURE
#define _QAUL_CONFIGURE

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


/**
 * start configuration
 */
void qaul_configureStart(void);

/**
 * stop configuration
 */
void qaul_configureStop(void);

/**
 * Check which configuration method to use.
 * If the network manager is present, this is the preferred configuration method.
 * Otherwise it will be configured using the tools 'ifconfig' and 'wificonfig'
 */
void qaul_defineConfigurationMethod(void);

/**
 * Find the wifi interface and write the name to @a network_settings
 *
 * @retval 1 wifi interface found
 * @retval 0 no wifi interface found
 */
int qaul_findWifiInterface(qaul_network_settings* network_settings);

/**
 * Check if a network interface with @a interface_name exists
 *
 * @retval 1 network interface exists
 * @retval 0 network interface not found
 */
int qaul_findNetworkInterface(const char* interface_name);

/**
 * Get network interfaces JSON
 *
 * @retval 1 success
 * @retval 0 error
 */
int qaul_getInterfacesJson(char* json_txt);

/**
 * Configure the network interface.
 */
void qaul_networkStart(void);

/**
 * Remove network interface configuration.
 */
void qaul_networkStop(void);

/**
 * start olsrd
 */
void qaul_olsrdStart(void);

/**
 * stop olsrd
 */
void qaul_olsrdStop(void);

/**
 * start port forwarding
 *
 * Forward the standard ports to the qaul services on user ports:
 * 80 => 8081 (TCP, web server for web users and captive portal)
 * 53 => 8053 (UDP, dummy DNS server for captive portal)
 * 67 => 8067 (UDP, DHCP server for captive portal)
 */
void qaul_startPortForwarding(void);

/**
 * stop port forwarding
 */
void qaul_stopPortForwarding(void);

/**
 * start Internet sharing gateway
 */
void qaul_startGateway(void);

/**
 * stop Internet sharing gateway
 */
void qaul_stopGateway(void);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
