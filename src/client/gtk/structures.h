/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * qaul-client structures and definitions that are shared between modules
 */

#include <net/if.h>  // IFNAMSIZ


#ifndef _QAUL_STRUCTURES
#define _QAUL_STRUCTURES

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


#define MAX_JSON_LEN 1024


/**
 * network configuration methods
 */
typedef enum qaul_config_method {
	NETWORK_MANAGER,
	CLI
} qaul_config_method;


/**
 * network configuration values
 */
typedef struct qaul_network_settings {
	qaul_config_method method;
	char	ipv4_address[16];
	char	ipv4_gateway[16];
	int		ipv4_netmask;
	char	ipv4_broadcast[16];
	char	ipv4_dns1[16];
	char	ipv4_dns2[16];
	int		wifi_channel;
	char	wifi_ssid[32 +1];
	char	wifi_bssid[48 +1];
	char	interface_name[IFNAMSIZ];
} qaul_network_settings;

qaul_network_settings network_settings;


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
