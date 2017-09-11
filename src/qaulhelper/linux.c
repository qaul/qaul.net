/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * linux specific functions of qaulhelper.
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
 *   qaulhelper startgateway <INTERFACE OUT> <INTERFACE OUT>
 *   qaulhelper startgateway eth0 wlan0
 *   qaulhelper stopgateway <INTERFACE OUT> <INTERFACE IN>
 *   qaulhelper stopgateway eth0 wlan0
 *   qaulhelper configurewifi <INTERFACE> <ESSID> <FREQUENCY> [<BSSID>]
 *   qaulhelper configurewifi wlan0 qaul.net 2462 02:11:87:88:D6:FF
 *   qaulhelper stopnetworking
 *   qaulhelper stopnetworking
 *   qaulhelper restartnetworking
 *   qaulhelper restartnetworking
 *   qaulhelper setip <INTERFACE> <IP> <SUBNET> <BROADCAST>
 *   qaulhelper setip wlan0 10.213.28.55 8 10.255.255.255
 *   qaulhelper setdns <INTERFACE>
 *   qaulhelper setdns wlan0
 *
 */

#include "qaulhelper.h"
#include <sys/wait.h>
#include "QaulConfig.h"
#include "qaul/utils/validate.h"


int start_olsrd (int argc, const char * argv[])
{
    pid_t pid1;
    int status, fd;
    char s[256];
    printf("start olsrd\n");

    if(argc >= 4)
    {
        // validate arguments
        if(strncmp(argv[2], "yes", 3)==0)
            snprintf(s, 255, "%s/lib/qaul/etc/olsrd_linux_gw.conf", QAUL_ROOT_PATH);
        else
            snprintf(s, 255, "%s/lib/qaul/etc/olsrd_linux.conf", QAUL_ROOT_PATH);

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
            execl(QAUL_ROOT_PATH"/lib/qaul/bin/olsrd", "olsrd", "-f", s, "-i", argv[3], "-d", "0", (char*)0);
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
        if (validate_ip(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
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
            // redirect standard output and error to /dev/null
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
            // redirect standard output and error to /dev/null
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
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl(QAUL_ROOT_PATH"/lib/qaul/bin/portfwd", "portfwd", "-c", QAUL_ROOT_PATH"/lib/qaul/etc/portfwd.conf", (char*)0);
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
    pid_t pid1, pid2, pid3, pid4;
    int fd, status;
    printf("start gateway\n");

    if(argc >= 4)
    {
        // validate arguments
        if (validate_interface(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        if (validate_interface(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }

        setuid(0);

        // allow forwarding
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/sysctl", "sysctl", "-w", "net.ipv4.ip_forward=1", (char*)0);
        }
        else
            waitpid(pid1, &status, 0);

        // set NAT and mask IP
        pid2 = fork();
        if (pid2 < 0)
            printf("fork for pid2 failed\n");
        else if(pid2 == 0)
        {
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/iptables", "iptables", "-t", "nat", "-A", "POSTROUTING", "-o", argv[2], "-j", "MASQUERADE", (char*)0);
        }
        else
            waitpid(pid2, &status, 0);

        // allow established connections
        pid3 = fork();
        if (pid3 < 0)
            printf("fork for pid3 failed\n");
        else if(pid3 == 0)
        {
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/iptables", "iptables", "-A", "FORWARD", "-i", argv[2], "-o", argv[3], "-m", "state", "--state", "RELATED,ESTABLISHED", "-j", "ACCEPT", (char*)0);
        }
        else
        	waitpid(pid3, &status, 0);

        // allow outgoing connections
        pid4 = fork();
        if (pid4 < 0)
            printf("fork for pid4 failed\n");
        else if(pid4 == 0)
        {
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/iptables", "iptables", "-A", "FORWARD", "-i", argv[3], "-o", argv[2], "-j", "ACCEPT", (char*)0);
        }
        else
            printf("Internet sharing activated\n");

        printf("gateway started\n");
    }
    else
        printf("missing argument\n");

    return 0;
}

int stop_gateway (int argc, const char * argv[])
{
    pid_t pid1, pid2, pid3;
    int fd, status;
    printf("stop gateway\n");

    if(argc >= 4)
    {
        // validate arguments
        if (validate_interface(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        if (validate_interface(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }

        setuid(0);

        // delete NAT
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/iptables", "iptables", "-t", "nat", "-D", "POSTROUTING", "-o", argv[2], "-j", "MASQUERADE", (char*)0);
        }
        else
            waitpid(pid1, &status, 0);

        // delete incoming rule
        pid2 = fork();
        if (pid2 < 0)
            printf("fork for pid2 failed\n");
        else if(pid2 == 0)
        {
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/iptables", "iptables", "-D", "FORWARD", "-i", argv[2], "-o", argv[3], "-m", "state", "--state", "RELATED,ESTABLISHED", "-j", "ACCEPT", (char*)0);
        }
        else
        	waitpid(pid2, &status, 0);

        // delete outgoing rule
        pid3 = fork();
        if (pid3 < 0)
            printf("fork for pid3 failed\n");
        else if(pid3 == 0)
        {
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/sbin/iptables", "iptables", "-D", "FORWARD", "-i", argv[3], "-o", argv[2], "-j", "ACCEPT", (char*)0);
        }
        else
            printf("Internet sharing deactivated\n");

        printf("gateway stopped\n");
    }
    else
        printf("missing argument\n");

    return 0;
}

int configure_wifi (int argc, const char * argv[])
{
    pid_t pid1, pid2, pid3, pid4, pid5, pid6;
    int status;
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
        
        printf("arv %s %s %s\n", argv[2], argv[3], argv[4]);

        // disconnect from any ibss network
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            execl("/sbin/iw", "iw", "dev", argv[2], "ibss", "leave", (char*)0);
        }
        else
            waitpid(pid1, &status, 0);

		printf("ibss disconnected\n");


        // flush IP address
        pid2 = fork();
        if (pid2 < 0)
            printf("fork for pid2 failed\n");
        else if(pid2 == 0)
        {
            execl("/bin/ip", "ip", "addr", "flush", "dev", argv[2], (char*)0);
        }
        else
            waitpid(pid2, &status, 0);

		printf("ip address flushed\n");


        // take wifi interface down
        pid3 = fork();
        if (pid3 < 0)
            printf("fork for pid3 failed\n");
        else if(pid3 == 0)
        {
            execl("/bin/ip", "ip", "link", "set", argv[2], "down", (char*)0);
        }
        else
            waitpid(pid3, &status, 0);

		printf("wifi interface down\n");


		// set adhoc mode
        pid4 = fork();
        if (pid4 < 0)
            printf("fork for pid4 failed\n");
        else if(pid4 == 0)
        {
            execl("/sbin/iw", "iw", "dev", argv[2], "set", "type", "ibss", (char*)0);
        }
        else
            waitpid(pid4, &status, 0);

        printf("adhoc mode set\n");


        // bring wifi interface up
        pid5 = fork();
        if (pid5 < 0)
            printf("fork for pid5 failed\n");
        else if(pid5 == 0)
        {
            execl("/bin/ip", "ip", "link", "set", argv[2], "up", (char*)0);
        }
        else
            waitpid(pid5, &status, 0);

        printf("wifi interface up\n");


		// join network
		if(argc >= 6)
		{
			// validate argument
			if (validate_interface(argv[5]) == 0)
			{
				printf("argument 4 not valid\n");
				return 0;
			}

			// configure essid, channel & bssid
			pid6 = fork();
			if (pid6 < 0)
				printf("fork for pid6 failed\n");
			else if(pid6 == 0)
			{
				execl("/sbin/iw", "iw", "dev", argv[2], "ibss", "join", argv[3], argv[4], argv[5], (char*)0);
			}
			else
				waitpid(pid6, &status, 0);

			printf("ESSID, channel & BSSID set\n");
		}
		else
		{
	        // configure essid & channel
	        pid6 = fork();
	        if (pid6 < 0)
	            printf("fork for pid6 failed\n");
	        else if(pid6 == 0)
	        {
	            execl("/sbin/iw", "iw", "dev", argv[2], "ibss", "join", argv[3], argv[4], (char*)0);
	        }
	        else
	            waitpid(pid6, &status, 0);

			printf("ESSID & channel set\n");
		}
    }
    else
        printf("missing argument\n");

	return 0;
}

#ifdef QAUL_STOP_NETWORKING

int stop_networking (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("stop networking\n");

    // become root
    setuid(0);

    // kill olsrd
    pid1 = fork();
    if (pid1 < 0)
        printf("fork for pid1 failed\n");
    else if(pid1 == 0)
    	execl("/usr/bin/killall", "killall", "wpa_supplicant", (char*)0);
    else
        waitpid(pid1, &status, 0);

    printf("networking stopped\n");
    return 0;
}

int restart_networking (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("restart networking\n");

    // become root
    setuid(0);

    // kill olsrd
    pid1 = fork();
    if (pid1 < 0)
        printf("fork for pid1 failed\n");
    else if(pid1 == 0)
        execl("/etc/init.d/networking", "networking", "restart", (char*)0);
    else
        waitpid(pid1, &status, 0);

    printf("networking restarted\n");
    return 0;
}

#endif // QAUL_STOP_NETWORKING

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
    pid_t pid1;
    int status;
    printf("set DNS\n");

	// set root rights
	setuid(0);

	// set dns via resolvconf
	pid1 = fork();
	if (pid1 < 0)
		printf("fork for pid1 failed\n");
	else if(pid1 == 0)
	{
		execl("/sbin/resolvconf", "resolvconf", "-a", argv[2], "<", QAUL_ROOT_PATH"/lib/qaul/etc/tail", (char*)0);
	}
	else
		waitpid(pid1, &status, 0);

	printf("DNS set\n");

    return 0;
}

int remove_dns (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("remove DNS\n");

	// set root rights
	setuid(0);

	// set dns via resolvconf
	pid1 = fork();
	if (pid1 < 0)
		printf("fork for pid1 failed\n");
	else if(pid1 == 0)
	{
		execl("/sbin/resolvconf", "resolvconf", "-d", argv[2], (char*)0);
	}
	else
		waitpid(pid1, &status, 0);

	printf("DNS removed\n");

    return 0;
}
