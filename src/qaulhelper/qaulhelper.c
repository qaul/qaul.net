/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * This file contains the main function for qaulhelper. 
 */

#include "qaulhelper.h"
#include "qaul/utils/validate.h"

int main (int argc, const char * argv[])
{
	// set OS specific  interface names for help messages
#ifdef QAUL_PORT_LINUX
	const char Help_InterfaceWifi[] = "wlan0";
	const char Help_InterfaceEther[] = "eth0";
#elif defined QAUL_PORT_OSX
	const char Help_InterfaceWifi[] = "en1";
	const char Help_InterfaceEther[] = "en0";
#else
	#error Something to fix
#endif

    if (argc > 1)
    {
        if(strncmp(argv[1], "startolsrd", 10) == 0)
        {
            start_olsrd(argc, argv);
        }
        else if(strncmp(argv[1], "stopolsrd", 9) == 0)
        {
            stop_olsrd(argc, argv);
        }
        else if(strncmp(argv[1], "startportforwarding", 19) == 0)
        {
            start_portforwarding(argc, argv);
        }
        else if(strncmp(argv[1], "stopportforwarding", 18) == 0)
        {
            stop_portforwarding(argc, argv);
        }
        else if(strncmp(argv[1], "startgateway", 12) == 0)
        {
            start_gateway(argc, argv);
        }
        else if(strncmp(argv[1], "stopgateway", 11) == 0)
        {
            stop_gateway(argc, argv);
        }
#ifdef QAUL_PORT_OSX
        else if(strncmp(argv[1], "enablewifi", 10) == 0)
        {
            enable_wifi(argc, argv);
        }
        else if(strncmp(argv[1], "disablewifi", 11) == 0)
        {
            disable_wifi(argc, argv);
        }
        else if(strncmp(argv[1], "createnetworkprofile", 20) == 0)
        {
            create_networkprofile(argc, argv);
        }
        else if(strncmp(argv[1], "switchnetworkprofile", 20) == 0)
        {
            switch_networkprofile(argc, argv);
        }
        else if(strncmp(argv[1], "deletenetworkprofile", 20) == 0)
        {
            delete_networkprofile(argc, argv);
        }
        else if(strncmp(argv[1], "createibss", 10) == 0)
        {
            create_ibss(argc, argv);
        }
        else if(strncmp(argv[1], "setdhcp", 7) == 0)
        {
            set_dhcp(argc, argv);
        }
#endif // QAUL_PORT_OSX
#ifdef QAUL_PORT_LINUX
        else if(strncmp(argv[1], "configurewifi", 10) == 0)
        {
            configure_wifi(argc, argv);
        }
#ifdef QAUL_STOP_NETWORKING
        else if(strncmp(argv[1], "stopnetworking", 10) == 0)
        {
        	stop_networking(argc, argv);
        }
        else if(strncmp(argv[1], "restartnetworking", 10) == 0)
        {
        	restart_networking(argc, argv);
        }
#endif // QAUL_STOP_NETWORKING
#endif // QAUL_PORT_LINUX
        else if(strncmp(argv[1], "setip", 5) == 0)
        {
            set_ip(argc, argv);
        }
        else if(strncmp(argv[1], "setdns", 6) == 0)
        {
            set_dns(argc, argv);
        }
#ifdef QAUL_PORT_LINUX
        else if(strncmp(argv[1], "removedns", 6) == 0)
        {
            set_dns(argc, argv);
        }
#endif // QAUL_PORT_LINUX
        else
        {
            printf("unknown command '%s'\n", argv[1]);
            printf("execute qaulhelper without arguments to see help instructions.\n");
        }
    }
    else
    {
        printf("\n");
        printf("qaulhelper executes helper functions for qaul.net\n\n");
        printf("usage:\n");
        printf("  qaulhelper startolsrd <ISGATEWAY yes|no> <INTERFACE>\n");
        printf("  qaulhelper startolsrd yes %s\n", Help_InterfaceWifi);
        printf("  qaulhelper stopolsrd\n");
        printf("  qaulhelper stopolsrd\n");
#ifdef QAUL_PORT_LINUX
        printf("  qaulhelper startportforwarding <INTERFACE> <IP>\n");
        printf("  qaulhelper startportforwarding %s 10.213.28.55\n", Help_InterfaceWifi);
#endif // QAUL_PORT_LINUX
#ifdef QAUL_PORT_OSX
        printf("  qaulhelper startportforwarding <INTERFACE>\n");
        printf("  qaulhelper startportforwarding %s\n", Help_InterfaceWifi);
#endif // QAUL_PORT_OSX
        printf("  qaulhelper stopportforwarding\n");
        printf("  qaulhelper stopportforwarding\n");
#ifdef QAUL_PORT_LINUX
        printf("  qaulhelper startgateway <INTERFACE OUT> <INTERFACE IN>\n");
        printf("  qaulhelper startgateway %s %s\n", Help_InterfaceEther, Help_InterfaceWifi);
        printf("  qaulhelper stopgateway <INTERFACE OUT> <INTERFACE IN>\n");
        printf("  qaulhelper stopgateway %s %s\n", Help_InterfaceEther, Help_InterfaceWifi);
        printf("  qaulhelper configurewifi <INTERFACE> <ESSID> <FREQENCY> [<BSSID>]\n");
        printf("  qaulhelper configurewifi %s qaul.net 2462 02:11:87:88:D6:FF\n", Help_InterfaceWifi);
#ifdef QAUL_STOP_NETWORKING
        printf("  qaulhelper stopnetworking\n");
        printf("  qaulhelper stopnetworking\n");
        printf("  qaulhelper restartnetworking\n");
        printf("  qaulhelper restartnetworking\n");
#endif // QAUL_STOP_NETWORKING
#endif // QAUL_PORT_LINUX
#ifdef QAUL_PORT_OSX
        printf("  qaulhelper startgateway <INTERFACE OUT>\n");
        printf("  qaulhelper startgateway %s\n", Help_InterfaceEther);
        printf("  qaulhelper stopgateway\n");
        printf("  qaulhelper stopgateway\n");
        printf("  qaulhelper configurewifi <INTERFACE> <ESSID> <CHANNEL> [<BSSID>]\n");
        printf("  qaulhelper configurewifi %s qaul.net 11 02:11:87:88:D6:FF\n", Help_InterfaceWifi);
#endif // QAUL_PORT_OSX
        printf("  qaulhelper setip <INTERFACE> <IP> <SUBNET> <BROADCAST>\n");
        printf("  qaulhelper setip %s 10.213.28.55 8 10.255.255.255\n", Help_InterfaceWifi);
        printf("  qaulhelper setdns <INTERFACE>\n");
        printf("  qaulhelper setdns %s\n", Help_InterfaceWifi);
#ifdef QAUL_PORT_LINUX
        printf("  qaulhelper removedns <INTERFACE>\n");
        printf("  qaulhelper removedns %s\n", Help_InterfaceWifi);
#endif // QAUL_PORT_LINUX
#ifdef QAUL_PORT_OSX
        printf("  qaulhelper enablewifi <OSXVERSION> <INTERFACE>\n");
        printf("  qaulhelper enablewifi 1038 %s\n", Help_InterfaceWifi);
        printf("  qaulhelper disablewifi <OSXVERSION> <INTERFACE>\n");
        printf("  qaulhelper disablewifi 1038 %s\n", Help_InterfaceWifi);
        printf("  qaulhelper createnetworkprofile <POFILENAME>\n");
        printf("  qaulhelper createnetworkprofile qaul.net\n");
        printf("  qaulhelper switchnetworkprofile <POFILENAME>\n");
        printf("  qaulhelper switchnetworkprofile qaul.net\n");
        printf("  qaulhelper deletenetworkprofile <POFILENAME>\n");
        printf("  qaulhelper deletenetworkprofile qaul.net\n");
        printf("  qaulhelper setip <SERVICENAME> <IP> <SUBNET> <ROUTER>\n");
        printf("  qaulhelper setip Wi-Fi 10.213.28.55 255.0.0.0 0.0.0.0\n");
        printf("  qaulhelper setdhcp <SERVICENAME> <INTERFACE>\n");
        printf("  qaulhelper setdhcp Wi-Fi %s\n", Help_InterfaceWifi);
        printf("  qaulhelper setdns <SERVICENAME>\n");
        printf("  qaulhelper setdns Wi-Fi\n");
        printf("\n");
        printf("only for OSX <= 10.5\n");
        printf("  qaulhelper createibss <ESSID> <CHANNEL>\n");
        printf("  qaulhelper createibss qaul.net 11\n");
#endif // QAUL_PORT_OSX
        printf("\n");
    }

    return 0;
}
