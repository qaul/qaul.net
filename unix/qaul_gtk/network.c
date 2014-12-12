/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * Configure your network via dbus with network manager.
 *
 * The following libraries are required to compile it:
 * libnm-gtk-dev
 *
 * Features:
 * add qaul.net profile
 * check if interface exists
 * find wifi interface
 * find all network interfaces
 * activate profile
 *
 * TODO:
 * check if profile exists
 * react if network manager wants to change network
 */

#include "network.h"
#include <string.h>  // strncpy()


/**
 * create session @a uuid. uuid must be a 37 char array.
 *
 * example uuid's:
 * 6cfb68c2-cf07-4a74-8a35-0b2c27bbadc9
 * 85404bed-573d-4156-b78b-ba718cfcf852
 * 8bccb671-8cd3-4cf1-aade-2146ed8dd406
 * ce89d6db-b5ab-4c57-9233-d3391491baee
 */
static int qaul_dbus_create_uuid(char* uuid)
{
	int i, num;
	char values[] = "0123456789abcdef";

	// create random sequence
	srand(time(NULL));
	for(i=0; i<36; ++i)
	{
		num = rand() % 16;
		strncpy(uuid +i, &values[num], 1);
	}

	// set predefined positions
	strncpy(uuid + 8, "-", 1);
	strncpy(uuid +13, "-", 1);
	strncpy(uuid +14, "4", 1);
	strncpy(uuid +18, "-", 1);
	strncpy(uuid +19, "9", 1); // uuid +19 is either: 8,9,a,b
	strncpy(uuid +23, "-", 1);
	strncpy(uuid +36, "\0", 1);
}

/**
 * append variant value
 */
static dbus_bool_t qaul_dbus_append_variant(DBusMessageIter* iter, int type, void* value)
{
	DBusMessageIter iter_value;
	char sig[2] = {type, '\0'};
	return(
		   dbus_message_iter_open_container(iter, DBUS_TYPE_VARIANT, sig, &iter_value)
		&& dbus_message_iter_append_basic (&iter_value, type, value)
		&& dbus_message_iter_close_container (iter, &iter_value)
	);
}

/**
 * append dict entry
 */
static dbus_bool_t qaul_dbus_append_dict_entry(DBusMessageIter* iter_dict, const char* key, int type, void* value)
{
	DBusMessageIter iter_entry;

	if (type == DBUS_TYPE_STRING)
	{
		const char *str = *((const char **) value);
		if (str == NULL)
			return TRUE;
	}

	return (
		   dbus_message_iter_open_container(iter_dict, DBUS_TYPE_DICT_ENTRY, NULL, &iter_entry)
		&& dbus_message_iter_append_basic(&iter_entry, DBUS_TYPE_STRING, &key)
		&& qaul_dbus_append_variant(&iter_entry, type, value)
		&& dbus_message_iter_close_container(iter_dict, &iter_entry)
	);
}

/**
 * append ssid
 */
static dbus_bool_t qaul_dbus_append_ssid(DBusMessageIter* iter, char* ssid)
{
	int i;
	unsigned char ssid_unsigned[33];

	strncpy(ssid_unsigned, ssid, sizeof(ssid_unsigned));

	for(i=0; i<strlen(ssid_unsigned); i++)
	{
		if(!dbus_message_iter_append_basic(iter, DBUS_TYPE_BYTE, &ssid_unsigned[i]))
		{
			printf("qaul_network_add_settings append SSID %i \"%s\" error\n", i, ssid_unsigned);
			return FALSE;
		}
	}

	return TRUE;
}

/**
 * append Hardware MAC address
 */
static dbus_bool_t qaul_dbus_append_device_mac(DBusMessageIter* iter, qaul_dbus_device_properties* device)
{
	DBusMessageIter iter_dict, iter_variant, iter_array;
	char* property_key;
	unsigned char mac[7];
	char buf[10];
	int i;

	property_key = "mac-address";

	// check if MAC address is set
	if(strlen(device->mac) == 17)
	{
		// convert mac address to string
		for(i=0; i<=6; i++)
		{
			sprintf(buf, "0x%c%c", device->mac[i*3], device->mac[(i*3)+1]);
			mac[i] = strtol(buf, NULL, 0);
		}

		if(!(
			   dbus_message_iter_open_container(iter, DBUS_TYPE_DICT_ENTRY, NULL, &iter_dict)
			&& dbus_message_iter_append_basic(&iter_dict, DBUS_TYPE_STRING, &property_key)
			&& dbus_message_iter_open_container(&iter_dict, DBUS_TYPE_VARIANT, "ay", &iter_variant)
			&& dbus_message_iter_open_container(&iter_variant, DBUS_TYPE_ARRAY, DBUS_TYPE_BYTE_AS_STRING, &iter_array)
			&& dbus_message_iter_append_basic(&iter_array, DBUS_TYPE_BYTE, &mac[0])
			&& dbus_message_iter_append_basic(&iter_array, DBUS_TYPE_BYTE, &mac[1])
			&& dbus_message_iter_append_basic(&iter_array, DBUS_TYPE_BYTE, &mac[2])
			&& dbus_message_iter_append_basic(&iter_array, DBUS_TYPE_BYTE, &mac[3])
			&& dbus_message_iter_append_basic(&iter_array, DBUS_TYPE_BYTE, &mac[4])
			&& dbus_message_iter_append_basic(&iter_array, DBUS_TYPE_BYTE, &mac[5])
			&& dbus_message_iter_close_container(&iter_variant, &iter_array)
			&& dbus_message_iter_close_container(&iter_dict, &iter_variant)
			&& dbus_message_iter_close_container(iter, &iter_dict)
			))
		{
			printf("qaul_dbus_append_device_mac append Hardware MAC address error\n");
			return FALSE;
		}
	}
	else
		printf("qaul_dbus_append_device_mac no Hardware MAC address found\n");

	return TRUE;
}

/**
 * initialize dbus connection
 */
int qaul_dbus_init(DBusConnection** dbus_connection)
{
	DBusError error;

	dbus_error_init(&error);
	*dbus_connection = dbus_bus_get(DBUS_BUS_SYSTEM, &error);

	if (dbus_error_is_set(&error))
	{
		printf("qaul_dbus_init error: %s\n", error.message);
		dbus_error_free(&error);
		return 0;
	}

	return 1;
}

/**
 * initialize dbus @a method call
 *
 * network manager dbus methods are documented here:
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 */
static DBusMessage* networkmanager_dbus_method_call(const char* method)
{
	return dbus_message_new_method_call(
									  "org.freedesktop.NetworkManager",
									  "/org/freedesktop/NetworkManager",
									  "org.freedesktop.NetworkManager",
									  method
									  );
}

/**
 * send data for dbus method call
 */
static int networkmanager_dbus_send(DBusMessage** msg, DBusConnection* dbus_connection)
{
	DBusPendingCall* pending;
	int success;

	// send dbus message
	// -1 is the default time out. Other time outs can be configured in milli seconds.
	success = dbus_connection_send_with_reply(dbus_connection, *msg, &pending, -1);
	if(!success)
	{
		printf("networkmanager_dbus_send dbus send error\n");
		return 0;
	}
	if(pending == NULL)
	{
		printf ("networkmanager_dbus_send dbus calling error\n");
		return 0;
	}

	dbus_connection_flush(dbus_connection);
	dbus_message_unref(*msg);
	dbus_pending_call_block(pending);
	*msg = dbus_pending_call_steal_reply(pending);
	dbus_pending_call_unref(pending);

	if(*msg == NULL)
		return 0;

	return 1;
}

/**
 * structure to retrieve a dbus property.
 */
typedef struct networkmanager_property {
	char* 	dbus_path;
	char*	dbus_interface;
	char*	dbus_property_name;
	int		value_int;
	char*	value_string;
	int 	value_string_len;
} networkmanager_property;

/**
 * get @a property via @a dbus_connection .
 * org.freedesktop.DBus.Properties
 * Get (String interface, String propname) -> (Variant value)
 */
static int networkmanager_get_property(DBusConnection* dbus_connection, networkmanager_property* property)
{
	DBusMessage* msg;
    DBusMessageIter iter, iter_variant;
	DBusPendingCall* pending;
	int success;
    char* value_str_ptr;

	msg = dbus_message_new_method_call(
									  "org.freedesktop.NetworkManager",
									  property->dbus_path,
									  "org.freedesktop.DBus.Properties",
									  "Get"
									  );

	if(msg == NULL)
	{
		printf("networkmanager_get_property msg error\n");
		return 0;
	}

    dbus_message_iter_init_append(msg, &iter);
    dbus_message_iter_append_basic(&iter, DBUS_TYPE_STRING, &property->dbus_interface);
	dbus_message_iter_append_basic(&iter, DBUS_TYPE_STRING, &property->dbus_property_name);

	// send dbus message
	// -1 is the default time out. Other time outs can be configured in milli seconds.
	success = dbus_connection_send_with_reply(dbus_connection, msg, &pending, -1);
	if(!success)
	{
		printf("networkmanager_get_property dbus send error\n");
		return 0;
	}
	if(pending == NULL)
	{
		printf ("networkmanager_get_property dbus calling error\n");
		return 0;
	}

	dbus_connection_flush(dbus_connection);
	dbus_message_unref(msg);
	dbus_pending_call_block(pending);
	msg = dbus_pending_call_steal_reply(pending);
	dbus_pending_call_unref(pending);

	if(msg == NULL)
	{
		printf("networkmanager_get_property msg error 2\n");
		return 0;
	}

	if(!dbus_message_iter_init(msg, &iter))
	{
		printf("networkmanager_get_property dbus_message_iter_init error\n");
		dbus_message_unref(msg);
		return 0;
	}

	if(dbus_message_iter_get_arg_type(&iter) == DBUS_TYPE_VARIANT)
	{
		dbus_message_iter_recurse(&iter, &iter_variant);
		if(dbus_message_iter_get_arg_type(&iter_variant) == DBUS_TYPE_STRING)
		{
			dbus_message_iter_get_basic(&iter_variant, &value_str_ptr);
			strncpy(property->value_string, value_str_ptr, property->value_string_len);
			printf("networkmanager_get_property %s %s: %s\n", property->dbus_path, property->dbus_property_name, value_str_ptr);
		}
		else if(dbus_message_iter_get_arg_type(&iter_variant) == DBUS_TYPE_UINT32)
		{
			dbus_message_iter_get_basic(&iter_variant, &property->value_int);
			printf("networkmanager_get_property %s %s: %i\n", property->dbus_path, property->dbus_property_name, property->value_int);
		}
		else
		{
			printf("networkmanager_get_property dbus_message_iter_get_arg_type error\n");
			dbus_message_unref(msg);
			return 0;
		}
	}
	else
	{
		printf("networkmanager_get_property dbus_message_iter_get_arg_type not variant error\n");
		dbus_message_unref(msg);
		return 0;
	}

	dbus_message_unref(msg);

	return 1;
}

/**
 * get all properties of @a dbus_device_path as @a value struct via dbus @a connection
 *
 * network manager dbus methods are documented here:
 * https://developer.gnome.org/NetworkManager/unstable/spec.html#org.freedesktop.NetworkManager.Device
 */
static int networkmanager_device_properties(DBusConnection* dbus_connection, qaul_dbus_device_properties* device)
{
	DBusMessage* msg;
    DBusMessageIter iter, iter_array, iter_dict, iter_prop;
	DBusPendingCall* pending;
	int success, set_Type, set_Interface, set_State, current_type;
    char* property_ptr;
    char* key_ptr;
	networkmanager_property device_property;

    const char* dbus_interface = "org.freedesktop.NetworkManager.Device";

    // reset values
    set_Type = 0;
    set_Interface = 0;
    set_State = 0;
    device->type = 0;
    device->state = 0;
    strncpy(device->interface, "", sizeof(device->interface));

	msg = dbus_message_new_method_call(
									  "org.freedesktop.NetworkManager",
									  device->dbus_device_path,
									  "org.freedesktop.DBus.Properties",
									  "GetAll"
									  );

	if(msg == NULL)
	{
		printf("networkmanager_device_properties msg error\n");
		return 0;
	}

    dbus_message_iter_init_append(msg, &iter);
    dbus_message_iter_append_basic(&iter, DBUS_TYPE_STRING, &dbus_interface);

	// send dbus message
	// -1 is the default time out. Other time outs can be configured in milli seconds.
	success = dbus_connection_send_with_reply(dbus_connection, msg, &pending, -1);
	if(!success)
	{
		printf("networkmanager_device_properties dbus send error\n");
		return 0;
	}
	if(pending == NULL)
	{
		printf ("networkmanager_device_properties dbus calling error\n");
		return 0;
	}

	dbus_connection_flush(dbus_connection);
	dbus_message_unref(msg);
	dbus_pending_call_block(pending);
	msg = dbus_pending_call_steal_reply(pending);
	dbus_pending_call_unref(pending);

	if(msg == NULL)
	{
		printf("networkmanager_device_properties msg error 2\n");
		return 0;
	}

	if(!dbus_message_iter_init(msg, &iter))
	{
		printf("networkmanager_device_properties dbus_message_iter_init error\n");
		dbus_message_unref(msg);
		return 0;
	}
	if(dbus_message_iter_get_arg_type(&iter) != DBUS_TYPE_ARRAY)
	{
		printf("networkmanager_device_properties dbus_message_iter_get_arg_type return is not a DBUS_TYPE_ARRAY %i\n", dbus_message_iter_get_arg_type(&iter));
		if(dbus_message_iter_get_arg_type(&iter) == DBUS_TYPE_STRING)
		{
			dbus_message_iter_get_basic(&iter, &property_ptr);
			printf("networkmanager_device_properties error msg: %s\n", property_ptr);
		}
		dbus_message_unref(msg);
		return 0;
	}

	// loop over the array
	dbus_message_iter_recurse(&iter, &iter_array);
	while((current_type = dbus_message_iter_get_arg_type(&iter_array)) != DBUS_TYPE_INVALID)
	{
		// check if it is a dict entry
		if(dbus_message_iter_get_arg_type(&iter_array) == DBUS_TYPE_DICT_ENTRY)
		{
			dbus_message_iter_recurse(&iter_array, &iter_dict);
			// get key
			dbus_message_iter_get_basic(&iter_dict, &key_ptr);
			// get property
			if(dbus_message_iter_next(&iter_dict))
			{
				dbus_message_iter_recurse(&iter_dict, &iter_prop);
				if (strcmp(key_ptr, "Interface") == 0)
				{
					if(dbus_message_iter_get_arg_type(&iter_prop) == DBUS_TYPE_STRING)
					{
						dbus_message_iter_get_basic(&iter_prop, &property_ptr);
						strncpy(device->interface, property_ptr, sizeof(device->interface));
						printf("networkmanager_device_get_property interface %s\n", property_ptr);
						set_Interface = 1;
					}
					else
						printf("networkmanager_device_properties Interface not DBUS_TYPE_STRING %i\n", dbus_message_iter_get_arg_type(&iter_prop));
				}
				else if(strcmp(key_ptr, "DeviceType") == 0)
				{
					if(dbus_message_iter_get_arg_type(&iter_prop) == DBUS_TYPE_UINT32)
					{
						dbus_message_iter_get_basic(&iter_prop, &device->type);
						printf("networkmanager_device_get_property DeviceType %i\n", device->type);
						set_Type = 1;
					}
					else
						printf("networkmanager_device_properties DeviceType not DBUS_TYPE_UINT32 %i\n", dbus_message_iter_get_arg_type(&iter_prop));
				}
				else if(strcmp(key_ptr, "State") == 0)
				{
					if(dbus_message_iter_get_arg_type(&iter_prop) == DBUS_TYPE_UINT32)
					{
						dbus_message_iter_get_basic(&iter_prop, &device->state);
						printf("networkmanager_device_get_property State %i\n", device->state);
						set_State = 1;
					}
					else
						printf("networkmanager_device_properties state not DBUS_TYPE_UINT32 %i\n", dbus_message_iter_get_arg_type(&iter_prop));
				}
			}
		}
		dbus_message_iter_next(&iter_array);
	}
	dbus_message_unref(msg);

	// store dbus path
	printf("networkmanager_device_properties device_property.dbus_path\n");
	device_property.dbus_path = device->dbus_device_path;

	printf("networkmanager_device_properties device_property.dbus_interface\n");
	if(device->type == 2)
		device_property.dbus_interface = "org.freedesktop.NetworkManager.Device.Wireless";
	else if(device->type == 1)
		device_property.dbus_interface = "org.freedesktop.NetworkManager.Device.Wired";

	// retrieve Hardware address property only for known device types
	if(device->type == 1 || device->type == 2)
	{
		printf("networkmanager_device_properties device->mac\n");
		device_property.dbus_property_name = "PermHwAddress";
		device_property.value_string = device->mac;
		device_property.value_string_len = sizeof(device->mac);
	}

	if(strlen(device_property.dbus_interface)>0)
	{
		if(!networkmanager_get_property(dbus_connection, &device_property))
			printf("qaul_network_device_get_by_interface get %s property error\n", device_property.dbus_property_name);
	}
	else
		printf("qaul_network_device_get_by_interface no supported type for Hardware MAC address\n");

	if(!set_Type)
		printf("networkmanager_device_properties Device type not set.\n");
	if(!set_Interface)
		printf("networkmanager_device_properties Interface name not set\n");
	if(!set_State)
		printf("networkmanager_device_properties Device state not set\n");

	return 1;
}

/**
 * Retrieve @a device properties of the device with @a interface_name
 *
 * @retval 1 device found, properties set
 * @retval 0 device not found, properties not set
 */
int qaul_network_device_get_by_interface(const char* interface_name, DBusConnection* dbus_connection, qaul_dbus_device_properties* device)
{
	DBusMessage* msg;
	DBusMessageIter iter, iter_array;
	int current_type;
	const char* device_path_ptr;

	printf("qaul_network_device_get_by_interface\n");

	msg = networkmanager_dbus_method_call("GetDevices");
	if(msg == NULL)
		return 0;

	dbus_message_iter_init_append(msg, &iter);
	if(!networkmanager_dbus_send(&msg, dbus_connection))
	{
		printf("qaul_network_device_get_by_interface networkmanager_dbus_send error\n");
		return 0;
	}
	if(!dbus_message_iter_init(msg, &iter)
		|| dbus_message_iter_get_arg_type(&iter) != DBUS_TYPE_ARRAY)
	{
		printf("qaul_network_device_get_by_interface dbus_message_iter_init | DBUS_TYPE_ARRAY error\n");
		dbus_message_unref(msg);
		return 0;
	}

	// loop recursively over the array
	dbus_message_iter_recurse (&iter, &iter_array);
	while((current_type = dbus_message_iter_get_arg_type(&iter_array)) != DBUS_TYPE_INVALID)
	{
		if(current_type == DBUS_TYPE_OBJECT_PATH)
		{
			dbus_message_iter_get_basic(&iter_array, &device_path_ptr);
			strncpy(device->dbus_device_path, device_path_ptr, sizeof(device->dbus_device_path));

			// get interface name, type
			// dbus low level: dbus_message_new_method_call interface:"org.freedesktop.DBus.Properties" method:"Get" property:"(String)Interface, (UInt32)DeviceType"
			if(networkmanager_device_properties(dbus_connection, device))
			{
				if(strcmp(device->interface, interface_name) == 0)
				{
					printf("qaul_network_device_get_by_interface found: %s\n", device->interface);
					dbus_message_unref(msg);
					return 1;
				}
			}
			else
				printf("qaul_network_device_get_by_interface networkmanager_device_get_property failed for %s\n", device->dbus_device_path);
		}
		else
			printf("qaul_network_device_get_by_interface Unknown current type [%i]\n", current_type);

		dbus_message_iter_next(&iter_array);
	}
	dbus_message_unref(msg);

	printf("qaul_network_device_get_by_interface no wifi device found\n");

	return 0;
}

/**
 * Retrieve dbus path to network @a device by @a interface_name of the IP interface
 *
 * @retval 1 success
 * @retval 0 error
 *
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 * GetDeviceByIpIface("wlan0")
 */
int qaul_network_device_get_by_ipName(const char* interface_name, DBusConnection* dbus_connection, qaul_dbus_device_properties* device)
{
	DBusMessage* msg;
	DBusMessageIter iter;
	const char* device_path;

	msg = networkmanager_dbus_method_call("GetDeviceByIpIface");
	if(msg == NULL)
		return 0;

	dbus_message_iter_init_append(msg, &iter);
	dbus_message_iter_append_basic(&iter, DBUS_TYPE_STRING, interface_name);

	if(!networkmanager_dbus_send(&msg, dbus_connection))
		return 0;

	if(!dbus_message_iter_init(msg, &iter) || dbus_message_iter_get_arg_type(&iter) != DBUS_TYPE_OBJECT_PATH)
	{
		dbus_message_unref(msg);
		return 0;
	}
	dbus_message_iter_get_basic(&iter, &device_path);

	strncpy(device->dbus_device_path, device_path, sizeof(device->dbus_device_path));
	dbus_message_unref(msg);

	return 1;
}

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
int qaul_network_devices_json(DBusConnection* dbus_connection, char* json_txt)
{
	DBusMessage* msg;
	DBusMessageIter iter, iter_array;
	int current_type, json_pos, i;
	const char* device_path_ptr;
	qaul_dbus_device_properties device;

	printf("qaul_network_devices_json\n");

	i = 0;
	msg = networkmanager_dbus_method_call("GetDevices");
	if(msg == NULL)
		return 0;

	dbus_message_iter_init_append(msg, &iter);
	if(!networkmanager_dbus_send(&msg, dbus_connection))
	{
		printf("qaul_network_devices_json networkmanager_dbus_send error\n");
		return 0;
	}
	if(!dbus_message_iter_init(msg, &iter)
		|| dbus_message_iter_get_arg_type(&iter) != DBUS_TYPE_ARRAY)
	{
		printf("qaul_network_devices_json dbus_message_iter_init | DBUS_TYPE_ARRAY error\n");
		dbus_message_unref(msg);
		return 0;
	}

	// loop recursively over the array
	json_pos = 0;
	strncpy(json_txt +json_pos, "", MAX_JSON_LEN -json_pos);
	dbus_message_iter_recurse (&iter, &iter_array);
	while((current_type = dbus_message_iter_get_arg_type(&iter_array)) != DBUS_TYPE_INVALID)
	{
		if(current_type == DBUS_TYPE_OBJECT_PATH)
		{
			dbus_message_iter_get_basic(&iter_array, &device_path_ptr);
			strncpy(device.dbus_device_path, device_path_ptr, sizeof(device.dbus_device_path));

			// get interface name and interface type
			// dbus low level: dbus_message_new_method_call interface:"org.freedesktop.DBus.Properties" method:"Get" property:"(String)Interface, (UInt32)DeviceType"
			if(networkmanager_device_properties(dbus_connection, &device))
			{
				// only wifi and ethernet connections can be handled at the moment
				if(device.type == 2 || device.type == 1)
				{
					if(i > 0)
					{
						json_pos = strlen(json_txt);
						strncpy(json_txt +json_pos, ",", MAX_JSON_LEN -json_pos);
					}
					i++;

					// write to json
					json_pos = strlen(json_txt);
					strncpy(json_txt +json_pos, "{\"name\":\"", MAX_JSON_LEN -json_pos);
					json_pos = strlen(json_txt);
					strncpy(json_txt +json_pos, device.interface, MAX_JSON_LEN -json_pos);
					json_pos = strlen(json_txt);

					if(device.type == 2)
					{
						strncpy(json_txt +json_pos, "\",\"ui_name\":\"WIFI (", MAX_JSON_LEN -json_pos);
						json_pos = strlen(json_txt);
						strncpy(json_txt +json_pos, device.interface, MAX_JSON_LEN -json_pos);
						json_pos = strlen(json_txt);
						strncpy(json_txt +json_pos, ")\",\"type\":1", MAX_JSON_LEN -json_pos);
					}
					else if(device.type == 1)
					{
						strncpy(json_txt +json_pos, "\",\"ui_name\":\"ETHERNET (", MAX_JSON_LEN -json_pos);
						json_pos = strlen(json_txt);
						strncpy(json_txt +json_pos, device.interface, MAX_JSON_LEN -json_pos);
						json_pos = strlen(json_txt);
						strncpy(json_txt +json_pos, ")\",\"type\":2", MAX_JSON_LEN -json_pos);
					}

					json_pos = strlen(json_txt);
					strncpy(json_txt +json_pos, "}", MAX_JSON_LEN -json_pos);
				}
			}
			else
				printf("qaul_network_devices_json networkmanager_device_get_property failed\n");
		}
		else
			printf("qaul_network_devices_json Unknown current type [%i]\n", current_type);

		dbus_message_iter_next(&iter_array);
	}
	dbus_message_unref(msg);

	return 1;
}

/**
 * find the first wifi device
 *
 * @retval 1 wifi device found, properties set
 * @retval 0 no wifi device found, properties not set
 */
int qaul_network_find_wifi(DBusConnection* dbus_connection, qaul_dbus_device_properties* device)
{
	DBusMessage* msg;
	DBusMessageIter iter, iter_array;
	int current_type;
	const char* device_path_ptr;

	printf("qaul_network_find_wifi\n");

	msg = networkmanager_dbus_method_call("GetDevices");
	if(msg == NULL)
		return 0;

	dbus_message_iter_init_append(msg, &iter);
	if(!networkmanager_dbus_send(&msg, dbus_connection))
	{
		printf("qaul_network_find_wifi networkmanager_dbus_send error\n");
		return 0;
	}
	if(!dbus_message_iter_init(msg, &iter)
		|| dbus_message_iter_get_arg_type(&iter) != DBUS_TYPE_ARRAY)
	{
		printf("qaul_network_find_wifi dbus_message_iter_init | DBUS_TYPE_ARRAY error\n");
		dbus_message_unref(msg);
		return 0;
	}

	// loop recursively over the array
	dbus_message_iter_recurse (&iter, &iter_array);
	while((current_type = dbus_message_iter_get_arg_type(&iter_array)) != DBUS_TYPE_INVALID)
	{
		if(current_type == DBUS_TYPE_OBJECT_PATH)
		{
			dbus_message_iter_get_basic(&iter_array, &device_path_ptr);
			strncpy(device->dbus_device_path, device_path_ptr, sizeof(device->dbus_device_path));

			// get interface name, type
			// dbus low level: dbus_message_new_method_call interface:"org.freedesktop.DBus.Properties" method:"Get" property:"(String)Interface, (UInt32)DeviceType"
			if(networkmanager_device_properties(dbus_connection, device))
			{
				if(device->type == 2 && device->state > 20)
				{
					printf("qaul_network_find_wifi wifi found: %s\n", device->interface);
					dbus_message_unref(msg);
					return 1;
				}
			}
			else
				printf("qaul_network_find_wifi networkmanager_device_get_property failed for %s\n", device->dbus_device_path);
		}
		else
			printf("qaul_network_find_wifi Unknown current type [%i]\n", current_type);

		dbus_message_iter_next(&iter_array);
	}
	dbus_message_unref(msg);

	printf("qaul_network_find_wifi no wifi device found\n");

	return 0;
}

/**
 * activate network connection
 *
 * The path to the access point can be given as accesspoint_path,
 * if it shall choose automatically use "/"
 *
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 * ActivateConnection ( o: connection, o: device, o: specific_object ) -> o
 */
int qaul_network_connection_activate(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings, qaul_dbus_device_properties* device)
{
	DBusMessage* msg;
	DBusMessageIter iter;
	char *connection_path, *device_path, *accesspoint_path, *active_connection_path, *error_msg;

	printf("qaul_network_activate_connection\n");

	connection_path = settings->dbus_connection_path;
	device_path = device->dbus_device_path;
	accesspoint_path = "/";

	msg = networkmanager_dbus_method_call("ActivateConnection");
	if(msg == NULL)
		return 0;

	dbus_message_iter_init_append(msg, &iter);
	dbus_message_iter_append_basic(&iter, DBUS_TYPE_OBJECT_PATH, &connection_path);
	dbus_message_iter_append_basic(&iter, DBUS_TYPE_OBJECT_PATH, &device_path);
	dbus_message_iter_append_basic(&iter, DBUS_TYPE_OBJECT_PATH, &accesspoint_path);
	if(!networkmanager_dbus_send(&msg, dbus_connection))
		return 0;

	if(!dbus_message_iter_init(msg, &iter))
	{
		printf("qaul_network_actiate_connection dbus_message_iter_init error\n");
		dbus_message_unref(msg);
		return 0;
	}
	if(dbus_message_iter_get_arg_type(&iter) != DBUS_TYPE_OBJECT_PATH)
	{
		if(dbus_message_iter_get_arg_type(&iter) == DBUS_TYPE_STRING)
		{
			dbus_message_iter_get_basic(&iter, &error_msg);
			printf("qaul_network_actiate_connection error: %s\n", error_msg);
		}
		else
			printf("qaul_network_actiate_connection dbus_message_iter_get_arg_type error\n");

		dbus_message_unref(msg);
		return 0;
	}
	dbus_message_iter_get_basic(&iter, &active_connection_path);

	strncpy(settings->dbus_active_connection_path, active_connection_path, sizeof(settings->dbus_active_connection_path));
	dbus_message_unref(msg);

	return 1;
}

/**
 * deactivate network connection
 *
 * https://developer.gnome.org/NetworkManager/unstable/spec.html
 * DeactivateConnection ( o: active_connection ) → nothing
 */
int qaul_network_connection_deactivate(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings)
{
	DBusMessage* msg;
	DBusMessageIter iter;
	char *connection_path;

	printf("qaul_network_deactivate_connection\n");

	connection_path = settings->dbus_active_connection_path;

	msg = networkmanager_dbus_method_call("DeactivateConnection");
	if(msg == NULL)
	{
		printf("qaul_network_deactivate_connection msg error\n");
		return 0;
	}

    dbus_message_iter_init_append(msg, &iter);
    dbus_message_iter_append_basic(&iter, DBUS_TYPE_OBJECT_PATH, &connection_path);
	if(!networkmanager_dbus_send(&msg, dbus_connection))
	{
		printf("qaul_network_deactivate_connection networkmanager_dbus_send error\n");
		return 0;
	}

	dbus_message_unref(msg);

	return 1;
}

/**
 * add connection settings to network manager
 */
int qaul_network_settings_add(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings, qaul_dbus_device_properties* device)
{
	DBusMessage* msg;
	DBusMessageIter iter;
	DBusMessageIter iter_array[7];
	char uuid[37];
	const char* connection_path_ptr;
	dbus_uint32_t wlan_channel_ui32;

	printf("qaul_network_settings_add\n");

	msg = dbus_message_new_method_call(
							"org.freedesktop.NetworkManager",
							"/org/freedesktop/NetworkManager/Settings",
							"org.freedesktop.NetworkManager.Settings",
							"AddConnection"
							);

	if(msg == NULL)
	{
		printf("qaul_network_settings_add msg error\n");
		return 0;
	}

	const char* connection_str = "connection";
	const char* connection_keys[]   = {"id",      "type", "uuid", "zone"};
	const char* connection_values[] = {"qaul.net", NULL,   NULL,  "public"};

	// connection uuid
	qaul_dbus_create_uuid(uuid);
	connection_values[2] = uuid;
	printf("uuid: %s\n", uuid);

	// configure settings type & structure
	if(device->type == 2)
	{
		connection_values[1] = "802-11-wireless";
	}
	else if(device->type == 1)
	{
		connection_values[1] = "802-3-ethernet";
	}

	dbus_message_iter_init_append(msg, &iter);

	// configure connection settings
	if(!(
		   dbus_message_iter_open_container (&iter, DBUS_TYPE_ARRAY, "{sa{sv}}", &iter_array[0])
		&& dbus_message_iter_open_container(&iter_array[0], DBUS_TYPE_DICT_ENTRY, NULL, &iter_array[1])
		&& dbus_message_iter_append_basic(&iter_array[1], DBUS_TYPE_STRING, &connection_str)
		&& dbus_message_iter_open_container(&iter_array[1], DBUS_TYPE_ARRAY, "{sv}", &iter_array[2])
		&& qaul_dbus_append_dict_entry(&iter_array[2], connection_keys[0], DBUS_TYPE_STRING, &connection_values[0])
		&& qaul_dbus_append_dict_entry(&iter_array[2], connection_keys[1], DBUS_TYPE_STRING, &connection_values[1])
		&& qaul_dbus_append_dict_entry(&iter_array[2], connection_keys[2], DBUS_TYPE_STRING, &connection_values[2])
		&& qaul_dbus_append_dict_entry(&iter_array[2], connection_keys[3], DBUS_TYPE_STRING, &connection_values[3])
		&& dbus_message_iter_close_container(&iter_array[1], &iter_array[2])
		&& dbus_message_iter_close_container(&iter_array[0], &iter_array[1])
		))
	{
		printf("qaul_network_settings_add append connection settings error\n");
		return 0;
	}

	// configure wireless device
	if(device->type == 2)
	{
		const char* wlan = connection_values[1];
		const char* wlan_keys[]   = {"ssid", "mode", "band", "channel"};
		const char* wlan_values[] = {NULL,  "adhoc", "bg"};
		wlan_channel_ui32 = settings->wifi_channel +0;

		if(!(
			   dbus_message_iter_open_container(&iter_array[0], DBUS_TYPE_DICT_ENTRY, NULL, &iter_array[1])
			&& dbus_message_iter_append_basic(&iter_array[1], DBUS_TYPE_STRING, &wlan)
			&& dbus_message_iter_open_container(&iter_array[1], DBUS_TYPE_ARRAY, "{sv}", &iter_array[2])

			// add hardware MAC address
			&& qaul_dbus_append_device_mac(&iter_array[2], device)

			// SSID dict entry as byte array
			&& dbus_message_iter_open_container(&iter_array[2], DBUS_TYPE_DICT_ENTRY, NULL, &iter_array[3])
			&& dbus_message_iter_append_basic(&iter_array[3], DBUS_TYPE_STRING, &wlan_keys[0])
			&& dbus_message_iter_open_container(&iter_array[3], DBUS_TYPE_VARIANT, "ay", &iter_array[4])
			&& dbus_message_iter_open_container(&iter_array[4], DBUS_TYPE_ARRAY, DBUS_TYPE_BYTE_AS_STRING, &iter_array[5])
			&& qaul_dbus_append_ssid(&iter_array[5], settings->wifi_ssid)
			&& dbus_message_iter_close_container(&iter_array[4], &iter_array[5])
			&& dbus_message_iter_close_container(&iter_array[3], &iter_array[4])
			&& dbus_message_iter_close_container(&iter_array[2], &iter_array[3])

			// end SSID dict entry
			&& qaul_dbus_append_dict_entry(&iter_array[2], wlan_keys[1], DBUS_TYPE_STRING, &wlan_values[1])
			&& qaul_dbus_append_dict_entry(&iter_array[2], wlan_keys[2], DBUS_TYPE_STRING, &wlan_values[2])
			&& qaul_dbus_append_dict_entry(&iter_array[2], wlan_keys[3], DBUS_TYPE_UINT32, &wlan_channel_ui32)
			&& dbus_message_iter_close_container(&iter_array[1], &iter_array[2])
			&& dbus_message_iter_close_container(&iter_array[0], &iter_array[1])
		))
		{
			printf("qaul_network_settings_add append wireless error\n");
			return 0;
		}
	}
	// configure Ethernet device
	else if(device->type == 1)
	{
		const char* ethernet = connection_values[1];

		if(!(
			   dbus_message_iter_open_container(&iter_array[0], DBUS_TYPE_DICT_ENTRY, NULL, &iter_array[1])
			&& dbus_message_iter_append_basic(&iter_array[1], DBUS_TYPE_STRING, &ethernet)
			&& dbus_message_iter_open_container(&iter_array[1], DBUS_TYPE_ARRAY, "{sv}", &iter_array[2])

			// add hardware MAC address
			&& qaul_dbus_append_device_mac(&iter_array[2], device)

			&& dbus_message_iter_close_container(&iter_array[1], &iter_array[2])
			&& dbus_message_iter_close_container(&iter_array[0], &iter_array[1])
		))
		{
			printf("qaul_network_settings_add append Ethernet error\n");
			return 0;
		}
	}

	// configure IP
	// check available settings:
	// http://sourcecodebrowser.com/network-manager/0.8/nm-setting-ip4-config_8h.html
	const char* ipv4 = "ipv4";
	const char* ipv4_method_key = "method";
	const char* ipv4_method_value = "manual";
	const char* ipv4_addresses = "addresses";
	dbus_uint32_t ipv4_address_ip;
	dbus_uint32_t ipv4_address_gateway;
	inet_pton(AF_INET, settings->ipv4_address, &ipv4_address_ip);
	inet_pton(AF_INET, settings->ipv4_gateway, &ipv4_address_gateway);
	dbus_uint32_t ipv4_address_netmask = settings->ipv4_netmask +0;
	const char* ipv4_dns = "dns";
	dbus_uint32_t ipv4_dns_ip1;
	dbus_uint32_t ipv4_dns_ip2;
	inet_pton(AF_INET, settings->ipv4_dns1, &ipv4_dns_ip1);
	inet_pton(AF_INET, settings->ipv4_dns2, &ipv4_dns_ip2);

	if(!(
		// append IP settings
		   dbus_message_iter_open_container(&iter_array[0], DBUS_TYPE_DICT_ENTRY, NULL, &iter_array[1])
		&& dbus_message_iter_append_basic(&iter_array[1], DBUS_TYPE_STRING, &ipv4)
		&& dbus_message_iter_open_container(&iter_array[1], DBUS_TYPE_ARRAY, "{sv}", &iter_array[2])

		// append IP method
		&& qaul_dbus_append_dict_entry(&iter_array[2], ipv4_method_key, DBUS_TYPE_STRING, &ipv4_method_value)

		// append IP addresses
		&& dbus_message_iter_open_container(&iter_array[2], DBUS_TYPE_DICT_ENTRY, NULL, &iter_array[3])
		&& dbus_message_iter_append_basic(&iter_array[3], DBUS_TYPE_STRING, &ipv4_addresses)
		&& dbus_message_iter_open_container(&iter_array[3], DBUS_TYPE_VARIANT, "aau", &iter_array[4])
		// append IP address
		&& dbus_message_iter_open_container(&iter_array[4], DBUS_TYPE_ARRAY, "au", &iter_array[5])
		&& dbus_message_iter_open_container(&iter_array[5], DBUS_TYPE_ARRAY, DBUS_TYPE_UINT32_AS_STRING, &iter_array[6])

		// append IP
		&& dbus_message_iter_append_basic(&iter_array[6], DBUS_TYPE_UINT32, &ipv4_address_ip)
		// append IP netmask
		&& dbus_message_iter_append_basic(&iter_array[6], DBUS_TYPE_UINT32, &ipv4_address_netmask)
		// append IP gateway
		&& dbus_message_iter_append_basic(&iter_array[6], DBUS_TYPE_UINT32, &ipv4_address_gateway)

		&& dbus_message_iter_close_container(&iter_array[5], &iter_array[6])
		&& dbus_message_iter_close_container(&iter_array[4], &iter_array[5])
		&& dbus_message_iter_close_container(&iter_array[3], &iter_array[4])
		&& dbus_message_iter_close_container(&iter_array[2], &iter_array[3])

		// append DNS settings
		&& dbus_message_iter_open_container(&iter_array[2], DBUS_TYPE_DICT_ENTRY, NULL, &iter_array[3])
		&& dbus_message_iter_append_basic(&iter_array[3], DBUS_TYPE_STRING, &ipv4_dns)
		&& dbus_message_iter_open_container(&iter_array[3], DBUS_TYPE_VARIANT, "au", &iter_array[4])
		&& dbus_message_iter_open_container(&iter_array[4], DBUS_TYPE_ARRAY, DBUS_TYPE_UINT32_AS_STRING, &iter_array[5])

		// append first DNS address
		&& dbus_message_iter_append_basic(&iter_array[5], DBUS_TYPE_UINT32, &ipv4_dns_ip1)
		// append second DNS address
		&& dbus_message_iter_append_basic(&iter_array[5], DBUS_TYPE_UINT32, &ipv4_dns_ip2)

		&& dbus_message_iter_close_container(&iter_array[4], &iter_array[5])
		&& dbus_message_iter_close_container(&iter_array[3], &iter_array[4])
		&& dbus_message_iter_close_container(&iter_array[2], &iter_array[3])

		// close IP settings
		&& dbus_message_iter_close_container(&iter_array[1], &iter_array[2])
		&& dbus_message_iter_close_container(&iter_array[0], &iter_array[1])
	))
	{
		printf("qaul_network_settings_add append DNS error\n");
		return 0;
	}

	// close settings
	if(!(
		dbus_message_iter_close_container(&iter, &iter_array[0])
	))
	{
		printf("qaul_network_settings_add close settings error\n");
		return 0;
	}

	if (!networkmanager_dbus_send(&msg, dbus_connection))
	{
		printf("qaul_network_settings_add networkmanager_dbus_send error\n");
		return 0;
	}

	if (!dbus_message_iter_init(msg, &iter)
	  || dbus_message_iter_get_arg_type(&iter) != DBUS_TYPE_OBJECT_PATH)
	{
		printf("qaul_network_settings_add != DBUS_TYPE_OBJECT_PATH error (%i), DBUS_TYPE_STRING %i \n", dbus_message_iter_get_arg_type(&iter), DBUS_TYPE_STRING);
		if(dbus_message_iter_get_arg_type(&iter) == DBUS_TYPE_STRING)
		{
			dbus_message_iter_get_basic(&iter, &connection_path_ptr);
			printf("qaul_network_settings_add error: %s\n", connection_path_ptr);
		}
		dbus_message_unref(msg);
		return 0;
	}

	dbus_message_iter_get_basic(&iter, &connection_path_ptr);
	strncpy(settings->dbus_connection_path, connection_path_ptr, sizeof(settings->dbus_connection_path));

	dbus_message_unref(msg);

	return 1;
}

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
int qaul_network_settings_delete(DBusConnection* dbus_connection, qaul_dbus_connection_settings* settings)
{
	DBusMessage* msg;
	DBusMessageIter iter;

	printf("qaul_network_delete_settings %s\n", settings->dbus_connection_path);

	msg = dbus_message_new_method_call(
							"org.freedesktop.NetworkManager",
							settings->dbus_connection_path,
							"org.freedesktop.NetworkManager.Settings.Connection",
							"Delete"
							);

	if(msg == NULL)
	{
		printf("qaul_network_delete_settings msg error\n");
		return 0;
	}

	dbus_message_iter_init_append(msg, &iter);
	if(!networkmanager_dbus_send(&msg, dbus_connection))
	{
		printf("qaul_network_delete_settings networkmanager_dbus_send error\n");
		return 0;
	}

	dbus_message_unref(msg);

	return 1;
}
