/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * qaul.net network configuration via network manager over dbus.
 */

#include<stdio.h> // printf

#include <net/if.h>  // IFNAMSIZ
#include <dbus-1.0/dbus/dbus.h>

#define MAX_JSON_LEN 1024


typedef struct qaul_dbus_connection_settings {
	char	dbus_connection_path[255];
	char	dbus_active_connection_path[255];
	char	ipv4_address[16];
	char	ipv4_gateway[16];
	int		ipv4_netmask;
	char	ipv4_dns1[16];
	char	ipv4_dns2[16];
	int		wifi_channel;
	char	wifi_ssid[IFNAMSIZ +1];
	char	wifi_bssid[48 +1];
} qaul_dbus_connection_settings;

/**
 * Network Manager dbus device properties
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 *
 * @param type       NM_DEVICE_TYPE:
 *                   NM_DEVICE_TYPE_UNKNOWN = 0
 *                   NM_DEVICE_TYPE_ETHERNET = 1
 *                   NM_DEVICE_TYPE_WIFI = 2
 * @param interface  name of network interface (e.g. "wlan0")
 * @param state      NM_DEVICE_STATE:
 *                   NM_DEVICE_STATE_UNMANAGED = 10
 *                   NM_DEVICE_STATE_UNAVAILABLE = 20
 *                   NM_DEVICE_STATE_DISCONNECTED = 30
 *                   NM_DEVICE_STATE_ACTIVATED = 100
 *                   NM_DEVICE_STATE_DEACTIVATING = 110
 *                   NM_DEVICE_STATE_FAILED = 120
 * @param mac        String of device's hardware MAC address
 *                   (e.g. "00:25:17:64:15:a0")
 * @param dbus_device_path
 *                   (e.g. "/org/freedesktop/NetworkManager/Devices/0")
 */
typedef struct qaul_dbus_device_properties {
	int 	type;
	char	interface[IFNAMSIZ +1];
	int		state;
	char	mac[18];
	char	dbus_device_path[255];
} qaul_dbus_device_properties;


int qaul_dbus_init(DBusConnection** dbus_connection);

int qaul_network_settings_add(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings, qaul_dbus_device_properties* device);
int qaul_network_settings_delete(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings);
int qaul_network_connection_activate(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings, qaul_dbus_device_properties* device);
int qaul_network_connection_deactivate(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings);
int qaul_network_device_get_by_ipName(const char* interface_name, DBusConnection* dbus_connection, qaul_dbus_device_properties* device);
int qaul_network_device_get_by_interface(const char* interface_name, DBusConnection* dbus_connection, qaul_dbus_device_properties* device);
int qaul_network_find_wifi(DBusConnection* dbus_connection, qaul_dbus_device_properties* device);
int qaul_network_devices_json(DBusConnection* dbus_connection, char* json_txt);


