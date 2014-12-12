/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * qaul helper functions running as root user
 *
 * For this executable the suid bit needs to be set, to be
 * callable from userspace. This is done by the qaul.net installer.
 * The path of the installed qaulhelper binary is:
 *   /usr/share/qaul/qaulhelper
 *
 * To set the suid bit manually for testing, open a Terminal, navigate
 * to the qaulhelper executable binary and type:
 *   sudo chown root qaulhelper
 *   sudo chgrp root qaulhelper
 *   sudo chmod 6755 qaulhelper
 *
 *
 * usage:
 *   qaulhelper startolsrd <ISGATEWAY yes|no> <INTERFACE>
 *   qaulhelper startolsrd yes wlan0
 *   qaulhelper stopolsrd
 *   qaulhelper stopolsrd
 *   qaulhelper startportforwarding <INTERFACE> <IP>
 *   qaulhelper startportforwarding wlan0 10.213.28.55
 *   qaulhelper stopportforwarding
 *   qaulhelper stopportforwarding
 *   qaulhelper startgateway <INTERFACE>
 *   qaulhelper startgateway wlan0
 *   qaulhelper stopgateway
 *   qaulhelper stopgateway
 *   qaulhelper startnetworkmanager
 *   qaulhelper startnetworkmanager
 *   qaulhelper stopnetworkmanager
 *   qaulhelper stopnetworkmanager
 *   qaulhelper enablewifi <INTERFACE>
 *   qaulhelper enablewifi wlan0
 *   qaulhelper configurewifi <INTERFACE> <ESSID> <CHANNEL> [<BSSID>]
 *   qaulhelper configurewifi wlan0 qaul.net 11 02:11:87:88:D6:FF
 *   qaulhelper setip <INTERFACE> <IP> <SUBNET> <BROADCAST>
 *   qaulhelper setip wlan0 10.213.28.55 8 10.255.255.255
 *   qaulhelper setdns <INTERFACE>
 *   qaulhelper setdns wlan0
 *
 */


#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>



/**
 * start olsrd
 */
int start_olsrd (int argc, const char * argv[]);

/**
 * stop olsrd
 */
int stop_olsrd (int argc, const char * argv[]);

/**
 * start port forwarding
 */
int start_portforwarding (int argc, const char * argv[]);

/**
 * stop port forwarding
 */
int stop_portforwarding (int argc, const char * argv[]);

/**
 * start network manager
 */
int start_networkmanager (int argc, const char * argv[]);

/**
 * stop network manager
 */
int stop_networkmanager (int argc, const char * argv[]);

/**
 * enable wifi
 */
int enable_wifi (int argc, const char * argv[]);

/**
 * configure wifi, create or join IBSS
 */
int configure_wifi (int argc, const char * argv[]);

/**
 * configure static IP
 */
int set_ip (int argc, const char * argv[]);

/**
 * set dns servers
 */
int set_dns (int argc, const char * argv[]);

/**
 * start gateway
 *
 * this function needs to be called as root
 */
int start_gateway (int argc, const char * argv[]);

/**
 * stop gateway
 */
int stop_gateway (int argc, const char * argv[]);


/**
 * validate IP argument
 */
int validate_ip (const char* str);

/**
 * validate interface argument
 */
int validate_interface (const char* str);

/**
 * validate essid argument
 */
int validate_essid (const char* str);

/**
 * validate service argument
 */
int validate_bssid (const char* str);

/**
 * validate number argument
 */
int validate_number (const char* str);

/**
 * validate path argument
 */
int validate_path (const char* str);

/**
 * check if the char is a number
 */
int validate_char_number (char mychar);

/**
 * check if the char is an ascii letter
 */
int validate_char_letter (char mychar);

/**
 * check char for problematic entities
 */
int validate_char_problematic (char mychar);


int main (int argc, const char * argv[])
{
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
        else if(strncmp(argv[1], "startgateway", 12) == 0)
        {
            start_gateway(argc, argv);
        }
        else if(strncmp(argv[1], "stopgateway", 11) == 0)
        {
            stop_gateway(argc, argv);
        }
#ifdef WITHOUT_NETWORKMANAGER
        else if(strncmp(argv[1], "stopportforwarding", 18) == 0)
        {
            stop_portforwarding(argc, argv);
        }
        else if(strncmp(argv[1], "startnetworkmanager", 19) == 0)
        {
            start_networkmanager(argc, argv);
        }
        else if(strncmp(argv[1], "stopnetworkmanager", 18) == 0)
        {
            stop_networkmanager(argc, argv);
        }
        else if(strncmp(argv[1], "enablewifi", 10) == 0)
        {
            enable_wifi(argc, argv);
        }
        else if(strncmp(argv[1], "configurewifi", 10) == 0)
        {
            configure_wifi(argc, argv);
        }
        else if(strncmp(argv[1], "setip", 5) == 0)
        {
            set_ip(argc, argv);
        }
        else if(strncmp(argv[1], "setdns", 6) == 0)
        {
            set_dns(argc, argv);
        }
#endif // WITHOUT_NETWORKMANAGER
        else
        {
            printf("unknown command ...\n");
            printf("execute qaulhelper without arguments to see help instructions.\n");
        }
    }
    else
    {
        printf("\n");
        printf("qaulhelper executes helper functions for qaul.net\n\n");
        printf("usage:\n");
        printf("  qaulhelper startolsrd <ISGATEWAY yes|no> <INTERFACE>\n");
        printf("  qaulhelper startolsrd yes wlan0\n");
        printf("  qaulhelper stopolsrd\n");
        printf("  qaulhelper stopolsrd\n");
        printf("  qaulhelper startportforwarding <INTERFACE> <IP>\n");
        printf("  qaulhelper startportforwarding wlan0 10.213.28.55\n");
        printf("  qaulhelper stopportforwarding\n");
        printf("  qaulhelper stopportforwarding\n");
        printf("  qaulhelper startgateway <INTERFACE>\n");
        printf("  qaulhelper startgateway wlan0\n");
        printf("  qaulhelper stopgateway\n");
        printf("  qaulhelper stopgateway\n");
#ifdef WITHOUT_NETWORKMANAGER
        printf("  qaulhelper startnetworkmanager\n");
        printf("  qaulhelper startnetworkmanager\n");
        printf("  qaulhelper stopnetworkmanager\n");
        printf("  qaulhelper stopnetworkmanager\n");
        printf("  qaulhelper enablewifi <INTERFACE>\n");
        printf("  qaulhelper enablewifi wlan0\n");
        printf("  qaulhelper configurewifi <INTERFACE> <ESSID> <CHANNEL> [<BSSID>]\n");
        printf("  qaulhelper configurewifi wlan0 qaul.net 11 02:11:87:88:D6:FF\n");
        printf("  qaulhelper setip <INTERFACE> <IP> <SUBNET> <BROADCAST>\n");
        printf("  qaulhelper setip wlan0 10.213.28.55 8 10.255.255.255\n");
        printf("  qaulhelper setdns <INTERFACE>\n");
        printf("  qaulhelper setdns wlan0\n");
#endif // WITHOUT_NETWORKMANAGER
        printf("\n");
    }

    return 0;
}


int start_olsrd (int argc, const char * argv[])
{
    pid_t pid1;
    int status, fd;
    char s[256];
    printf("start olsrd\n");

    if(argc >= 4)
    {
        // validate arguments
        if(strncmp(argv[3], "yes", 3)==0)
            sprintf(s,"/opt/qaul/etc/olsrd_linux_gw.conf");
        else
            sprintf(s,"/opt/qaul/etc/olsrd_linux.conf");

        if (validate_interface(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }

        // become root
        setuid(0);

        // start olsrd
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/opt/qaul/bin/olsrd", "olsrd", "-f", s, "-i", argv[3], "-d", "0", (char*)0);
        }
        else
            waitpid(pid1, &status, 0);

        printf("olsrd started\n");
    }
    else
        printf("missing argument\n");

	return 0;
}

int stop_olsrd (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("stop olsrd\n");

    // become root
    setuid(0);

    // kill olsrd
    pid1 = fork();
    if (pid1 < 0)
        printf("fork for pid1 failed\n");
    else if(pid1 == 0)
        execl("/usr/bin/killall", "killall", "olsrd", (char*)0);
    else
        waitpid(pid1, &status, 0);

    printf("olsrd stopped\n");
	return 0;
}

int start_portforwarding (int argc, const char * argv[])
{
    pid_t pid1, pid2, pid3;
    int fd, status;
    printf("start portforwarding\n");

    if(argc >= 4)
    {
        // validate arguments
        if (validate_interface(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }

        // become root
        setuid(0);

        // forward tcp port 80 (http) by iptables
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            // redirect standart output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/iptables", "iptables", "-t", "nat", "-I", "PREROUTING", "1", "-i", argv[2], "-p", "tcp", "-d", argv[3], "--dport", "80", "-j", "REDIRECT", "--to-port", "8081", (char*)0);
        }
        else
            printf("tcp port 80 forwarded\n");

        // forward udp port 53 (dns) by iptables
        pid2 = fork();
        if (pid2 < 0)
            printf("fork for pid2 failed\n");
        else if(pid2 == 0)
        {
            // redirect standart output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/iptables", "iptables", "-t", "nat", "-I", "PREROUTING", "1", "-i", argv[2], "-p", "udp", "-d", argv[3], "--dport", "53", "-j", "REDIRECT", "--to-port", "8053", (char*)0);
        }
        else
            printf("udp port 53 forwarded\n");

        // forward DHCP port via portfwd
        pid3 = fork();
        if (pid3 < 0)
            printf("fork for pid3 failed\n");
        else if(pid3 == 0)
        {
            // redirect standart output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/opt/qaul/bin/portfwd", "portfwd", "-c", "/opt/qaul/etc/portfwd.conf", (char*)0);
        }
        else
            printf("udp port 67 forwarded\n");

        printf("portforwarding started\n");
    }
    else
        printf("missing argument\n");

    return 0;
}

int stop_portforwarding (int argc, const char * argv[])
{
    pid_t pid1, pid2;
    int status;
    printf("stop port forwarding\n");

    // become root
    setuid(0);

    // remove firewall rules
    pid1 = fork();
    if (pid1 < 0)
        printf("fork for pid1 failed\n");
    else if(pid1 == 0)
        execl("/sbin/iptables", "iptables", "-t", "nat", "-D", "PREROUTING", "1", (char*)0);
    else
        printf("tcp port 80 forwarding stopped\n");

    // stop portfwd
    pid2 = fork();
    if (pid2 < 0)
        printf("fork for pid2 failed\n");
    else if(pid2 == 0)
        execl("/usr/bin/killall", "killall", "socat", (char*)0);
    else
        waitpid(pid2, &status, 0);

    printf("port forwarding stopped\n");
	return 0;
}

int start_gateway (int argc, const char * argv[])
{
    pid_t pid1, pid2, pid3;
    int fd, status;
    printf("start gateway\n");

    if(argc >= 3)
    {
        // validate arguments
        if (validate_interface(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }

        // NOTE: don't do that withsetuid, for that the user
        //       has to enter password for this service
        //
        // become root
        //setuid(0);

        // set gateway variable
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            // redirect standart output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/usr/sbin/sysctl", "sysctl", "-w", "net.inet.ip.forwarding=1", (char*)0);
        }
        else
            waitpid(pid1, &status, 0);

        // start natd
        pid2 = fork();
        if (pid2 < 0)
            printf("fork for pid2 failed\n");
        else if(pid2 == 0)
        {
            // redirect standart output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/usr/sbin/natd", "natd", "-interface", argv[2], (char*)0);
        }
        else
            waitpid(pid2, &status, 0);

        // set
        pid3 = fork();
        if (pid3 < 0)
            printf("fork for pid3 failed\n");
        else if(pid3 == 0)
        {
            // redirect standart output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/ipfw", "ipfw", "add", "1055", "divert", "natd", "all", "from", "any", "to", "any", "via", argv[2], (char*)0);
        }
        else
            printf("NAT activated\n");

        printf("gateway started\n");
    }
    else
        printf("missing argument\n");

    return 0;
}

int stop_gateway (int argc, const char * argv[])
{
    pid_t pid1, pid2;
    printf("stop gateway\n");

    // become root
    setuid(0);

    // remove firewall rules
    pid1 = fork();
    if (pid1 < 0)
        printf("fork for pid1 failed\n");
    else if(pid1 == 0)
        execl("/sbin/ipfw", "ipfw", "delete", "1055", (char*)0);
    else
        printf("firewall rules removed\n");

    // stop port forwarding
    pid2 = fork();
    if (pid2 < 0)
        printf("fork for pid2 failed\n");
    else if(pid2 == 0)
        execl("/usr/bin/killall", "killall", "natd", (char*)0);
    else
        printf("natd stopped\n");

    printf("gateway stopped\n");
    return 0;
}

#ifdef WITHOUT_NETWORKMANAGER

int start_networkmanager (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("start network manager\n");

	// become root
	setuid(0);

	// start network manager
	pid1 = fork();
	if (pid1 < 0)
		printf("fork for pid1 failed\n");
	else if(pid1 == 0)
        execl("/usr/bin/service", "service", "network-manager", "start", (char*)0);
	else
		waitpid(pid1, &status, 0);

	printf("network manager started\n");

    return 0;
}

int stop_networkmanager (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("stop network manager\n");

    // become root
    setuid(0);

    // network manager
    pid1 = fork();
    if (pid1 < 0)
        printf("fork for pid2 failed\n");
    else if(pid1 == 0)
        execl("/usr/bin/service", "service", "network-manager", "stop", (char*)0);
    else
        waitpid(pid1, &status, 0);

    printf("network manager stopped\n");
	return 0;
}

int enable_wifi (int argc, const char * argv[])
{
    pid_t pid1, pid2;
    int status;
    printf("enable wifi\n");

	// become root
	setuid(0);

	// deblock wifi
	pid1 = fork();
	if (pid1 < 0)
		printf("fork for pid1 failed\n");
	else if(pid1 == 0)
	{
		execl("rfkill", "unblock", "all", (char*)0);
	}
	else
		waitpid(pid1, &status, 0);

	// enable wifi
	pid2 = fork();
	if (pid2 < 0)
		printf("fork for pid1 failed\n");
	else if(pid2 == 0)
	{
		execl("nmcli", "nm", "wifi", "on", (char*)0);
	}
	else
		waitpid(pid2, &status, 0);


	printf("wifi enabled\n");

	return 0;
}

int configure_wifi (int argc, const char * argv[])
{
    pid_t pid1, pid2, pid3, pid4, pid5, pid6;
    int status;
    char s[256];
    printf("create or join ibss\n");

    if(argc >= 5)
    {
        // validate arguments
        if (validate_interface(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        if (validate_essid(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }
        if (validate_number(argv[4]) == 0)
        {
            printf("argument 3 not valid\n");
            return 0;
        }

        // become root
        setuid(0);

        // take wifi interface down
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            execl("/bin/ip", "ip", "link", "set", argv[2], "down", (char*)0);
        }
        else
            waitpid(pid1, &status, 0);

		printf("wifi interface down\n");

        // set adhoc mode
        pid2 = fork();
        if (pid2 < 0)
            printf("fork for pid2 failed\n");
        else if(pid2 == 0)
        {
            execl("/sbin/iwconfig", argv[2], "mode", "ad-hoc", (char*)0);
        }
        else
            waitpid(pid2, &status, 0);

        printf("adhoc mode set\n");

        // set channel
        pid3 = fork();
        if (pid3 < 0)
            printf("fork for pid3 failed\n");
        else if(pid3 == 0)
        {
            execl("/sbin/iwconfig", argv[2], "channel", argv[4], (char*)0);
        }
        else
            waitpid(pid3, &status, 0);

		printf("channel set\n");

        // set essid
        pid4 = fork();
        if (pid4 < 0)
            printf("fork for pid4 failed\n");
        else if(pid4 == 0)
        {
            sprintf(s, "'%s'", argv[3]);
            execl("/sbin/iwconfig", argv[2], "essid", s, (char*)0);
        }
        else
            waitpid(pid4, &status, 0);

		printf("essid set\n");

		// configure BSSID
		if(argc >= 6)
		{
			// validate argument
			if (validate_interface(argv[5]) == 0)
			{
				printf("argument 4 not valid\n");
				return 0;
			}

			// take wifi interface down
			pid5 = fork();
			if (pid5 < 0)
				printf("fork for pid5 failed\n");
			else if(pid5 == 0)
			{
				execl("/sbin/iwconfig", argv[2], "ap", argv[5], (char*)0);
			}
			else
				waitpid(pid5, &status, 0);

			printf("BSSID set\n");
		}

        // bring wifi interface up
        pid6 = fork();
        if (pid6 < 0)
            printf("fork for pid6 failed\n");
        else if(pid6 == 0)
        {
            execl("/bin/ip", "ip", "link", "set", argv[2], "up", (char*)0);
        }
        else
            waitpid(pid6, &status, 0);

        printf("wifi configured\n");
    }
    else
        printf("missing argument\n");

	return 0;
}

int set_ip (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    char s[256];
    printf("configure ip\n");

    if(argc >= 6)
    {
        // validate arguments
        if (validate_interface(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        if (validate_ip(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }
        if (validate_number(argv[4]) == 0)
        {
            printf("argument 3 not valid\n");
            return 0;
        }
        if (validate_ip(argv[5]) == 0)
        {
            printf("argument 4 not valid\n");
            return 0;
        }

        // become root
        setuid(0);

        // set ip manually
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            sprintf(s,"%s/%s", argv[3], argv[4]);
            execl("/bin/ip", "ip", "addr", "add", s, "dev", argv[2], "broadcast", argv[5], (char*)0);
		}
        else
            waitpid(pid1, &status, 0);

        printf("ip configured\n");
    }
    else
        printf("missing argument\n");

	return 0;
}

int set_dns (int argc, const char * argv[])
{
    pid_t pid1, pid2, pid3;
    int status;
    printf("set DNS\n");

	// set root rights
	setuid(0);

	// remove tail
	pid1 = fork();
	if (pid1 < 0)
		printf("fork for pid1 failed\n");
	else if(pid1 == 0)
	{
		execl("/bin/rm", "rm", "/etc/resolvconf/resolv.conf.d/tail", (char*)0);
	}
	else
		waitpid(pid1, &status, 0);

	// set dns
	pid2 = fork();
	if (pid2 < 0)
		printf("fork for pid2 failed\n");
	else if(pid2 == 0)
	{
		execl("/bin/cp", "/opt/qaul/etc/tail", "/etc/resolvconf/resolv.conf.d/tail", (char*)0);
	}
	else
		waitpid(pid2, &status, 0);

	// reload resolv file
	pid3 = fork();
	if (pid3 < 0)
		printf("fork for pid3 failed\n");
	else if(pid3 == 0)
	{
		execl("/sbin/resolvconf", "resolvconf", "-u", (char*)0);
	}
	else
		waitpid(pid3, &status, 0);

	printf("DNS set\n");

    return 0;
}

#endif // WITHOUT_NETWORKMANAGER


/**
 * validation functions
 */

int validate_ip (const char* str)
{
    int i;

    // check length
    if(strlen(str)>15)
    {
        printf("argument too long\n");
        return 0;
    }

    // check numbers and dots
    for(i=0; i<strlen(str); i++)
    {
        if(validate_char_number(str[i]) == 0 && strncmp(&str[i], ".", 1) != 0)
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}

int validate_interface (const char* str)
{
    int i;

    // check length
    if(strlen(str)>20)
    {
        printf("argument too long\n");
        return 0;
    }

    // check numbers and dots
    for(i=0; i<strlen(str); i++)
    {
        if(validate_char_number(str[i]) == 0 &&
           validate_char_letter(str[i]) == 0
           )
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}

int validate_number (const char* str)
{
    int i;

    // check length
    if(strlen(str)>10)
    {
        printf("argument too long\n");
        return 0;
    }

    // check numbers and dots
    for(i=0; i<strlen(str); i++)
    {
        if(validate_char_number(str[i]) == 0)
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}

int validate_path (const char* str)
{
    int i;

    // check length
    if(strlen(str)>200)
    {
        printf("argument too long\n");
        return 0;
    }

    // check if it is a valid path
    if(strncmp(&str[0], "/", 1) != 0)
    {
        printf("not an absolute path\n");
        return 0;
    }

    for(i=0; i<strlen(str); i++)
    {
        if(
           strncmp(&str[i], ":", 1) == 0
           )
        {
            printf("invalid character\n");
            return 0;
        }
    }

    return 1;
}

int validate_essid (const char* str)
{
    int i;

    // check length
    if(strlen(str)>50)
    {
        printf("argument too long\n");
        return 0;
    }

    // check numbers and dots
    for(i=0; i<strlen(str); i++)
    {
        if(validate_char_number(str[i]) == 0 &&
           validate_char_letter(str[i]) == 0 &&
           strncmp(&str[i], ".", 1) != 0 &&
           strncmp(&str[i], "-", 1) != 0 &&
           strncmp(&str[i], "_", 1) != 0
           )
        {
            printf("invalid character\n");
            return 0;
        }
    }
    return 1;
}

int validate_bssid (const char* str)
{
    int i;

    // check length
    if(strlen(str)!=17)
    {
        printf("BSSID not correct\n");
        return 0;
    }

    // check validity of positions
    for(i=0; i<strlen(str); i++)
    {
		if(i == 2 || i == 5 || i == 8 || i == 11 || i == 14 )
		{
			// needs to be colon
			if(strncmp(&str[i], ".", 1) != 0)
			{
				printf("BSSID: invalid character at position %i\n", i);
				return 0;
			}
		}
        else
        {
			// needs to be a number or a letter
			if(validate_char_number(str[i]) == 0 && validate_char_letter(str[i]) == 0)
			{
				printf("BSSID: invalid character at position %i\n", i);
				return 0;
			}
		}
    }
    return 1;
}

int validate_char_number (char mychar)
{
    if(strncmp(&mychar, "0", 1)==0)
        return 1;
    if(strncmp(&mychar, "1", 1)==0)
        return 1;
    if(strncmp(&mychar, "2", 1)==0)
        return 1;
    if(strncmp(&mychar, "3", 1)==0)
        return 1;
    if(strncmp(&mychar, "4", 1)==0)
        return 1;
    if(strncmp(&mychar, "5", 1)==0)
        return 1;
    if(strncmp(&mychar, "6", 1)==0)
        return 1;
    if(strncmp(&mychar, "7", 1)==0)
        return 1;
    if(strncmp(&mychar, "8", 1)==0)
        return 1;
    if(strncmp(&mychar, "9", 1)==0)
        return 1;

    return 0;
}

int validate_char_letter (char mychar)
{
    if(strncmp(&mychar, "a", 1)==0 || strncmp(&mychar, "A", 1)==0)
        return 1;
    if(strncmp(&mychar, "b", 1)==0 || strncmp(&mychar, "B", 1)==0)
        return 1;
    if(strncmp(&mychar, "c", 1)==0 || strncmp(&mychar, "C", 1)==0)
        return 1;
    if(strncmp(&mychar, "d", 1)==0 || strncmp(&mychar, "D", 1)==0)
        return 1;
    if(strncmp(&mychar, "e", 1)==0 || strncmp(&mychar, "E", 1)==0)
        return 1;
    if(strncmp(&mychar, "f", 1)==0 || strncmp(&mychar, "F", 1)==0)
        return 1;
    if(strncmp(&mychar, "g", 1)==0 || strncmp(&mychar, "G", 1)==0)
        return 1;
    if(strncmp(&mychar, "h", 1)==0 || strncmp(&mychar, "H", 1)==0)
        return 1;
    if(strncmp(&mychar, "i", 1)==0 || strncmp(&mychar, "I", 1)==0)
        return 1;
    if(strncmp(&mychar, "j", 1)==0 || strncmp(&mychar, "J", 1)==0)
        return 1;
    if(strncmp(&mychar, "k", 1)==0 || strncmp(&mychar, "K", 1)==0)
        return 1;
    if(strncmp(&mychar, "l", 1)==0 || strncmp(&mychar, "L", 1)==0)
        return 1;
    if(strncmp(&mychar, "m", 1)==0 || strncmp(&mychar, "M", 1)==0)
        return 1;
    if(strncmp(&mychar, "n", 1)==0 || strncmp(&mychar, "N", 1)==0)
        return 1;
    if(strncmp(&mychar, "o", 1)==0 || strncmp(&mychar, "O", 1)==0)
        return 1;
    if(strncmp(&mychar, "p", 1)==0 || strncmp(&mychar, "P", 1)==0)
        return 1;
    if(strncmp(&mychar, "q", 1)==0 || strncmp(&mychar, "Q", 1)==0)
        return 1;
    if(strncmp(&mychar, "r", 1)==0 || strncmp(&mychar, "R", 1)==0)
        return 1;
    if(strncmp(&mychar, "s", 1)==0 || strncmp(&mychar, "S", 1)==0)
        return 1;
    if(strncmp(&mychar, "t", 1)==0 || strncmp(&mychar, "T", 1)==0)
        return 1;
    if(strncmp(&mychar, "u", 1)==0 || strncmp(&mychar, "U", 1)==0)
        return 1;
    if(strncmp(&mychar, "v", 1)==0 || strncmp(&mychar, "V", 1)==0)
        return 1;
    if(strncmp(&mychar, "w", 1)==0 || strncmp(&mychar, "W", 1)==0)
        return 1;
    if(strncmp(&mychar, "x", 1)==0 || strncmp(&mychar, "X", 1)==0)
        return 1;
    if(strncmp(&mychar, "y", 1)==0 || strncmp(&mychar, "Y", 1)==0)
        return 1;
    if(strncmp(&mychar, "z", 1)==0 || strncmp(&mychar, "Z", 1)==0)
        return 1;

    return 0;
}

int validate_char_problematic (char mychar)
{
    if(strncmp(&mychar, "\"", 1) == 0)
        return 0;
    if(strncmp(&mychar, "'", 1) == 0)
        return 0;
    if(strncmp(&mychar, "`", 1) == 0)
        return 0;
    if(strncmp(&mychar, ";", 1) == 0)
        return 0;
    if(strncmp(&mychar, "\\", 1) == 0)
        return 0;
    if(strncmp(&mychar, "&", 1) == 0)
        return 0;
    if(strncmp(&mychar, ">", 1) == 0)
        return 0;
    if(strncmp(&mychar, "<", 1) == 0)
        return 0;
    if(strncmp(&mychar, "|", 1) == 0)
        return 0;
    if(strncmp(&mychar, "$", 1) == 0)
        return 0;
    if(strncmp(&mychar, "*", 1) == 0)
        return 0;
    if(strncmp(&mychar, "%", 1) == 0)
        return 0;
    if(strncmp(&mychar, "?", 1) == 0)
        return 0;
    if(strncmp(&mychar, "!", 1) == 0)
        return 0;
    if(strncmp(&mychar, "#", 1) == 0)
        return 0;
    if(strncmp(&mychar, "~", 1) == 0)
        return 0;
    if(strncmp(&mychar, "=", 1) == 0)
        return 0;
    if(strncmp(&mychar, "(", 1) == 0)
        return 0;
    if(strncmp(&mychar, "[", 1) == 0)
        return 0;
    if(strncmp(&mychar, "{", 1) == 0)
        return 0;
    if(strncmp(&mychar, "^", 1) == 0)
        return 0;

    return 1;
}
