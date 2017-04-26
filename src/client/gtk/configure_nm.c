/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <string.h>

#include "configure.h"
#include "configure_nm.h"
#include "networkmanager_configuration.h"

// dbus / network manager specific variables
DBusConnection*	network_dbus_connection;
qaul_dbus_connection_settings network_dbus_settings;
qaul_dbus_device_properties network_device;


int qaul_initNetworkManager(void)
{
	printf("qaul_initNetworkManager\n");

	// initialize dbus connection
	if(qaul_dbus_init(&network_dbus_connection) == 0)
		return 0;

	// check if network manager is present
	if(qaul_dbus_test_networkmanager(network_dbus_connection) == 0)
		return 0;

	return 1;
}

int qaul_findWifiInterface_nm(qaul_network_settings* network)
{
	int success;

	success = qaul_network_find_wifi(network_dbus_connection, &network_device);
	if(success)
	{
		strncpy(network->interface_name, network_device.interface, sizeof(network->interface_name));
		network->interface_name[sizeof(network->interface_name)-1] = '\0';
	}

	return success;
}

int qaul_findNetworkInterface_nm(const char* interface_name)
{
	return qaul_network_device_get_by_interface(interface_name, network_dbus_connection, &network_device);
}

int qaul_getInterfacesJson_nm(char* json_txt)
{
	return qaul_network_devices_json(network_dbus_connection, json_txt);
}

void qaul_networkStart_nm(void)
{
	// add network configuration
	if(qaul_network_settings_add(network_dbus_connection, &network_dbus_settings, &network_settings, &network_device))
	{
		printf("[configure] Network connection setting added: %s\n", network_dbus_settings.dbus_connection_path);

		// activate configuration
		if(qaul_network_connection_activate(network_dbus_connection, &network_dbus_settings, &network_device))
			printf("[configure] Network connection activated: %s\n", network_dbus_settings.dbus_active_connection_path);
		else
			printf("[configure] Network connection not activated\n");
	}
	else
		printf("[configure] Network connection settings not added\n");
}

void qaul_networkStop_nm(void)
{
	// deactivate connection
	if(qaul_network_connection_deactivate(network_dbus_connection, &network_dbus_settings))
		printf("[quit] connection deactivated\n");
	else
		printf("[quit] connection not deactivated\n");

	// delete connection settings
	if(qaul_network_settings_delete(network_dbus_connection, &network_dbus_settings))
		printf("[quit] connection settings deleted\n");
	else
		printf("[quit] connection settings not deleted\n");
}

