/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <stdio.h>
#include <stdlib.h>

#include "qaullib.h"
#include <QaulConfig.h>

#include "configure.h"
#include "configure_nm.h"
#include "configure_cli.h"


void qaul_defineConfigurationMethod(void)
{
	printf("qaul_defineConfigurationMethod\n" );

	// check if network manager is running
	if(qaul_initNetworkManager())
	{
		printf("qaul_defineConfigurationMethod == NETWORK_MANAGER\n" );
		network_settings.method = NETWORK_MANAGER;
	}
	else
	{
		printf("qaul_defineConfigurationMethod == CLI\n" );
		network_settings.method = CLI;
	}
}

int qaul_findWifiInterface(qaul_network_settings* settings)
{
	int success;

	printf("qaul_findWifiInterface\n" );

	if(settings->method == NETWORK_MANAGER)
		success = qaul_findWifiInterface_nm(settings);
	else
		success = qaul_findWifiInterface_cli(settings);

	return success;
}

int qaul_findNetworkInterface(const char* interface_name)
{
	int success;

	printf("qaul_findNetworkInterface\n" );

	if(network_settings.method == NETWORK_MANAGER)
		success = qaul_findNetworkInterface_nm(interface_name);
	else
		success = qaul_findNetworkInterface_cli(interface_name);

	return success;
}

int qaul_getInterfacesJson(char* json_txt)
{
	int success;

	printf("qaul_getInterfacesJson\n" );

	if(network_settings.method == NETWORK_MANAGER)
		success = qaul_getInterfacesJson_nm(json_txt);
	else
		success = qaul_getInterfacesJson_cli(json_txt);

	return success;
}

void qaul_networkStart(void)
{
	printf("qaul_networkStart\n" );

	if(network_settings.method == NETWORK_MANAGER)
		qaul_networkStart_nm();
	else
		qaul_networkStart_cli();
}

void qaul_networkStop(void)
{
	printf("qaul_networkStop\n" );

	if(network_settings.method == NETWORK_MANAGER)
		qaul_networkStop_nm();
	else
		qaul_networkStop_cli();
}

void qaul_olsrdStart(void)
{
	char command[255];

	printf("qaul_olsrdStart\n");

	if(Qaullib_IsGateway())
		sprintf(command, "%s/lib/qaul/bin/qaulhelper startolsrd yes %s", QAUL_ROOT_PATH, network_settings.interface_name);
	else
		sprintf(command, "%s/lib/qaul/bin/qaulhelper startolsrd no %s", QAUL_ROOT_PATH, network_settings.interface_name);
	system(command);

	printf("command: %s\n", command);
}

void qaul_olsrdStop(void)
{
	char command[255];
	sprintf(command, "%s/lib/qaul/bin/qaulhelper stopolsrd", QAUL_ROOT_PATH);
	system(command);
}

void qaul_startPortForwarding(void)
{
	char command[255];
	sprintf(command, "%s/lib/qaul/bin/qaulhelper startportforwarding %s %s", QAUL_ROOT_PATH, network_settings.interface_name, network_settings.ipv4_address);
	system(command);
}

void qaul_stopPortForwarding(void)
{
	char command[255];
	sprintf(command, "%s/lib/qaul/bin/qaulhelper stopportforwarding", QAUL_ROOT_PATH);
	system(command);
}

void qaul_startGateway(void)
{
	char command[255];
	sprintf(command, "%s/lib/qaul/bin/qaulhelper startgateway %s %s", QAUL_ROOT_PATH, Qaullib_GetGatewayInterface(), network_settings.interface_name);
	system(command);
}

void qaul_stopGateway(void)
{
	char command[255];
	sprintf(command, "%s/lib/qaul/bin/qaulhelper stopgateway %s %s", QAUL_ROOT_PATH, Qaullib_GetGatewayInterface(), Qaullib_GetInterface());
	system(command);
}
