/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * qaul.net network configuration via network manager over dbus.
 */

#include <stdio.h> // printf
#include <net/if.h>  // IFNAMSIZ
#include <dbus-1.0/dbus/dbus.h>

#include "structures.h"


#ifndef _QAUL_NETCONF_NM
#define _QAUL_NETCONF_NM

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


typedef struct qaul_dbus_connection_settings {
	DBusConnection* dbus_connection;
	char	dbus_connection_path[255];
	char	dbus_active_connection_path[255];
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


/**
 * initialize dbus connection
 *
 * @retval 1 success
 * @retval 0 error
 */
int qaul_dbus_init(DBusConnection** dbus_connection);

/**
 * test if network manager is usable
 *
 * @retval 1 network manager is active
 * @retval 0 network manager is not usable
 */
int qaul_dbus_test_networkmanager(DBusConnection* dbus_connection);

/**
 * add connection settings to network manager
 */
int qaul_network_settings_add(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings, qaul_network_settings* network, qaul_dbus_device_properties* device);

/**
 * delete network @a settings via network manager over @a dbus_connection .
 *
 * @retval 1 success
 * @retval 0 error
 *
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 * org.freedesktop.NetworkManager.Settings.Connection
 * Delete ( ) → nothing
 */
int qaul_network_settings_delete(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings);

/**
 * activate network connection
 *
 * The path to the access point can be given as accesspoint_path,
 * if it shall choose automatically use "/"
 *
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 * ActivateConnection ( o: connection, o: device, o: specific_object ) -> o
 */
int qaul_network_connection_activate(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings, qaul_dbus_device_properties* device);

/**
 * deactivate network connection
 *
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 * DeactivateConnection ( o: active_connection ) → nothing
 */
int qaul_network_connection_deactivate(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings);

/**
 * Retrieve dbus path to network @a device by @a interface_name of the IP interface
 *
 * @retval 1 success
 * @retval 0 error
 *
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 * GetDeviceByIpIface("wlan0")
 */
int qaul_network_device_get_by_ipName(const char* interface_name, DBusConnection* dbus_connection, qaul_dbus_device_properties* device);

/**
 * Retrieve @a device properties of the device with @a interface_name
 *
 * @retval 1 device found, properties set
 * @retval 0 device not found, properties not set
 */
int qaul_network_device_get_by_interface(const char* interface_name, DBusConnection* dbus_connection, qaul_dbus_device_properties* device);

/**
 * find the first wifi device
 *
 * @retval 1 wifi device found, properties set
 * @retval 0 no wifi device found, properties not set
 */
int qaul_network_find_wifi(DBusConnection* dbus_connection, qaul_dbus_device_properties* device);

/**
 * Write a Json configuration to @a json_txt of available network devices.
 * Only Wifi and Ethernet devices are represented as the other device cannot be
 * configured at the moment. @a json_txt needs to be a pointer ot a char buffer
 * of the size MAX_JSON_LEN +1.
 *
 * @retval 1 success
 * @retval 2 error
 *
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 * GetDevices() → ao
 *
 * Dbus messages:
 * http://dbus.freedesktop.org/doc/api/html/group__DBusMessage.html
 * DBUS_EXPORT int dbus_message_iter_get_arg_type(DBusMessageIter * iter)
 */
int qaul_network_devices_json(DBusConnection* dbus_connection, char* json_txt);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
