/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

//#include <regex.h>
#include <stdio.h>
#include <string.h>

#include <QaulConfig.h>
#include "configure.h"
#include "configure_cli.h"


/**
 * Check if @a interface_name is a wifi interface
 *
 * @retval 1   wifi interface
 * @retval 0   not a wifi interface
 * @retval -1  failed to run program iw
 */
static int qaul_interfaceIsWifi_cli(const char* interface_name)
{
	FILE *fp;
	char line[1024], command[255];
	int success, lines;

	success = 0;
	lines = 0;

	// Open the command for reading
	sprintf(command, "/sbin/iw %s info", interface_name);
	fp = popen(command, "r");
	if (fp == NULL)
	{
		printf("Failed to run command\n" );
		return -1;
	}

	// process output one line at a time
	while (fgets(line, sizeof(line)-1, fp) != NULL)
	{
		lines++;
	}

	// close the the file pointer
	pclose(fp);

	if(lines > 1)
	{
		printf("%s is a wifi interface\n", interface_name);
		success = 1;
	}
	else
		printf("%s is not a wifi interface\n", interface_name);

	return success;
}

/**
 * Check if @a interface_name is not configurable
 *
 * @retval 1   configurable
 * @retval 0   not configurable
 * @retval -1  failed to run program
 * @retval -2  interface not found
 */
static int qaul_interfaceConfigurable_cli(const char* interface_name)
{
	FILE *fp;
	char line[1024], command[256];
	int success = 1;

	// check if interface is 'lo'
	if(strncmp(interface_name, "lo", 2) == 0)
	{
		printf("lo interface found\n" );
		return 0;
	}

	// check if interface is a virtual interface
	snprintf(command, sizeof(command)-1, "ls -la /sys/class/net/%s", interface_name);
	strncpy(&command[sizeof(command)-1], "\0", 1);
	fp = popen(command, "r");
	if (fp == NULL)
	{
		printf("Failed to run command\n");
		return -1;
	}

	// process output one line at a time
	while (fgets(line, sizeof(line)-1, fp) != NULL)
	{
		printf("%s", line);

		// search for pattern "/virtual/" in line
		if(strstr(line, "/virtual/") == NULL)
		{
			printf("%s is a physical interface\n", interface_name);
			success = 1;
		}
		else
			printf("%s is a virtual interface\n", interface_name);
	}

	// close the the file pointer
	pclose(fp);

	return success;

}

/**
 * Check network interface type of @a interface_name
 *
 * @retval 0  unknown or unconfigurable interface (this interface will not be shown)
 * @retval 1  ethernet interface
 * @retval 2  wifi interface
 */
static int qaul_interfaceType_cli(const char* interface_name)
{
	FILE *fp;
	char path[1024];

	if(qaul_interfaceIsWifi_cli(interface_name))
	{
		printf("%s is a wifi interface\n", interface_name);
		return 2;
	}
	else if(qaul_interfaceConfigurable_cli(interface_name))
	{
		printf("%s is a physical interface\n", interface_name);
		return 1;
	}

	printf("%s is an unconfigurable interface\n", interface_name);
	return 0;

}


int qaul_findWifiInterface_cli(qaul_network_settings* network)
{
	FILE *fp;
	char line[1024];
	int success, last_space;

	success = 0;

	// Open the command for reading
	fp = popen("/sbin/iw dev", "r");
	if (fp == NULL)
	{
		printf("Failed to run command\n" );
		return success;
	}

	// process output one line at a time
	while(fgets(line, sizeof(line)-1, fp) != NULL)
	{
		printf("%s", line);

		// find: \t\tInterface wlan0
		if(strncmp(line, "\t", 1) == 0)
		{
			if(strncmp(&line[1], "\t", 1) != 0)
			{
				// find last
				for(last_space = strlen(line)-1; last_space > 1; last_space--)
				{
					if(strncmp(&line[last_space], " ", 1) == 0)
					{
						// save interface name
						last_space++;
						strncpy(network->interface_name, &line[last_space], strlen(line)-last_space-1);
						strncpy(&network->interface_name[strlen(line)-last_space-1], "\0", 1);
						success = 1;
						break;
					}
				}

				if(success == 1)
					break;
			}
		}
	}

	// close the the file pointer
	pclose(fp);

	return success;
}

int qaul_findNetworkInterface_cli(const char* interface_name)
{
	FILE *fp;
	char line[1024], command[256];
	int success = 0;

	// create command
	snprintf(command, sizeof(command)-1, "/bin/ip link show %s", interface_name);

	// Open the command for reading
	fp = popen(command, "r");
	if (fp == NULL)
	{
		printf("command failed: %s\n", command);
		return success;
	}

	// process output one line at a time
	while (fgets(line, sizeof(line)-1, fp) != NULL)
	{
		printf("%s", line);

		if(strncmp(&line[1], ":", 1) == 0)
		{
			printf("network interface found: %s\n", interface_name);
			success = 1;
		}
		else
			printf("network interface not found: %s\n", interface_name);

		break;
	}

	// close the the file pointer
	pclose(fp);

	return success;
}

/**
 * extracts the network interface name from an output line
 */
static int qaul_extractNetworkInterface_cli(const char* line, char* interface_name)
{
	int start, len, success;
	success = 0;
	start = 0;
	len = 0;

	// find: '3: wlan0: '
	if(strncmp(&line[1], ": ", 2) == 0)
		start = 3;
	else if(strncmp(&line[2], ": ", 2) == 0)
		start = 4;
	
	if(start > 0)
	{
		// find last
		for(len = 0; start + len < strlen(line); len++)
		{
			if(strncmp(&line[start +len], ":", 1)==0)
			{
				break;
			}
		}
		
		if(len > 0 && len < IFNAMSIZ)
		{
			strncpy(interface_name, &line[start], len);
			strncpy(&interface_name[len], "\0", len);
			success = 1;
			
			printf("interface_name: %s\n", interface_name);
		}
	}

	return success;
}

int qaul_getInterfacesJson_cli(char* json_txt)
{
	FILE *fp;
	char line[1024], interface_name[IFNAMSIZ];
	int success, type, json_pos, i;

	success = 0;
	i = 0;
	
	// reset json interfaces list
	strncpy(json_txt, "\0", 1);
	
	printf("qaul_getInterfacesJson_cli\n");
	
	// Open the command for reading
	fp = popen("/bin/ip link show", "r");
	if (fp == NULL)
	{
		printf("command failed: /bin/ip link show\n");
		return success;
	}

	// process output one line at a time
	while (fgets(line, sizeof(line)-1, fp) != NULL)
	{
		success = 1;
		
		if(qaul_extractNetworkInterface_cli(line, interface_name))
		{
			// check the interface type
			type = qaul_interfaceType_cli(interface_name);

			// only wifi and ethernet connections can be handled at the moment
			if(type == 2 || type == 1)
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
				strncpy(json_txt +json_pos, interface_name, MAX_JSON_LEN -json_pos);
				json_pos = strlen(json_txt);

				if(type == 2)
				{
					strncpy(json_txt +json_pos, "\",\"ui_name\":\"WIFI (", MAX_JSON_LEN -json_pos);
					json_pos = strlen(json_txt);
					strncpy(json_txt +json_pos, interface_name, MAX_JSON_LEN -json_pos);
					json_pos = strlen(json_txt);
					strncpy(json_txt +json_pos, ")\",\"type\":1", MAX_JSON_LEN -json_pos);
				}
				else if(type == 1)
				{
					strncpy(json_txt +json_pos, "\",\"ui_name\":\"ETHERNET (", MAX_JSON_LEN -json_pos);
					json_pos = strlen(json_txt);
					strncpy(json_txt +json_pos, interface_name, MAX_JSON_LEN -json_pos);
					json_pos = strlen(json_txt);
					strncpy(json_txt +json_pos, ")\",\"type\":2", MAX_JSON_LEN -json_pos);
				}

				json_pos = strlen(json_txt);
				strncpy(json_txt +json_pos, "}", MAX_JSON_LEN -json_pos);
			}
		}
	}

	// close the the file pointer
	pclose(fp);

	return success;
}

void qaul_networkStart_cli(void)
{
	char command[255];
	
	printf("qaul_networkStart_cli\n");

	// configure wifi
	// qaulhelper configurewifi <INTERFACE> <ESSID> <CHANNEL> [<BSSID>]
	// qaulhelper configurewifi wlan0 qaul.net 11 02:11:87:88:D6:FF
	if(qaul_interfaceIsWifi_cli(network_settings.interface_name))
	{
		sprintf(command, "%s/lib/qaul/bin/qaulhelper configurewifi %s %s %i", QAUL_ROOT_PATH, network_settings.interface_name, network_settings.wifi_ssid, network_settings.wifi_channel);
		system(command);
	}

	// configure ip
	// qaulhelper setip <INTERFACE> <IP> <SUBNET> <BROADCAST>
	// qaulhelper setip wlan0 10.213.28.55 8 10.255.255.255
	sprintf(command, "%s/lib/qaul/bin/qaulhelper setip %s %s %i %s", QAUL_ROOT_PATH, network_settings.interface_name, network_settings.ipv4_address, network_settings.ipv4_netmask, network_settings.ipv4_broadcast);
	system(command);

	// set DNS
	// qaulhelper setdns <INTERFACE>
	// qaulhelper setdns wlan0
	sprintf(command, "%s/lib/qaul/bin/qaulhelper setdns %s", QAUL_ROOT_PATH, network_settings.interface_name);
	system(command);
}

void qaul_networkStop_cli(void)
{
	char command[255];

	// set Interface to DHCP ?
	
	// remove DNS
	sprintf(command, "%s/lib/qaul/bin/qaulhelper removedns %s", QAUL_ROOT_PATH, network_settings.interface_name);
	system(command);	
}
