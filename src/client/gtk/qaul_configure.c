/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <stdio.h>
#include <string.h>

#include "qaullib.h"
#include "structures.h"
#include "qaul_configure.h"
#include "configure.h"

int username_flag;

int qaul_configure(void)
{
    // initialize qaul library
    if(qaulConfigureCounter == 0)
    {
        // everything is fine
        Qaullib_ConfigStart();
        qaulConfigureCounter = 3;
        username_flag = 0;
    }

    // check authorization
    if(qaulConfigureCounter == 10)
    {
        // nothing to be done here
    	qaulConfigureCounter = 20;
    }

    // TODO: enable networking

    // get network interface
    if(qaulConfigureCounter == 20)
    {
        // decide which configuration method to choose
    	qaul_defineConfigurationMethod();

    	// check if interface has been configured manually
    	if(Qaullib_GetInterfaceManual())
    	{
    		printf("[configure] interface manually configured\n");
    		if(qaul_findNetworkInterface(Qaullib_GetInterface()))
    		{
    			network_interface_found = 1;
    			strncpy(network_settings.interface_name, Qaullib_GetInterface(), sizeof(network_settings.interface_name));
    			strncpy(&network_settings.interface_name[sizeof(network_settings.interface_name)], "\0", 1);
    		}
    		else
    			printf("[configure] manually configured interface \"%s\" not found\n", Qaullib_GetInterface());
    	}
    	// find wifi interface
    	else
    	{
    		if(qaul_findWifiInterface(&network_settings))
    		{
    			Qaullib_SetInterface(network_settings.interface_name);
    			network_interface_found = 1;
    		}
    		else
    			printf("[configure] No WiFi interface found\n");
    	}

    	// TODO: enable wifi

    	qaulConfigureCounter = 21;
    }

    // configure network interface
    if(qaulConfigureCounter == 21)
    {
        if(network_interface_found)
        {
        	printf("[configure] Network interface %s\n", network_settings.interface_name);
        	fprintf(stderr,"IP: %s\n\n", Qaullib_GetIP());

        	// get network configuration
        	strncpy(network_settings.ipv4_address, Qaullib_GetIP(), sizeof(network_settings.ipv4_address));
        	Qaullib_GetConfString("net.gateway", network_settings.ipv4_gateway);
        	network_settings.ipv4_netmask = Qaullib_GetConfInt("net.mask");
        	Qaullib_GetConfString("net.broadcast", network_settings.ipv4_broadcast);
        	strncpy(network_settings.ipv4_dns1, "5.45.96.220", sizeof(network_settings.ipv4_dns1));
        	strncpy(network_settings.ipv4_dns2, "185.82.22.133", sizeof(network_settings.ipv4_dns2));
        	network_settings.wifi_channel = Qaullib_GetConfInt("wifi.channel");
        	Qaullib_GetConfString("wifi.ssid", network_settings.wifi_ssid);
		
        	// start network
        	qaul_networkStart();
        }

    	qaulConfigureCounter = 29;
    }

    // check if username is set
    if(qaulConfigureCounter == 30)
    {
        if(Qaullib_ExistsUsername())
			qaulConfigureCounter = 40;
        else
        {
        	// wait until user name is set
        	if(username_flag == 0)
        	{
        		username_flag = 1;
        		fprintf(stderr,"Waiting until a username is set ...\n");
        		fprintf(stderr,"Open http://localhost:8081/qaul.html in your web browser to set it ...\n\n");
        	}

        	// wait
            qaulConfigureCounter--;
        }
    }

    // start olsrd
    if(qaulConfigureCounter == 40)
    {
    	printf("[configure] start olsrd \n");

    	// stop already running olsrd
    	qaul_olsrdStop();

        // start olsrd
        qaul_olsrdStart();

        qaulConfigureCounter = 44;
    }

    // connect ipc
    if(qaulConfigureCounter == 45)
    {
        printf("[configure] connect ipc \n");
        Qaullib_IpcConnect();
        qaulConfigureCounter = 46;
    }

    // start captive portal
    if(qaulConfigureCounter == 46)
    {
    	printf("[configure] start captive portal \n");
    	Qaullib_SetConfVoIP();
        Qaullib_UDP_StartServer();
        Qaullib_CaptiveStart();

        // configure firewall
        qaul_startPortForwarding();
        if(Qaullib_IsGateway())
        	qaul_startGateway();

        qaulConfigureCounter = 50;
    }

    // start timers
    if(qaulConfigureCounter == 50)
    {
        printf("[configure] timers \n");

		// start timers
        qaul_startTimers();

        Qaullib_ConfigurationFinished();

        qaulConfigureCounter = 60;
    }

    // end configuration
	if(qaulConfigureCounter >= 60)
	{
		printf("[configure] Finished\n");
		return 0;
	}

	qaulConfigureCounter++;
	return 1;
}
