/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * osx specific functions of qaulhelper.
 *
 *
 * usage:
 *   qaulhelper startolsrd <ISGATEWAY yes|no> <INTERFACE> 
 *   qaulhelper startolsrd yes en1
 *   qaulhelper stopolsrd
 *   qaulhelper startportforwarding <INTERFACE>
 *   qaulhelper startportforwarding en1
 *   qaulhelper stopportforwarding
 *   qaulhelper enablewifi <OSXVERSION> <INTERFACE>
 *   qaulhelper enablewifi 1038 en1
 *   qaulhelper disablewifi <OSXVERSION> <INTERFACE>
 *   qaulhelper disablewifi 1038 en1
 *   qaulhelper createnetworkprofile <POFILENAME>
 *   qaulhelper createnetworkprofile qaul.net
 *   qaulhelper switchnetworkprofile <POFILENAME>
 *   qaulhelper switchnetworkprofile qaul.net
 *   qaulhelper deletenetworkprofile <POFILENAME>
 *   qaulhelper deletenetworkprofile qaul.net
 *   qaulhelper setip <SERVICENAME> <IP> <SUBNET> <ROUTER>
 *   qaulhelper setip Wi-Fi 10.213.28.55 255.0.0.0 0.0.0.0
 *   qaulhelper setdhcp <SERVICENAME> <INTERFACENAME>
 *   qaulhelper setdhcp Wi-Fi en1
 *   qaulhelper setdns <SERVICENAME>
 *   qaulhelper setdns Wi-Fi
 *   qaulhelper startgateway <INTERFACE>
 *   qaulhelper startgateway en1
 *   qaulhelper stopgateway
 *   qaulhelper stopgateway
 *
 * only for OSX <= 10.5
 *   qaulhelper createibss <ESSID> <CHANNEL>
 *   qaulhelper createibss qaul.net 11
 *
 *
 * NOTE: 
 *   networksetup path ( /usr/sbin/networksetup ) valid on OSX >= 10.5
 *   path on OSX < 10.5 was :
 *   /System/Library/CoreServices/RemoteManagement/ARDAgent.app/Contents/Support/networksetup
 */

#include "qaulhelper.h"


#define NSAppKitVersionNumber10_4 824
#define NSAppKitVersionNumber10_5 949


int start_olsrd (int argc, const char * argv[])
{
    pid_t pid1;
    int status, fd;
    char s[256], p[256];
    printf("start olsrd\n");
    
    if(argc >= 4)
    {
        // validate arguments
        if(strncmp(argv[2], "yes", 3)==0)
            sprintf(s,"%s/etc/olsrd_osx_gw.conf", QAUL_ROOT_PATH);
        else
            sprintf(s,"%s/etc/olsrd_osx.conf", QAUL_ROOT_PATH);
        
        if (validate_interface(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }
        
        // set olsrd binary path
        sprintf(p,"%s/bin/olsrd", QAUL_ROOT_PATH);
        
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
            execl(p, "olsrd", "-f", s, "-i", argv[3], "-d", "0", (char*)0);
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
    pid_t pid0, pid1, pid2, pid3;
    int fd, status;
    char p[256];
    printf("start portforwarding\n");
    
    if(argc >= 3)
    {
        // validate arguments
        if (validate_interface(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        
        // set socat binary path
        sprintf(p,"%s/bin/socat", QAUL_ROOT_PATH);
        
        // become root
        setuid(0);

        // set firewall variable (just in case ...)
        pid0 = fork();
        if (pid0 < 0)
            printf("fork for pid0 failed\n");
        else if(pid0 == 0)
        {
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/usr/sbin/sysctl", "sysctl", "-w", "net.inet.ip.fw.enable=1", (char*)0);
        }
        else
            waitpid(pid0, &status, 0);
        
        // forward port 80 by firewall
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
            execl("/sbin/ipfw", "ipfw", "add", "1053", "fwd", "localhost,8081", "tcp", "from", "any", "to", "any", "80", "in", "recv", argv[2], (char*)0);
        }
        else
            printf("tcp port 80 forwarded\n");

        // forward udp ports by socat
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
            execl(p, "socat", "UDP4-RECVFROM:53,fork", "UDP4-SENDTO:localhost:8053", (char*)0);
        }
        else
            printf("udp port 53 forwarded\n");
        
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
            execl(p, "socat", "UDP4-RECVFROM:67,fork", "UDP4-SENDTO:localhost:8067", (char*)0);
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
        execl("/sbin/ipfw", "ipfw", "delete", "1053", (char*)0);
    else
        printf("tcp port 80 forwarding stopped\n");
    
    // stop port forwarding
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

int enable_wifi (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("enable wifi\n");
    
    if(argc >= 4)
    {
        // validate arguments
        if (validate_number(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        if (validate_interface(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }
        
        // become root
        setuid(0);

        // enable wifi
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            if(atoi(argv[2]) <= NSAppKitVersionNumber10_5)
                execl("/usr/sbin/networksetup", "networksetup", "-setairportpower", "on", (char*)0);
            else
                execl("/usr/sbin/networksetup", "networksetup", "-setairportpower", argv[3], "on", (char*)0);
        }
        else
            waitpid(pid1, &status, 0);
        
        printf("wifi enabled\n");
    }
    else
        printf("missing argument\n");
    
	return 0;
}

int disable_wifi (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("disable wifi\n");
    
    if(argc >= 4)
    {
        // validate arguments
        if (validate_number(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        if (validate_interface(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }

        // become root
        setuid(0);

        // disable wifi
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            if(atoi(argv[2]) <= NSAppKitVersionNumber10_5)
                execl("/usr/sbin/networksetup", "networksetup", "-setairportpower", "off", (char*)0);
            else
                execl("/usr/sbin/networksetup", "networksetup", "-setairportpower", argv[3], "off", (char*)0);        
        }
        else
            waitpid(pid1, &status, 0);

        printf("wifi disabled\n");
    }
    else
        printf("missing argument\n");
    
	return 0;
}

int create_networkprofile (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("create network profile\n");
    
    if(argc >= 3)
    {
        // validate arguments
        if (validate_service(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        
        // become root
        setuid(0);

        // create network profile
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
            execl("/usr/sbin/networksetup", "networksetup", "-createlocation", argv[2], "populate", (char*)0);
        else
            waitpid(pid1, &status, 0);
        
        printf("network profile created\n");
    }
    else
        printf("missing argument\n");
    
	return 0;
}

int switch_networkprofile (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("switch network profile\n");
    
    if(argc >= 3)
    {
        // validate arguments
        if (validate_service(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        
        // become root
        setuid(0);

        // switch network profile
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
            execl("/usr/sbin/networksetup", "networksetup", "-switchtolocation", argv[2], (char*)0);
        else
            waitpid(pid1, &status, 0);
        
        printf("network profile switched\n");
    }
    else
        printf("missing argument\n");
    
	return 0;
}

int delete_networkprofile (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("delete network profile\n");
    
    if(argc >= 3)
    {
        // validate arguments
        if (validate_service(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        
        // become root
        setuid(0);

        // switch network profile
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
            execl("/usr/sbin/networksetup", "networksetup", "-deletelocation", argv[2], (char*)0);
        else
            waitpid(pid1, &status, 0);

        printf("network profile deleted\n");
    }
    else
        printf("missing argument\n");
    
	return 0;
}

int create_ibss (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    char s1[256];
    char s2[256];
    printf("create or join ibss\n");
    
    if(argc >= 4)
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
        
        // become root
        setuid(0);

        // create or join ibss
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
        {
            sprintf(s1,"-i%s", argv[2]);
            sprintf(s2,"-c%s", argv[3]);
            execl("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport", "airport", s1, s2, (char*)0);
        }
        else
            waitpid(pid1, &status, 0);

        printf("ibss created\n");
    }
    else
        printf("missing argument\n");
    
	return 0;
}

int set_ip (int argc, const char * argv[])
{
    pid_t pid1;
    int status;
    printf("configure ip\n");
    
    if(argc >= 6)
    {
        // validate arguments
        if (validate_service(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        if (validate_ip(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }
        if (validate_ip(argv[4]) == 0)
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
            execl("/usr/sbin/networksetup", "networksetup", "-setmanual", argv[2], argv[3], argv[4], argv[5], (char*)0);
        else
            waitpid(pid1, &status, 0);

        printf("ip configured\n");
    }
    else
        printf("missing argument\n");
    
	return 0;
}

int set_dhcp (int argc, const char * argv[])
{
    pid_t pid1, pid2;
    int status;
    printf("set DHCP\n");
    
    if(argc >= 3)
    {
        // validate arguments
        if (validate_service(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }
        if (validate_interface(argv[3]) == 0)
        {
            printf("argument 2 not valid\n");
            return 0;
        }
        
        // become root
        setuid(0);

        // set dhcp
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
            execl("/usr/sbin/networksetup", "networksetup", "-setdhcp", argv[2], (char*)0);
        else
            waitpid(pid1, &status, 0);
        
        usleep(200000);
        
        pid2 = fork();
        if (pid2 < 0)
            printf("fork for pid2 failed\n");
        else if(pid2 == 0)
            execl("/usr/sbin/ipconfig", "ipconfig", "set", argv[3], "DHCP", (char*)0);
        else
            waitpid(pid2, &status, 0);
        
        printf("DHCP set\n");
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
    
    if(argc >= 3)
    {
        // validate arguments
        if (validate_service(argv[2]) == 0)
        {
            printf("argument 1 not valid\n");
            return 0;
        }

        // set root rights
        setuid(0);

        // set dns
        pid1 = fork();
        if (pid1 < 0)
            printf("fork for pid1 failed\n");
        else if(pid1 == 0)
            execl("/usr/sbin/networksetup", "networksetup", "-setdnsservers", argv[2], "5.45.96.220", "185.82.22.133", (char*)0);
        else
            waitpid(pid1, &status, 0);
        
        printf("DNS set\n");
    }
    else
        printf("missing argument\n");
    
    return 0;
}

int start_gateway (int argc, const char * argv[])
{
    pid_t pid1, pid2, pid3, pid4;
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
        
        // NOTE: don't do that with setuid, for that the user
        //       has to enter password for this service
        //
        // become root
        setuid(0);
        
        // set gateway variable
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
            execl("/usr/sbin/sysctl", "sysctl", "-w", "net.inet.ip.forwarding=1", (char*)0);
        }
        else
            waitpid(pid1, &status, 0);

        // stop natd if it is already running
        pid2 = fork();
        if (pid2 < 0)
            printf("fork for pid2 failed\n");
        else if(pid2 == 0)
            execl("/usr/bin/killall", "killall", "natd", (char*)0);
        else
            printf("natd stopped\n");

        // start natd
        pid3 = fork();
        if (pid3 < 0)
            printf("fork for pid2 failed\n");
        else if(pid3 == 0)
        {
            // redirect standard output and error to /dev/null
            // the program otherwise often didn't return correctly
            fd = open("/dev/null", O_WRONLY | O_CREAT | O_APPEND);
            dup2(fd, STDOUT_FILENO);
            dup2(fd, STDERR_FILENO);
            close(fd);
            // execute program
            execl("/usr/sbin/natd", "natd", "-interface", argv[2], (char*)0);
        }
        else
            waitpid(pid3, &status, 0);
        
        // set 
        pid4 = fork();
        if (pid4 < 0)
            printf("fork for pid3 failed\n");
        else if(pid4 == 0)
        {
            // redirect standard output and error to /dev/null
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
